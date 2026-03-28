use core::option::Option;
use std::{ffi::CString, i16, ptr};

use esp_idf_svc::sys::lcd_bindings::{
    LV_CHART_AXIS_PRIMARY_X, LV_CHART_AXIS_PRIMARY_Y, LV_CHART_AXIS_SECONDARY_Y, LV_CHART_TYPE_BAR,
    LV_CHART_TYPE_LINE, LV_CHART_UPDATE_MODE_SHIFT, LV_OPA_COVER, LV_PART_INDICATOR, LV_PART_ITEMS,
    LV_PART_MAIN, LV_PART_TICKS, LV_TEXT_ALIGN_CENTER, lv_area_t, lv_chart_add_series,
    lv_chart_axis_t, lv_chart_get_pressed_point, lv_chart_get_series_next, lv_chart_refresh,
    lv_chart_set_all_value, lv_chart_set_axis_tick, lv_chart_set_div_line_count,
    lv_chart_set_next_value, lv_chart_set_point_count, lv_chart_set_range, lv_chart_set_type,
    lv_chart_set_update_mode, lv_chart_type_t, lv_chart_update_mode_t, lv_color16_t, lv_draw_label,
    lv_draw_label_dsc_init, lv_draw_label_dsc_t, lv_draw_rect, lv_draw_rect_dsc_init,
    lv_draw_rect_dsc_t, lv_event_code_t_LV_EVENT_DRAW_PART_BEGIN,
    lv_event_code_t_LV_EVENT_DRAW_PART_END, lv_event_code_t_LV_EVENT_VALUE_CHANGED,
    lv_event_get_draw_part_dsc, lv_event_get_target, lv_event_t, lv_label_set_text,
    lv_obj_add_event_cb, lv_obj_set_style_line_color, lv_obj_set_style_pad_column,
    lv_obj_set_style_text_color, lv_obj_set_style_width, lv_palette_main,
    lv_palette_t_LV_PALETTE_BLUE, lv_palette_t_LV_PALETTE_GREEN, lv_palette_t_LV_PALETTE_RED,
    lv_palette_t_LV_PALETTE_YELLOW, lv_snprintf, lvgl_port_lock, lvgl_port_unlock, objects,
    ui_font_roboto_reg_14,
};
// use log::info;

use crate::{
    client::mppt::{self, DAYS, HistoryDay, HistoryLifetime, MpptError},
    ui::vars::set_var_hist_det_day,
};

/// Draw the bar chart day labels
#[unsafe(no_mangle)]
pub unsafe extern "C" fn history_chart_draw_event_begin_cb(event: *mut lv_event_t) {
    unsafe {
        let dsc = *lv_event_get_draw_part_dsc(event);

        if dsc.part == LV_PART_TICKS {
            if !dsc.label_dsc.is_null() {
                (*dsc.label_dsc).opa = LV_OPA_COVER as _;
            }
            if !dsc.line_dsc.is_null() {
                (*dsc.line_dsc).opa = LV_OPA_COVER as _;
            }

            if !dsc.text.is_null() && dsc.id == LV_CHART_AXIS_PRIMARY_X {
                if !dsc.label_dsc.is_null() {
                    (*dsc.label_dsc).color = lv_color16_t::default();
                }

                let day = if dsc.value == 0 {
                    "T".to_string()
                } else if dsc.value == 1 {
                    "Y".to_string()
                } else {
                    dsc.value.to_string()
                };

                lv_snprintf(
                    dsc.text,
                    dsc.text_length as _,
                    CString::new("%s").unwrap().as_ptr(),
                    CString::new(day).unwrap().as_ptr(),
                );
            }
        }
    }
}

/// Draw the abs/float/bulk as stacked bars over the top of base bar which is total power.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn history_chart_draw_event_end_cb(event: *mut lv_event_t) {
    unsafe {
        let dsc = *lv_event_get_draw_part_dsc(event);

        if dsc.part == LV_PART_ITEMS {
            let (abs_value, bulk_value, is_error) = {
                let history = &mppt::HISTORY.read().unwrap().history;
                if let Some(Some(history)) = history.get(dsc.id as usize) {
                    (
                        history.abs_pct(),
                        history.bulk_pct(),
                        history.errors != MpptError::None,
                    )
                } else {
                    return;
                }
            };

            let draw_area = *dsc.draw_area;
            // info!("da {draw_area:?} av {abs_value} bv {bulk_value}");

            // Absorption column
            let height = draw_area.y2 - draw_area.y1;

            let mut abs_rect_dsc: lv_draw_rect_dsc_t = Default::default();
            lv_draw_rect_dsc_init(&mut abs_rect_dsc);

            abs_rect_dsc.bg_color = lv_palette_main(lv_palette_t_LV_PALETTE_RED);

            let abs_area = lv_area_t {
                x1: draw_area.x1,
                y1: draw_area.y1 + (height as f32 * (1.0 - (abs_value + bulk_value))) as i16,
                x2: draw_area.x2,
                y2: draw_area.y2,
            };

            lv_draw_rect(dsc.draw_ctx, &abs_rect_dsc, &abs_area);

            // Bulk column
            let mut bulk_rect_dsc: lv_draw_rect_dsc_t = Default::default();
            lv_draw_rect_dsc_init(&mut bulk_rect_dsc);

            bulk_rect_dsc.bg_color = lv_palette_main(if is_error {
                lv_palette_t_LV_PALETTE_RED
            } else {
                lv_palette_t_LV_PALETTE_BLUE
            });

            let bulk_area = lv_area_t {
                x1: draw_area.x1,
                y1: draw_area.y1 + (height as f32 * (1.0 - bulk_value)) as i16,
                x2: draw_area.x2,
                y2: draw_area.y2,
            };

            lv_draw_rect(dsc.draw_ctx, &bulk_rect_dsc, &bulk_area);

            // Yield value label - draw this last so that it will go over the last column
            let mut yield_label_dsc: lv_draw_label_dsc_t = Default::default();
            lv_draw_label_dsc_init(&mut yield_label_dsc);

            yield_label_dsc.font = &ui_font_roboto_reg_14;
            yield_label_dsc.align = LV_TEXT_ALIGN_CENTER as _;

            let yield_label_area = lv_area_t {
                x1: draw_area.x1,
                y1: draw_area.y1 + 4,
                x2: draw_area.x2,
                y2: draw_area.y2 + 20,
            };
            lv_draw_label(
                dsc.draw_ctx,
                &yield_label_dsc,
                &yield_label_area,
                CString::new(dsc.value.to_string()).unwrap().as_ptr(),
                ptr::null_mut(),
            );
        }
    }
}

/// Show the day details if a bar is pressed
#[unsafe(no_mangle)]
pub unsafe extern "C" fn history_chart_bar_pressed_cb(event: *mut lv_event_t) {
    unsafe {
        let chart_obj = lv_event_get_target(event);

        let day = lv_chart_get_pressed_point(chart_obj);

        set_var_hist_det_day(day as _);
    }
}

/// Create the history yield bar and the pmax line charts. This MUST be called while holding the lv lock
pub unsafe fn create_history_charts() {
    unsafe {
        let chart = objects.chart_history;

        lv_obj_set_style_width(chart, 0, LV_PART_INDICATOR);
        lv_chart_set_type(chart, LV_CHART_TYPE_BAR as lv_chart_type_t);
        lv_chart_set_update_mode(chart, LV_CHART_UPDATE_MODE_SHIFT as lv_chart_update_mode_t);
        lv_chart_set_range(chart, LV_CHART_AXIS_PRIMARY_Y as lv_chart_axis_t, 0, 1200);
        lv_chart_set_axis_tick(
            chart,
            LV_CHART_AXIS_PRIMARY_Y as lv_chart_axis_t,
            10,
            5,
            5,
            5,
            true,
            48,
        );
        lv_chart_set_axis_tick(
            chart,
            LV_CHART_AXIS_PRIMARY_X as lv_chart_axis_t,
            10,
            5,
            DAYS as _,
            1,
            true,
            30,
        );
        lv_chart_set_div_line_count(chart, 3, 0);
        lv_obj_set_style_pad_column(chart, 8, LV_PART_MAIN);
        lv_obj_set_style_pad_column(chart, 0, LV_PART_ITEMS);
        lv_obj_set_style_line_color(
            chart,
            lv_palette_main(lv_palette_t_LV_PALETTE_BLUE),
            LV_PART_TICKS,
        );
        lv_obj_set_style_text_color(
            chart,
            lv_palette_main(lv_palette_t_LV_PALETTE_BLUE),
            LV_PART_TICKS,
        );

        lv_obj_add_event_cb(
            chart,
            Some(history_chart_draw_event_begin_cb),
            lv_event_code_t_LV_EVENT_DRAW_PART_BEGIN,
            ptr::null_mut(),
        );

        lv_obj_add_event_cb(
            chart,
            Some(history_chart_draw_event_end_cb),
            lv_event_code_t_LV_EVENT_DRAW_PART_END,
            ptr::null_mut(),
        );

        lv_obj_add_event_cb(
            chart,
            Some(history_chart_bar_pressed_cb),
            lv_event_code_t_LV_EVENT_VALUE_CHANGED,
            ptr::null_mut(),
        );

        lv_chart_set_point_count(chart, mppt::DAYS as _);
        let series1 = lv_chart_add_series(
            chart,
            lv_palette_main(lv_palette_t_LV_PALETTE_YELLOW),
            LV_CHART_AXIS_PRIMARY_Y as lv_chart_axis_t,
        );
        lv_chart_set_all_value(chart, series1, i16::MAX);

        // Create the pmax line chart
        create_hist_pmax_chart();
    }
}

/// The pmax line chart has it's opacity reduced because it 'covers'
/// the bar chart but we want the labels to be full
#[unsafe(no_mangle)]
pub unsafe extern "C" fn history_chart_pmax_draw_event_begin_cb(event: *mut lv_event_t) {
    unsafe {
        let dsc = *lv_event_get_draw_part_dsc(event);

        if dsc.part == LV_PART_TICKS {
            if !dsc.label_dsc.is_null() {
                (*dsc.label_dsc).opa = LV_OPA_COVER as _;
            }
            if !dsc.line_dsc.is_null() {
                (*dsc.line_dsc).opa = LV_OPA_COVER as _;
            }
        }
    }
}
/// Create the pmax chart. This MUST be called while holding the lv lock
unsafe fn create_hist_pmax_chart() {
    unsafe {
        let chart = objects.chart_pmax;

        lv_obj_set_style_width(chart, 0, LV_PART_INDICATOR);
        lv_chart_set_type(chart, LV_CHART_TYPE_LINE as lv_chart_type_t);
        lv_chart_set_update_mode(chart, LV_CHART_UPDATE_MODE_SHIFT as lv_chart_update_mode_t);
        lv_chart_set_range(chart, LV_CHART_AXIS_SECONDARY_Y as lv_chart_axis_t, 0, 400);
        lv_chart_set_axis_tick(
            chart,
            LV_CHART_AXIS_SECONDARY_Y as lv_chart_axis_t,
            10,
            5,
            5,
            5,
            true,
            40,
        );
        lv_chart_set_div_line_count(chart, 3, 0);
        lv_obj_set_style_line_color(
            chart,
            lv_palette_main(lv_palette_t_LV_PALETTE_GREEN),
            LV_PART_TICKS,
        );
        lv_obj_set_style_text_color(
            chart,
            lv_palette_main(lv_palette_t_LV_PALETTE_GREEN),
            LV_PART_TICKS,
        );

        lv_obj_add_event_cb(
            chart,
            Some(history_chart_pmax_draw_event_begin_cb),
            lv_event_code_t_LV_EVENT_DRAW_PART_BEGIN,
            ptr::null_mut(),
        );

        lv_chart_set_point_count(chart, mppt::DAYS as _);
        let series1 = lv_chart_add_series(
            chart,
            lv_palette_main(lv_palette_t_LV_PALETTE_GREEN),
            LV_CHART_AXIS_SECONDARY_Y as lv_chart_axis_t,
        );
        lv_chart_set_all_value(chart, series1, i16::MAX);
    }
}

/// Set the day bar and pmax line chart values, and the lifetime value
pub fn update_history_charts(history: &[Option<HistoryDay>], lifetime: Option<&HistoryLifetime>) {
    unsafe {
        if lvgl_port_lock(-1) {
            let obj = objects.hist_det_lifetime;
            let lifetime_yield = format!(
                "{:.1}",
                lifetime
                    .map(|life| life.lifetime_yield as f32 / 1000.0)
                    .unwrap_or(0_f32)
            );
            lv_label_set_text(obj, CString::new(lifetime_yield).unwrap().as_ptr());

            let chart_power = objects.chart_history;
            let chart_pmax = objects.chart_pmax;

            let ser_power = lv_chart_get_series_next(chart_power, ptr::null());
            let ser_pmax = lv_chart_get_series_next(chart_pmax, ptr::null());

            let mut max_power: i16 = 0;

            for day in 0..DAYS {
                let (yield_, p_max) = if let Some(Some(hist)) = history.get(day) {
                    max_power = max_power.max(hist.yield_ as _);

                    (hist.yield_ as _, hist.p_max as _)
                } else {
                    (i16::MAX, i16::MAX)
                };

                lv_chart_set_next_value(chart_power, ser_power, yield_ as _);
                lv_chart_set_next_value(chart_pmax, ser_pmax, p_max as _);
            }

            lv_chart_set_range(
                chart_power,
                LV_CHART_AXIS_PRIMARY_Y as lv_chart_axis_t,
                0,
                max_power + 50,
            );

            lv_chart_refresh(chart_power);
            lv_chart_refresh(chart_pmax);

            lvgl_port_unlock();

            // Hide loading widget
            set_var_hist_det_day(-1);
        }
    }
}

/// Set all the history detail values for the clicked day bar
pub unsafe fn history_details(history: &HistoryDay) {
    unsafe {
        if lvgl_port_lock(-1) {
            let obj = objects.hist_det_day;
            let day = history.day;
            if day == 0 {
                lv_label_set_text(obj, c"Today".as_ptr());
            } else if day == 1 {
                lv_label_set_text(obj, c"Yesterday".as_ptr());
            } else {
                lv_label_set_text(
                    obj,
                    CString::new(format!("{day} Days Ago")).unwrap().as_ptr(),
                );
            };

            let obj = objects.hist_det_yield;
            lv_label_set_text(
                obj,
                CString::new(history.yield_.to_string()).unwrap().as_ptr(),
            );

            let obj = objects.hist_det_pmax;
            lv_label_set_text(
                obj,
                CString::new(history.p_max.to_string()).unwrap().as_ptr(),
            );

            let obj = objects.hist_det_vmax;
            lv_label_set_text(
                obj,
                CString::new(format!("{:.2}", history.v_max))
                    .unwrap()
                    .as_ptr(),
            );

            let bulk_secs: u16 = history.bulk.as_secs() as _;
            let bulk_hours = bulk_secs / 3600;
            let bulk_mins = (bulk_secs % 3600) / 60;

            let obj = objects.hist_det_bulk;
            lv_label_set_text(
                obj,
                CString::new(if bulk_hours > 0 {
                    format!("{bulk_hours}h {bulk_mins}m")
                } else {
                    format!("{bulk_mins}m")
                })
                .unwrap()
                .as_ptr(),
            );

            let abs_secs: u16 = history.abs.as_secs() as _;
            let abs_hours = abs_secs / 3600;
            let abs_mins = (abs_secs % 3600) / 60;

            let obj = objects.hist_det_abs;
            lv_label_set_text(
                obj,
                CString::new(if abs_hours > 0 {
                    format!("{abs_hours}h {abs_mins}m")
                } else {
                    format!("{abs_mins}m")
                })
                .unwrap()
                .as_ptr(),
            );

            let float_secs: u16 = history.float.as_secs() as _;
            let float_hours = float_secs / 3600;
            let float_mins = (float_secs % 3600) / 60;

            let obj = objects.hist_det_float;
            lv_label_set_text(
                obj,
                CString::new(if float_hours > 0 {
                    format!("{float_hours}h {float_mins}m")
                } else {
                    format!("{float_mins}m")
                })
                .unwrap()
                .as_ptr(),
            );

            let obj = objects.hist_det_bat_max;
            lv_label_set_text(
                obj,
                CString::new(format!("{:.2}", history.bat_max))
                    .unwrap()
                    .as_ptr(),
            );

            let obj = objects.hist_det_bat_min;
            lv_label_set_text(
                obj,
                CString::new(format!("{:.2}", history.bat_min))
                    .unwrap()
                    .as_ptr(),
            );

            let obj = objects.hist_det_errors;
            lv_label_set_text(
                obj,
                if history.errors == MpptError::None {
                    c"-".as_ptr()
                } else {
                    CString::new(history.errors.to_string()).unwrap().as_ptr()
                },
            );

            lvgl_port_unlock();
        }
    }
}
