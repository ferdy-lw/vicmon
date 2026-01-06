#include <string.h>

#include "screens.h"
#include "images.h"
#include "fonts.h"
#include "actions.h"
#include "vars.h"
#include "styles.h"
#include "ui.h"

#include <string.h>

objects_t objects;
lv_obj_t *tick_value_change_obj;

static void event_handler_cb_main_obj0(lv_event_t *e) {
    lv_event_code_t event = lv_event_get_code(e);
    void *flowState = lv_event_get_user_data(e);
    (void)flowState;
    
    if (event == LV_EVENT_VALUE_CHANGED) {
        lv_obj_t *ta = lv_event_get_target(e);
        if (tick_value_change_obj != ta) {
            bool value = lv_obj_has_state(ta, LV_STATE_CHECKED);
            assignBooleanProperty(flowState, 3, 3, value, "Failed to assign Checked state");
        }
    }
}

static void event_handler_cb_main_ac_watts_arc(lv_event_t *e) {
    lv_event_code_t event = lv_event_get_code(e);
    void *flowState = lv_event_get_user_data(e);
    (void)flowState;
    
    if (event == LV_EVENT_VALUE_CHANGED) {
        lv_obj_t *ta = lv_event_get_target(e);
        if (tick_value_change_obj != ta) {
            int32_t value = lv_arc_get_value(ta);
            assignIntegerProperty(flowState, 5, 3, value, "Failed to assign Value in Arc widget");
        }
    }
}

static void event_handler_cb_main_soc(lv_event_t *e) {
    lv_event_code_t event = lv_event_get_code(e);
    void *flowState = lv_event_get_user_data(e);
    (void)flowState;
    
    if (event == LV_EVENT_VALUE_CHANGED) {
        lv_obj_t *ta = lv_event_get_target(e);
        if (tick_value_change_obj != ta) {
            int32_t value = lv_arc_get_value(ta);
            assignIntegerProperty(flowState, 13, 3, value, "Failed to assign Value in Arc widget");
        }
    }
}

static void event_handler_cb_main_pv_power(lv_event_t *e) {
    lv_event_code_t event = lv_event_get_code(e);
    void *flowState = lv_event_get_user_data(e);
    (void)flowState;
    
    if (event == LV_EVENT_VALUE_CHANGED) {
        lv_obj_t *ta = lv_event_get_target(e);
        if (tick_value_change_obj != ta) {
            int32_t value = lv_arc_get_value(ta);
            assignIntegerProperty(flowState, 35, 3, value, "Failed to assign Value in Arc widget");
        }
    }
}

static void event_handler_cb_main_yield(lv_event_t *e) {
    lv_event_code_t event = lv_event_get_code(e);
    void *flowState = lv_event_get_user_data(e);
    (void)flowState;
    
    if (event == LV_EVENT_VALUE_CHANGED) {
        lv_obj_t *ta = lv_event_get_target(e);
        if (tick_value_change_obj != ta) {
            int32_t value = lv_arc_get_value(ta);
            assignIntegerProperty(flowState, 41, 3, value, "Failed to assign Value in Arc widget");
        }
    }
}

static void event_handler_cb_main_go_config(lv_event_t *e) {
    lv_event_code_t event = lv_event_get_code(e);
    void *flowState = lv_event_get_user_data(e);
    (void)flowState;
    
    if (event == LV_EVENT_CLICKED) {
        e->user_data = (void *)0;
        flowPropagateValueLVGLEvent(flowState, 48, 0, e);
    }
}

static void event_handler_cb_config_go_main(lv_event_t *e) {
    lv_event_code_t event = lv_event_get_code(e);
    void *flowState = lv_event_get_user_data(e);
    (void)flowState;
    
    if (event == LV_EVENT_CLICKED) {
        e->user_data = (void *)0;
        flowPropagateValueLVGLEvent(flowState, 0, 0, e);
    }
}

static void event_handler_cb_config_wifi_btn(lv_event_t *e) {
    lv_event_code_t event = lv_event_get_code(e);
    void *flowState = lv_event_get_user_data(e);
    (void)flowState;
    
}

static void event_handler_cb_config_obj12(lv_event_t *e) {
    lv_event_code_t event = lv_event_get_code(e);
    void *flowState = lv_event_get_user_data(e);
    (void)flowState;
    
    if (event == LV_EVENT_VALUE_CHANGED) {
        lv_obj_t *ta = lv_event_get_target(e);
        if (tick_value_change_obj != ta) {
            int32_t value = lv_slider_get_value(ta);
            assignIntegerProperty(flowState, 10, 3, value, "Failed to assign Value in Slider widget");
        }
    }
}

void create_screen_main() {
    void *flowState = getFlowState(0, 0);
    (void)flowState;
    lv_obj_t *obj = lv_obj_create(0);
    objects.main = obj;
    lv_obj_set_pos(obj, 0, 0);
    lv_obj_set_size(obj, 800, 480);
    lv_obj_clear_flag(obj, LV_OBJ_FLAG_SCROLLABLE);
    lv_obj_set_style_bg_color(obj, lv_color_hex(0xff000000), LV_PART_MAIN | LV_STATE_DEFAULT);
    {
        lv_obj_t *parent_obj = obj;
        {
            // inverter_container
            lv_obj_t *obj = lv_obj_create(parent_obj);
            objects.inverter_container = obj;
            lv_obj_set_pos(obj, 500, 0);
            lv_obj_set_size(obj, 300, 480);
            lv_obj_set_style_pad_left(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_top(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_right(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_bottom(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_bg_opa(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_border_width(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_radius(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_add_flag(obj, LV_OBJ_FLAG_EVENT_BUBBLE);
            {
                lv_obj_t *parent_obj = obj;
                {
                    lv_obj_t *obj = lv_label_create(parent_obj);
                    lv_obj_set_pos(obj, 40, 60);
                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                    add_style_labels(obj);
                    lv_obj_set_style_text_font(obj, &ui_font_roboto_reg_48, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_label_set_text(obj, "INVERTER");
                }
                {
                    lv_obj_t *obj = lv_switch_create(parent_obj);
                    objects.obj0 = obj;
                    lv_obj_set_pos(obj, 60, 147);
                    lv_obj_set_size(obj, 180, 70);
                    lv_obj_add_event_cb(obj, event_handler_cb_main_obj0, LV_EVENT_ALL, flowState);
                    lv_obj_set_style_bg_color(obj, lv_color_hex(0xff4789d0), LV_PART_INDICATOR | LV_STATE_DEFAULT);
                    lv_obj_set_style_bg_color(obj, lv_color_hex(0xfff2f2f2), LV_PART_KNOB | LV_STATE_DEFAULT);
                    lv_obj_set_style_bg_color(obj, lv_color_hex(0xff222629), LV_PART_MAIN | LV_STATE_DEFAULT);
                }
                {
                    lv_obj_t *obj = lv_label_create(parent_obj);
                    objects.obj3 = obj;
                    lv_obj_set_pos(obj, 0, 230);
                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                    add_style_labels(obj);
                    lv_obj_set_style_align(obj, LV_ALIGN_TOP_MID, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_obj_set_style_text_font(obj, &ui_font_roboto_reg_14, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_label_set_text(obj, "");
                }
                {
                    // ac_watts_arc
                    lv_obj_t *obj = lv_arc_create(parent_obj);
                    objects.ac_watts_arc = obj;
                    lv_obj_set_pos(obj, 60, 255);
                    lv_obj_set_size(obj, 180, 180);
                    lv_arc_set_range(obj, 0, 1500);
                    lv_arc_set_bg_start_angle(obj, 0);
                    lv_arc_set_bg_end_angle(obj, 360);
                    lv_arc_set_rotation(obj, 90);
                    lv_obj_add_event_cb(obj, event_handler_cb_main_ac_watts_arc, LV_EVENT_ALL, flowState);
                    lv_obj_clear_flag(obj, LV_OBJ_FLAG_CLICKABLE);
                    add_style_arcs(obj);
                    {
                        lv_obj_t *parent_obj = obj;
                        {
                            lv_obj_t *obj = lv_obj_create(parent_obj);
                            lv_obj_set_pos(obj, 0, 0);
                            lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                            lv_obj_set_style_pad_left(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_pad_top(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_pad_right(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_pad_bottom(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_bg_opa(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_border_width(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_radius(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_clear_flag(obj, LV_OBJ_FLAG_SCROLLABLE);
                            lv_obj_set_style_align(obj, LV_ALIGN_CENTER, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_layout(obj, LV_LAYOUT_FLEX, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_flex_main_place(obj, LV_FLEX_ALIGN_START, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_pad_column(obj, 1, LV_PART_MAIN | LV_STATE_DEFAULT);
                            {
                                lv_obj_t *parent_obj = obj;
                                {
                                    lv_obj_t *obj = lv_label_create(parent_obj);
                                    objects.obj4 = obj;
                                    lv_obj_set_pos(obj, 0, 0);
                                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                                    add_style_labels(obj);
                                    lv_obj_set_style_text_font(obj, &ui_font_roboto_med_48, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_label_set_text(obj, "");
                                }
                                {
                                    lv_obj_t *obj = lv_label_create(parent_obj);
                                    lv_obj_set_pos(obj, 82, 10);
                                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                                    add_style_labels(obj);
                                    lv_obj_set_style_text_font(obj, &ui_font_roboto_reg_32, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_obj_set_style_pad_top(obj, 9, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_label_set_text(obj, "W");
                                }
                            }
                        }
                        {
                            lv_obj_t *obj = lv_img_create(parent_obj);
                            objects.obj1 = obj;
                            lv_obj_set_pos(obj, 79, 110);
                            lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                            lv_img_set_src(obj, &img_power);
                            add_style_images(obj);
                            lv_obj_set_style_img_recolor(obj, lv_color_hex(0xfff5a3a3), LV_PART_MAIN | LV_STATE_DEFAULT);
                        }
                    }
                }
                {
                    // inv_error
                    lv_obj_t *obj = lv_label_create(parent_obj);
                    objects.inv_error = obj;
                    lv_obj_set_pos(obj, 0, 439);
                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                    add_style_labels_error(obj);
                    lv_obj_set_style_align(obj, LV_ALIGN_TOP_MID, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_label_set_text(obj, "");
                }
            }
        }
        {
            // solar_container
            lv_obj_t *obj = lv_obj_create(parent_obj);
            objects.solar_container = obj;
            lv_obj_set_pos(obj, 0, 0);
            lv_obj_set_size(obj, 500, 480);
            lv_obj_set_style_pad_left(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_top(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_right(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_bottom(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_bg_opa(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_border_width(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_radius(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_add_flag(obj, LV_OBJ_FLAG_EVENT_BUBBLE);
            {
                lv_obj_t *parent_obj = obj;
                {
                    lv_obj_t *obj = lv_label_create(parent_obj);
                    lv_obj_set_pos(obj, 176, 60);
                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                    add_style_labels(obj);
                    lv_obj_set_style_text_font(obj, &ui_font_roboto_reg_48, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_label_set_text(obj, "SOLAR");
                }
                {
                    // soc
                    lv_obj_t *obj = lv_arc_create(parent_obj);
                    objects.soc = obj;
                    lv_obj_set_pos(obj, 120, 125);
                    lv_obj_set_size(obj, 260, 260);
                    lv_arc_set_bg_start_angle(obj, 0);
                    lv_arc_set_bg_end_angle(obj, 360);
                    lv_arc_set_rotation(obj, 90);
                    lv_obj_add_event_cb(obj, event_handler_cb_main_soc, LV_EVENT_ALL, flowState);
                    lv_obj_clear_flag(obj, LV_OBJ_FLAG_CLICKABLE);
                    add_style_arcs(obj);
                    {
                        lv_obj_t *parent_obj = obj;
                        {
                            lv_obj_t *obj = lv_label_create(parent_obj);
                            objects.obj5 = obj;
                            lv_obj_set_pos(obj, 0, 25);
                            lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                            add_style_labels(obj);
                            lv_obj_set_style_align(obj, LV_ALIGN_TOP_MID, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_text_font(obj, &ui_font_roboto_reg_14, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_label_set_text(obj, "");
                        }
                        {
                            lv_obj_t *obj = lv_obj_create(parent_obj);
                            lv_obj_set_pos(obj, 0, -65);
                            lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                            lv_obj_set_style_pad_left(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_pad_top(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_pad_right(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_pad_bottom(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_bg_opa(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_border_width(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_radius(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_clear_flag(obj, LV_OBJ_FLAG_SCROLLABLE);
                            lv_obj_set_style_align(obj, LV_ALIGN_CENTER, LV_PART_MAIN | LV_STATE_DEFAULT);
                            {
                                lv_obj_t *parent_obj = obj;
                                {
                                    // batt_indicator_image
                                    lv_obj_t *obj = lv_img_create(parent_obj);
                                    objects.batt_indicator_image = obj;
                                    lv_obj_set_pos(obj, 0, 0);
                                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                                    lv_img_set_src(obj, &img_batt_90);
                                    add_style_images(obj);
                                    lv_obj_set_style_img_recolor(obj, lv_color_hex(0xffa3caf5), LV_PART_MAIN | LV_STATE_DEFAULT);
                                }
                                {
                                    lv_obj_t *obj = lv_label_create(parent_obj);
                                    lv_obj_set_pos(obj, 25, 11);
                                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                                    add_style_labels(obj);
                                    lv_obj_set_style_text_font(obj, &ui_font_roboto_reg_14, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_label_set_text(obj, "Battery");
                                }
                            }
                        }
                        {
                            // batt_alarm
                            lv_obj_t *obj = lv_label_create(parent_obj);
                            objects.batt_alarm = obj;
                            lv_obj_set_pos(obj, 0, 82);
                            lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                            add_style_labels_error(obj);
                            lv_obj_set_style_align(obj, LV_ALIGN_TOP_MID, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_label_set_text(obj, "");
                        }
                        {
                            // soc_unknown_container
                            lv_obj_t *obj = lv_obj_create(parent_obj);
                            objects.soc_unknown_container = obj;
                            lv_obj_set_pos(obj, 0, 0);
                            lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                            lv_obj_set_style_pad_left(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_pad_top(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_pad_right(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_pad_bottom(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_bg_opa(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_border_width(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_radius(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_add_flag(obj, LV_OBJ_FLAG_HIDDEN);
                            lv_obj_clear_flag(obj, LV_OBJ_FLAG_SCROLLABLE);
                            lv_obj_set_style_align(obj, LV_ALIGN_CENTER, LV_PART_MAIN | LV_STATE_DEFAULT);
                            {
                                lv_obj_t *parent_obj = obj;
                                {
                                    lv_obj_t *obj = lv_label_create(parent_obj);
                                    lv_obj_set_pos(obj, 0, 0);
                                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                                    add_style_labels(obj);
                                    lv_obj_set_style_text_font(obj, &ui_font_roboto_med_48, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_label_set_text(obj, "~");
                                }
                            }
                        }
                        {
                            // soc_container
                            lv_obj_t *obj = lv_obj_create(parent_obj);
                            objects.soc_container = obj;
                            lv_obj_set_pos(obj, 0, 0);
                            lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                            lv_obj_set_style_pad_left(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_pad_top(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_pad_right(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_pad_bottom(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_bg_opa(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_border_width(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_radius(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_clear_flag(obj, LV_OBJ_FLAG_SCROLLABLE);
                            lv_obj_set_style_align(obj, LV_ALIGN_CENTER, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_layout(obj, LV_LAYOUT_FLEX, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_pad_column(obj, 1, LV_PART_MAIN | LV_STATE_DEFAULT);
                            {
                                lv_obj_t *parent_obj = obj;
                                {
                                    lv_obj_t *obj = lv_label_create(parent_obj);
                                    objects.obj6 = obj;
                                    lv_obj_set_pos(obj, 0, 0);
                                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                                    add_style_labels(obj);
                                    lv_obj_set_style_text_font(obj, &ui_font_roboto_med_48, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_label_set_text(obj, "");
                                }
                                {
                                    lv_obj_t *obj = lv_label_create(parent_obj);
                                    lv_obj_set_pos(obj, 49, 10);
                                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                                    add_style_labels(obj);
                                    lv_obj_set_style_text_font(obj, &ui_font_roboto_reg_32, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_obj_set_style_pad_top(obj, 8, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_label_set_text(obj, "%");
                                }
                            }
                        }
                        {
                            lv_obj_t *obj = lv_obj_create(parent_obj);
                            lv_obj_set_pos(obj, 0, 55);
                            lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                            lv_obj_set_style_pad_left(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_pad_top(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_pad_right(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_pad_bottom(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_bg_opa(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_border_width(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_radius(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_clear_flag(obj, LV_OBJ_FLAG_SCROLLABLE);
                            lv_obj_set_style_layout(obj, LV_LAYOUT_FLEX, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_pad_column(obj, 8, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_align(obj, LV_ALIGN_CENTER, LV_PART_MAIN | LV_STATE_DEFAULT);
                            {
                                lv_obj_t *parent_obj = obj;
                                {
                                    lv_obj_t *obj = lv_obj_create(parent_obj);
                                    lv_obj_set_pos(obj, 0, 0);
                                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                                    lv_obj_set_style_pad_left(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_obj_set_style_pad_top(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_obj_set_style_pad_right(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_obj_set_style_pad_bottom(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_obj_set_style_bg_opa(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_obj_set_style_border_width(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_obj_set_style_radius(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_obj_clear_flag(obj, LV_OBJ_FLAG_SCROLLABLE);
                                    lv_obj_set_style_layout(obj, LV_LAYOUT_FLEX, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_obj_set_style_pad_column(obj, 1, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    {
                                        lv_obj_t *parent_obj = obj;
                                        {
                                            lv_obj_t *obj = lv_label_create(parent_obj);
                                            objects.obj7 = obj;
                                            lv_obj_set_pos(obj, 0, 0);
                                            lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                                            add_style_labels(obj);
                                            lv_obj_set_style_text_font(obj, &ui_font_roboto_med_22, LV_PART_MAIN | LV_STATE_DEFAULT);
                                            lv_label_set_text(obj, "");
                                        }
                                        {
                                            lv_obj_t *obj = lv_label_create(parent_obj);
                                            lv_obj_set_pos(obj, 58, 5);
                                            lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                                            add_style_labels(obj);
                                            lv_obj_set_style_text_font(obj, &ui_font_roboto_reg_14, LV_PART_MAIN | LV_STATE_DEFAULT);
                                            lv_obj_set_style_pad_top(obj, 5, LV_PART_MAIN | LV_STATE_DEFAULT);
                                            lv_label_set_text(obj, "V");
                                        }
                                    }
                                }
                                {
                                    lv_obj_t *obj = lv_obj_create(parent_obj);
                                    lv_obj_set_pos(obj, 0, 0);
                                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                                    lv_obj_set_style_pad_left(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_obj_set_style_pad_top(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_obj_set_style_pad_right(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_obj_set_style_pad_bottom(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_obj_set_style_bg_opa(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_obj_set_style_border_width(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_obj_set_style_radius(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_obj_clear_flag(obj, LV_OBJ_FLAG_SCROLLABLE);
                                    lv_obj_set_style_layout(obj, LV_LAYOUT_FLEX, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_obj_set_style_pad_column(obj, 1, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    {
                                        lv_obj_t *parent_obj = obj;
                                        {
                                            lv_obj_t *obj = lv_label_create(parent_obj);
                                            objects.obj8 = obj;
                                            lv_obj_set_pos(obj, 0, 0);
                                            lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                                            add_style_labels(obj);
                                            lv_obj_set_style_text_font(obj, &ui_font_roboto_med_22, LV_PART_MAIN | LV_STATE_DEFAULT);
                                            lv_label_set_text(obj, "");
                                        }
                                        {
                                            lv_obj_t *obj = lv_label_create(parent_obj);
                                            lv_obj_set_pos(obj, 0, 0);
                                            lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                                            add_style_labels(obj);
                                            lv_obj_set_style_text_font(obj, &ui_font_roboto_reg_14, LV_PART_MAIN | LV_STATE_DEFAULT);
                                            lv_obj_set_style_pad_top(obj, 5, LV_PART_MAIN | LV_STATE_DEFAULT);
                                            lv_label_set_text(obj, "A");
                                        }
                                    }
                                }
                            }
                        }
                        {
                            lv_obj_t *obj = lv_obj_create(parent_obj);
                            lv_obj_set_pos(obj, 0, 100);
                            lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                            lv_obj_set_style_pad_left(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_pad_top(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_pad_right(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_pad_bottom(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_bg_opa(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_border_width(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_radius(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_clear_flag(obj, LV_OBJ_FLAG_SCROLLABLE);
                            lv_obj_set_style_pad_column(obj, 1, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_layout(obj, LV_LAYOUT_FLEX, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_align(obj, LV_ALIGN_CENTER, LV_PART_MAIN | LV_STATE_DEFAULT);
                            {
                                lv_obj_t *parent_obj = obj;
                                {
                                    lv_obj_t *obj = lv_img_create(parent_obj);
                                    objects.obj2 = obj;
                                    lv_obj_set_pos(obj, 0, 0);
                                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                                    lv_img_set_src(obj, &img_temp);
                                    add_style_images(obj);
                                    lv_obj_set_style_img_recolor(obj, lv_color_hex(0xfff5c5a3), LV_PART_MAIN | LV_STATE_DEFAULT);
                                }
                                {
                                    lv_obj_t *obj = lv_label_create(parent_obj);
                                    objects.obj9 = obj;
                                    lv_obj_set_pos(obj, 19, 4);
                                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                                    add_style_labels(obj);
                                    lv_obj_set_style_text_font(obj, &ui_font_roboto_med_18, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_obj_set_style_pad_top(obj, 4, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_label_set_text(obj, "");
                                }
                                {
                                    lv_obj_t *obj = lv_label_create(parent_obj);
                                    lv_obj_set_pos(obj, 39, 6);
                                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                                    add_style_labels(obj);
                                    lv_obj_set_style_text_font(obj, &ui_font_roboto_reg_14, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_obj_set_style_pad_top(obj, 6, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_label_set_text(obj, "C");
                                }
                            }
                        }
                    }
                }
                {
                    // pv_power
                    lv_obj_t *obj = lv_arc_create(parent_obj);
                    objects.pv_power = obj;
                    lv_obj_set_pos(obj, 75, 80);
                    lv_obj_set_size(obj, 350, 350);
                    lv_arc_set_range(obj, 0, 400);
                    lv_arc_set_bg_start_angle(obj, 0);
                    lv_arc_set_bg_end_angle(obj, 90);
                    lv_arc_set_rotation(obj, 135);
                    lv_obj_add_event_cb(obj, event_handler_cb_main_pv_power, LV_EVENT_ALL, flowState);
                    lv_obj_clear_flag(obj, LV_OBJ_FLAG_CLICKABLE);
                    add_style_arcs(obj);
                }
                {
                    lv_obj_t *obj = lv_obj_create(parent_obj);
                    lv_obj_set_pos(obj, 0, 361);
                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                    lv_obj_set_style_pad_left(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_obj_set_style_pad_top(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_obj_set_style_pad_right(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_obj_set_style_pad_bottom(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_obj_set_style_bg_opa(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_obj_set_style_border_width(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_obj_set_style_radius(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_obj_clear_flag(obj, LV_OBJ_FLAG_SCROLLABLE);
                    {
                        lv_obj_t *parent_obj = obj;
                        {
                            // image_solar
                            lv_obj_t *obj = lv_img_create(parent_obj);
                            objects.image_solar = obj;
                            lv_obj_set_pos(obj, 88, 9);
                            lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                            lv_img_set_src(obj, &img_solar);
                            add_style_images(obj);
                            lv_obj_set_style_img_recolor(obj, lv_color_hex(0xffffff00), LV_PART_MAIN | LV_STATE_DEFAULT);
                        }
                        {
                            lv_obj_t *obj = lv_obj_create(parent_obj);
                            lv_obj_set_pos(obj, 0, 35);
                            lv_obj_set_size(obj, 120, LV_SIZE_CONTENT);
                            lv_obj_set_style_pad_left(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_pad_top(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_pad_right(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_pad_bottom(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_bg_opa(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_border_width(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_radius(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_clear_flag(obj, LV_OBJ_FLAG_SCROLLABLE);
                            lv_obj_set_style_layout(obj, LV_LAYOUT_FLEX, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_flex_main_place(obj, LV_FLEX_ALIGN_END, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_pad_column(obj, 1, LV_PART_MAIN | LV_STATE_DEFAULT);
                            {
                                lv_obj_t *parent_obj = obj;
                                {
                                    lv_obj_t *obj = lv_label_create(parent_obj);
                                    objects.obj10 = obj;
                                    lv_obj_set_pos(obj, 0, 30);
                                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                                    add_style_labels(obj);
                                    lv_obj_set_style_text_font(obj, &ui_font_roboto_med_48, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_label_set_text(obj, "");
                                }
                                {
                                    lv_obj_t *obj = lv_label_create(parent_obj);
                                    lv_obj_set_pos(obj, 82, 40);
                                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                                    add_style_labels(obj);
                                    lv_obj_set_style_text_font(obj, &ui_font_roboto_reg_32, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_obj_set_style_pad_top(obj, 9, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_label_set_text(obj, "W");
                                }
                            }
                        }
                    }
                }
                {
                    // yield
                    lv_obj_t *obj = lv_arc_create(parent_obj);
                    objects.yield = obj;
                    lv_obj_set_pos(obj, 75, 80);
                    lv_obj_set_size(obj, 350, 350);
                    lv_arc_set_range(obj, 0, 850);
                    lv_arc_set_bg_start_angle(obj, 0);
                    lv_arc_set_bg_end_angle(obj, 90);
                    lv_arc_set_mode(obj, LV_ARC_MODE_REVERSE);
                    lv_arc_set_rotation(obj, 315);
                    lv_obj_add_event_cb(obj, event_handler_cb_main_yield, LV_EVENT_ALL, flowState);
                    lv_obj_clear_flag(obj, LV_OBJ_FLAG_CLICKABLE);
                    add_style_arcs(obj);
                }
                {
                    lv_obj_t *obj = lv_obj_create(parent_obj);
                    lv_obj_set_pos(obj, 331, 361);
                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                    lv_obj_set_style_pad_left(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_obj_set_style_pad_top(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_obj_set_style_pad_right(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_obj_set_style_pad_bottom(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_obj_set_style_bg_opa(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_obj_set_style_border_width(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_obj_set_style_radius(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_obj_clear_flag(obj, LV_OBJ_FLAG_SCROLLABLE);
                    {
                        lv_obj_t *parent_obj = obj;
                        {
                            // image_sun
                            lv_obj_t *obj = lv_img_create(parent_obj);
                            objects.image_sun = obj;
                            lv_obj_set_pos(obj, 51, 9);
                            lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                            lv_img_set_src(obj, &img_sun);
                            add_style_images(obj);
                            lv_obj_set_style_img_recolor(obj, lv_color_hex(0xffffff00), LV_PART_MAIN | LV_STATE_DEFAULT);
                        }
                        {
                            lv_obj_t *obj = lv_obj_create(parent_obj);
                            lv_obj_set_pos(obj, 0, 35);
                            lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                            lv_obj_set_style_pad_left(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_pad_top(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_pad_right(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_pad_bottom(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_bg_opa(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_border_width(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_radius(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_clear_flag(obj, LV_OBJ_FLAG_SCROLLABLE);
                            lv_obj_set_style_pad_column(obj, 1, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_layout(obj, LV_LAYOUT_FLEX, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_obj_set_style_flex_main_place(obj, LV_FLEX_ALIGN_START, LV_PART_MAIN | LV_STATE_DEFAULT);
                            {
                                lv_obj_t *parent_obj = obj;
                                {
                                    lv_obj_t *obj = lv_label_create(parent_obj);
                                    objects.obj11 = obj;
                                    lv_obj_set_pos(obj, 0, 30);
                                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                                    add_style_labels(obj);
                                    lv_obj_set_style_text_font(obj, &ui_font_roboto_med_48, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_label_set_text(obj, "");
                                }
                                {
                                    lv_obj_t *obj = lv_label_create(parent_obj);
                                    lv_obj_set_pos(obj, 82, 40);
                                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                                    add_style_labels(obj);
                                    lv_obj_set_style_text_font(obj, &ui_font_roboto_reg_32, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_obj_set_style_pad_top(obj, 9, LV_PART_MAIN | LV_STATE_DEFAULT);
                                    lv_label_set_text(obj, "Wh");
                                }
                            }
                        }
                    }
                }
                {
                    // solar_error
                    lv_obj_t *obj = lv_label_create(parent_obj);
                    objects.solar_error = obj;
                    lv_obj_set_pos(obj, 0, 439);
                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                    add_style_labels_error(obj);
                    lv_obj_set_style_align(obj, LV_ALIGN_TOP_MID, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_label_set_text(obj, "");
                }
            }
        }
        {
            // go_config
            lv_obj_t *obj = lv_img_create(parent_obj);
            objects.go_config = obj;
            lv_obj_set_pos(obj, 0, 0);
            lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
            lv_img_set_src(obj, &img_settings);
            lv_obj_add_event_cb(obj, event_handler_cb_main_go_config, LV_EVENT_ALL, flowState);
            lv_obj_add_flag(obj, LV_OBJ_FLAG_CLICKABLE);
            lv_obj_set_style_img_recolor(obj, lv_color_hex(0xff595858), LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_img_recolor_opa(obj, 255, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_top(obj, 5, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_bottom(obj, 5, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_left(obj, 5, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_right(obj, 5, LV_PART_MAIN | LV_STATE_DEFAULT);
        }
    }
    
    tick_screen_main();
}

void delete_screen_main() {
    lv_obj_del(objects.main);
    objects.main = 0;
    objects.inverter_container = 0;
    objects.obj0 = 0;
    objects.obj3 = 0;
    objects.ac_watts_arc = 0;
    objects.obj4 = 0;
    objects.obj1 = 0;
    objects.inv_error = 0;
    objects.solar_container = 0;
    objects.soc = 0;
    objects.obj5 = 0;
    objects.batt_indicator_image = 0;
    objects.batt_alarm = 0;
    objects.soc_unknown_container = 0;
    objects.soc_container = 0;
    objects.obj6 = 0;
    objects.obj7 = 0;
    objects.obj8 = 0;
    objects.obj2 = 0;
    objects.obj9 = 0;
    objects.pv_power = 0;
    objects.image_solar = 0;
    objects.obj10 = 0;
    objects.yield = 0;
    objects.image_sun = 0;
    objects.obj11 = 0;
    objects.solar_error = 0;
    objects.go_config = 0;
    deletePageFlowState(0);
}

void tick_screen_main() {
    void *flowState = getFlowState(0, 0);
    (void)flowState;
    {
        bool new_val = evalBooleanProperty(flowState, 3, 3, "Failed to evaluate Checked state");
        bool cur_val = lv_obj_has_state(objects.obj0, LV_STATE_CHECKED);
        if (new_val != cur_val) {
            tick_value_change_obj = objects.obj0;
            if (new_val) lv_obj_add_state(objects.obj0, LV_STATE_CHECKED);
            else lv_obj_clear_state(objects.obj0, LV_STATE_CHECKED);
            tick_value_change_obj = NULL;
        }
    }
    {
        const char *new_val = evalTextProperty(flowState, 4, 3, "Failed to evaluate Text in Label widget");
        const char *cur_val = lv_label_get_text(objects.obj3);
        if (strcmp(new_val, cur_val) != 0) {
            tick_value_change_obj = objects.obj3;
            lv_label_set_text(objects.obj3, new_val);
            tick_value_change_obj = NULL;
        }
    }
    {
        int32_t new_val = evalIntegerProperty(flowState, 5, 3, "Failed to evaluate Value in Arc widget");
        int32_t cur_val = lv_arc_get_value(objects.ac_watts_arc);
        if (new_val != cur_val) {
            tick_value_change_obj = objects.ac_watts_arc;
            lv_arc_set_value(objects.ac_watts_arc, new_val);
            tick_value_change_obj = NULL;
        }
    }
    {
        const char *new_val = evalTextProperty(flowState, 7, 3, "Failed to evaluate Text in Label widget");
        const char *cur_val = lv_label_get_text(objects.obj4);
        if (strcmp(new_val, cur_val) != 0) {
            tick_value_change_obj = objects.obj4;
            lv_label_set_text(objects.obj4, new_val);
            tick_value_change_obj = NULL;
        }
    }
    {
        const char *new_val = evalTextProperty(flowState, 10, 3, "Failed to evaluate Text in Label widget");
        const char *cur_val = lv_label_get_text(objects.inv_error);
        if (strcmp(new_val, cur_val) != 0) {
            tick_value_change_obj = objects.inv_error;
            lv_label_set_text(objects.inv_error, new_val);
            tick_value_change_obj = NULL;
        }
    }
    {
        int32_t new_val = evalIntegerProperty(flowState, 13, 3, "Failed to evaluate Value in Arc widget");
        int32_t cur_val = lv_arc_get_value(objects.soc);
        if (new_val != cur_val) {
            tick_value_change_obj = objects.soc;
            lv_arc_set_value(objects.soc, new_val);
            tick_value_change_obj = NULL;
        }
    }
    {
        const char *new_val = evalTextProperty(flowState, 14, 3, "Failed to evaluate Text in Label widget");
        const char *cur_val = lv_label_get_text(objects.obj5);
        if (strcmp(new_val, cur_val) != 0) {
            tick_value_change_obj = objects.obj5;
            lv_label_set_text(objects.obj5, new_val);
            tick_value_change_obj = NULL;
        }
    }
    {
        const char *new_val = evalTextProperty(flowState, 18, 3, "Failed to evaluate Text in Label widget");
        const char *cur_val = lv_label_get_text(objects.batt_alarm);
        if (strcmp(new_val, cur_val) != 0) {
            tick_value_change_obj = objects.batt_alarm;
            lv_label_set_text(objects.batt_alarm, new_val);
            tick_value_change_obj = NULL;
        }
    }
    {
        const char *new_val = evalTextProperty(flowState, 22, 3, "Failed to evaluate Text in Label widget");
        const char *cur_val = lv_label_get_text(objects.obj6);
        if (strcmp(new_val, cur_val) != 0) {
            tick_value_change_obj = objects.obj6;
            lv_label_set_text(objects.obj6, new_val);
            tick_value_change_obj = NULL;
        }
    }
    {
        const char *new_val = evalTextProperty(flowState, 26, 3, "Failed to evaluate Text in Label widget");
        const char *cur_val = lv_label_get_text(objects.obj7);
        if (strcmp(new_val, cur_val) != 0) {
            tick_value_change_obj = objects.obj7;
            lv_label_set_text(objects.obj7, new_val);
            tick_value_change_obj = NULL;
        }
    }
    {
        const char *new_val = evalTextProperty(flowState, 29, 3, "Failed to evaluate Text in Label widget");
        const char *cur_val = lv_label_get_text(objects.obj8);
        if (strcmp(new_val, cur_val) != 0) {
            tick_value_change_obj = objects.obj8;
            lv_label_set_text(objects.obj8, new_val);
            tick_value_change_obj = NULL;
        }
    }
    {
        const char *new_val = evalTextProperty(flowState, 34, 3, "Failed to evaluate Text in Label widget");
        const char *cur_val = lv_label_get_text(objects.obj9);
        if (strcmp(new_val, cur_val) != 0) {
            tick_value_change_obj = objects.obj9;
            lv_label_set_text(objects.obj9, new_val);
            tick_value_change_obj = NULL;
        }
    }
    {
        int32_t new_val = evalIntegerProperty(flowState, 35, 3, "Failed to evaluate Value in Arc widget");
        int32_t cur_val = lv_arc_get_value(objects.pv_power);
        if (new_val != cur_val) {
            tick_value_change_obj = objects.pv_power;
            lv_arc_set_value(objects.pv_power, new_val);
            tick_value_change_obj = NULL;
        }
    }
    {
        const char *new_val = evalTextProperty(flowState, 39, 3, "Failed to evaluate Text in Label widget");
        const char *cur_val = lv_label_get_text(objects.obj10);
        if (strcmp(new_val, cur_val) != 0) {
            tick_value_change_obj = objects.obj10;
            lv_label_set_text(objects.obj10, new_val);
            tick_value_change_obj = NULL;
        }
    }
    {
        int32_t new_val = evalIntegerProperty(flowState, 41, 3, "Failed to evaluate Value in Arc widget");
        int32_t cur_val = lv_arc_get_value(objects.yield);
        if (new_val != cur_val) {
            tick_value_change_obj = objects.yield;
            lv_arc_set_value(objects.yield, new_val);
            tick_value_change_obj = NULL;
        }
    }
    {
        const char *new_val = evalTextProperty(flowState, 45, 3, "Failed to evaluate Text in Label widget");
        const char *cur_val = lv_label_get_text(objects.obj11);
        if (strcmp(new_val, cur_val) != 0) {
            tick_value_change_obj = objects.obj11;
            lv_label_set_text(objects.obj11, new_val);
            tick_value_change_obj = NULL;
        }
    }
    {
        const char *new_val = evalTextProperty(flowState, 47, 3, "Failed to evaluate Text in Label widget");
        const char *cur_val = lv_label_get_text(objects.solar_error);
        if (strcmp(new_val, cur_val) != 0) {
            tick_value_change_obj = objects.solar_error;
            lv_label_set_text(objects.solar_error, new_val);
            tick_value_change_obj = NULL;
        }
    }
}

void create_screen_config() {
    void *flowState = getFlowState(0, 1);
    (void)flowState;
    lv_obj_t *obj = lv_obj_create(0);
    objects.config = obj;
    lv_obj_set_pos(obj, 0, 0);
    lv_obj_set_size(obj, 800, 480);
    {
        lv_obj_t *parent_obj = obj;
        {
            // go_main
            lv_obj_t *obj = lv_img_create(parent_obj);
            objects.go_main = obj;
            lv_obj_set_pos(obj, 0, 0);
            lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
            lv_img_set_src(obj, &img_solar);
            lv_obj_add_event_cb(obj, event_handler_cb_config_go_main, LV_EVENT_ALL, flowState);
            lv_obj_add_flag(obj, LV_OBJ_FLAG_CLICKABLE);
            lv_obj_set_style_img_recolor(obj, lv_color_hex(0xff000000), LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_img_recolor_opa(obj, 255, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_top(obj, 5, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_bottom(obj, 5, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_left(obj, 5, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_right(obj, 5, LV_PART_MAIN | LV_STATE_DEFAULT);
        }
        {
            lv_obj_t *obj = lv_obj_create(parent_obj);
            lv_obj_set_pos(obj, 518, 59);
            lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
            lv_obj_set_style_pad_left(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_top(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_right(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_bottom(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_bg_opa(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_border_width(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_radius(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            {
                lv_obj_t *parent_obj = obj;
                {
                    // wifi_btn
                    lv_obj_t *obj = lv_btn_create(parent_obj);
                    objects.wifi_btn = obj;
                    lv_obj_set_pos(obj, 18, 0);
                    lv_obj_set_size(obj, 140, 50);
                    lv_obj_add_event_cb(obj, event_handler_cb_config_wifi_btn, LV_EVENT_ALL, flowState);
                    lv_obj_set_style_bg_color(obj, lv_color_hex(theme_colors[eez_flow_get_selected_theme_index()][1]), LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_obj_set_style_text_font(obj, &ui_font_roboto_reg_14, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_obj_set_style_bg_color(obj, lv_color_hex(0xffff0000), LV_PART_MAIN | LV_STATE_DISABLED);
                    lv_obj_set_style_text_color(obj, lv_color_hex(0xff000000), LV_PART_MAIN | LV_STATE_DISABLED);
                    {
                        lv_obj_t *parent_obj = obj;
                        {
                            lv_obj_t *obj = lv_label_create(parent_obj);
                            objects.obj13 = obj;
                            lv_obj_set_pos(obj, 0, 0);
                            lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                            lv_obj_set_style_align(obj, LV_ALIGN_CENTER, LV_PART_MAIN | LV_STATE_DEFAULT);
                            lv_label_set_text(obj, "");
                        }
                    }
                }
                {
                    lv_obj_t *obj = lv_label_create(parent_obj);
                    objects.obj14 = obj;
                    lv_obj_set_pos(obj, 0, 65);
                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                    lv_obj_set_style_text_font(obj, &ui_font_roboto_reg_14, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_label_set_text(obj, "");
                }
            }
        }
        {
            lv_obj_t *obj = lv_obj_create(parent_obj);
            lv_obj_set_pos(obj, 520, 199);
            lv_obj_set_size(obj, 170, 130);
            lv_obj_set_style_pad_left(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_top(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_right(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_bottom(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_bg_opa(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_border_width(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_radius(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            {
                lv_obj_t *parent_obj = obj;
                {
                    lv_obj_t *obj = lv_label_create(parent_obj);
                    lv_obj_set_pos(obj, 0, 0);
                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                    lv_obj_set_style_align(obj, LV_ALIGN_TOP_MID, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_obj_set_style_text_font(obj, &ui_font_roboto_reg_14, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_label_set_text(obj, "BL Delay");
                }
                {
                    lv_obj_t *obj = lv_label_create(parent_obj);
                    objects.obj15 = obj;
                    lv_obj_set_pos(obj, 0, 27);
                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                    lv_obj_set_style_align(obj, LV_ALIGN_TOP_MID, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_obj_set_style_text_font(obj, &ui_font_roboto_med_48, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_label_set_text(obj, "");
                }
                {
                    lv_obj_t *obj = lv_label_create(parent_obj);
                    lv_obj_set_pos(obj, 0, 70);
                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                    lv_obj_set_style_align(obj, LV_ALIGN_TOP_MID, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_obj_set_style_text_font(obj, &ui_font_roboto_reg_14, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_label_set_text(obj, "seconds");
                }
                {
                    lv_obj_t *obj = lv_slider_create(parent_obj);
                    objects.obj12 = obj;
                    lv_obj_set_pos(obj, 15, 100);
                    lv_obj_set_size(obj, 140, 15);
                    lv_slider_set_range(obj, 5, 100);
                    lv_obj_add_event_cb(obj, event_handler_cb_config_obj12, LV_EVENT_ALL, flowState);
                    lv_obj_set_style_bg_color(obj, lv_color_hex(theme_colors[eez_flow_get_selected_theme_index()][1]), LV_PART_INDICATOR | LV_STATE_DEFAULT);
                    lv_obj_set_style_bg_color(obj, lv_color_hex(theme_colors[eez_flow_get_selected_theme_index()][1]), LV_PART_KNOB | LV_STATE_DEFAULT);
                }
            }
        }
        {
            lv_obj_t *obj = lv_obj_create(parent_obj);
            lv_obj_set_pos(obj, 17, 39);
            lv_obj_set_size(obj, 410, LV_SIZE_CONTENT);
            lv_obj_set_style_border_width(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            add_style_device_config(obj);
            lv_obj_set_style_radius(obj, 10, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_bg_opa(obj, 255, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_top(obj, 5, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_bottom(obj, 5, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_left(obj, 5, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_right(obj, 5, LV_PART_MAIN | LV_STATE_DEFAULT);
            {
                lv_obj_t *parent_obj = obj;
                {
                    lv_obj_t *obj = lv_label_create(parent_obj);
                    lv_obj_set_pos(obj, 0, 3);
                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                    lv_obj_set_style_text_font(obj, &ui_font_roboto_reg_32, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_obj_set_style_align(obj, LV_ALIGN_TOP_MID, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_label_set_text(obj, "Inverter");
                }
                {
                    lv_obj_t *obj = lv_label_create(parent_obj);
                    objects.obj16 = obj;
                    lv_obj_set_pos(obj, 0, 42);
                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                    lv_obj_set_style_text_font(obj, &ui_font_roboto_med_18, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_obj_set_style_align(obj, LV_ALIGN_TOP_MID, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_label_set_text(obj, "");
                }
                {
                    lv_obj_t *obj = lv_label_create(parent_obj);
                    lv_obj_set_pos(obj, 5, 60);
                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                    lv_obj_set_style_text_font(obj, &ui_font_roboto_reg_14, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_label_set_text(obj, "Key:");
                }
                {
                    lv_obj_t *obj = lv_label_create(parent_obj);
                    objects.obj17 = obj;
                    lv_obj_set_pos(obj, 5, 80);
                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                    lv_obj_set_style_text_font(obj, &ui_font_roboto_med_18, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_label_set_text(obj, "");
                }
                {
                    lv_obj_t *obj = lv_label_create(parent_obj);
                    lv_obj_set_pos(obj, 5, 105);
                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                    lv_obj_set_style_text_font(obj, &ui_font_roboto_reg_14, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_label_set_text(obj, "Pin:");
                }
                {
                    lv_obj_t *obj = lv_label_create(parent_obj);
                    objects.obj18 = obj;
                    lv_obj_set_pos(obj, 31, 105);
                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                    lv_obj_set_style_text_font(obj, &ui_font_roboto_med_18, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_label_set_text(obj, "");
                }
                {
                    lv_obj_t *obj = lv_label_create(parent_obj);
                    lv_obj_set_pos(obj, 106, 105);
                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                    lv_obj_set_style_text_font(obj, &ui_font_roboto_reg_14, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_label_set_text(obj, "Size:");
                }
                {
                    lv_obj_t *obj = lv_label_create(parent_obj);
                    lv_obj_set_pos(obj, 142, 105);
                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                    lv_obj_set_style_text_font(obj, &ui_font_roboto_med_18, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_label_set_text(obj, "2000");
                }
            }
        }
        {
            lv_obj_t *obj = lv_obj_create(parent_obj);
            lv_obj_set_pos(obj, 17, 191);
            lv_obj_set_size(obj, 410, LV_SIZE_CONTENT);
            lv_obj_set_style_border_width(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_clear_flag(obj, LV_OBJ_FLAG_SCROLLABLE);
            add_style_device_config(obj);
            lv_obj_set_style_bg_opa(obj, 255, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_radius(obj, 10, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_top(obj, 5, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_bottom(obj, 5, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_left(obj, 5, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_right(obj, 5, LV_PART_MAIN | LV_STATE_DEFAULT);
            {
                lv_obj_t *parent_obj = obj;
                {
                    lv_obj_t *obj = lv_label_create(parent_obj);
                    lv_obj_set_pos(obj, 0, 3);
                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                    lv_obj_set_style_text_font(obj, &ui_font_roboto_reg_32, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_obj_set_style_align(obj, LV_ALIGN_TOP_MID, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_label_set_text(obj, "MPPT");
                }
                {
                    lv_obj_t *obj = lv_label_create(parent_obj);
                    objects.obj19 = obj;
                    lv_obj_set_pos(obj, 0, 42);
                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                    lv_obj_set_style_text_font(obj, &ui_font_roboto_med_18, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_obj_set_style_align(obj, LV_ALIGN_TOP_MID, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_label_set_text(obj, "");
                }
                {
                    lv_obj_t *obj = lv_label_create(parent_obj);
                    lv_obj_set_pos(obj, 5, 60);
                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                    lv_obj_set_style_text_font(obj, &ui_font_roboto_reg_14, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_label_set_text(obj, "Key:");
                }
                {
                    lv_obj_t *obj = lv_label_create(parent_obj);
                    objects.obj20 = obj;
                    lv_obj_set_pos(obj, 5, 80);
                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                    lv_obj_set_style_text_font(obj, &ui_font_roboto_med_18, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_label_set_text(obj, "");
                }
                {
                    lv_obj_t *obj = lv_label_create(parent_obj);
                    lv_obj_set_pos(obj, 5, 105);
                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                    lv_obj_set_style_text_font(obj, &ui_font_roboto_reg_14, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_label_set_text(obj, "Array Size:");
                }
                {
                    lv_obj_t *obj = lv_label_create(parent_obj);
                    lv_obj_set_pos(obj, 76, 105);
                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                    lv_obj_set_style_text_font(obj, &ui_font_roboto_med_18, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_label_set_text(obj, "400");
                }
            }
        }
        {
            lv_obj_t *obj = lv_obj_create(parent_obj);
            lv_obj_set_pos(obj, 17, 343);
            lv_obj_set_size(obj, 410, LV_SIZE_CONTENT);
            lv_obj_set_style_border_width(obj, 0, LV_PART_MAIN | LV_STATE_DEFAULT);
            add_style_device_config(obj);
            lv_obj_set_style_radius(obj, 10, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_bottom(obj, 10, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_left(obj, 5, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_right(obj, 5, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_pad_top(obj, 5, LV_PART_MAIN | LV_STATE_DEFAULT);
            lv_obj_set_style_bg_opa(obj, 255, LV_PART_MAIN | LV_STATE_DEFAULT);
            {
                lv_obj_t *parent_obj = obj;
                {
                    lv_obj_t *obj = lv_label_create(parent_obj);
                    lv_obj_set_pos(obj, 0, 3);
                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                    lv_obj_set_style_text_font(obj, &ui_font_roboto_reg_32, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_obj_set_style_align(obj, LV_ALIGN_TOP_MID, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_label_set_text(obj, "BMV");
                }
                {
                    lv_obj_t *obj = lv_label_create(parent_obj);
                    objects.obj21 = obj;
                    lv_obj_set_pos(obj, 0, 42);
                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                    lv_obj_set_style_text_font(obj, &ui_font_roboto_med_18, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_obj_set_style_align(obj, LV_ALIGN_TOP_MID, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_label_set_text(obj, "");
                }
                {
                    lv_obj_t *obj = lv_label_create(parent_obj);
                    lv_obj_set_pos(obj, 5, 60);
                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                    lv_obj_set_style_text_font(obj, &ui_font_roboto_reg_14, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_label_set_text(obj, "Key:");
                }
                {
                    lv_obj_t *obj = lv_label_create(parent_obj);
                    objects.obj22 = obj;
                    lv_obj_set_pos(obj, 5, 80);
                    lv_obj_set_size(obj, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
                    lv_obj_set_style_text_font(obj, &ui_font_roboto_med_18, LV_PART_MAIN | LV_STATE_DEFAULT);
                    lv_label_set_text(obj, "");
                }
            }
        }
    }
    
    tick_screen_config();
}

void delete_screen_config() {
    lv_obj_del(objects.config);
    objects.config = 0;
    objects.go_main = 0;
    objects.wifi_btn = 0;
    objects.obj13 = 0;
    objects.obj14 = 0;
    objects.obj15 = 0;
    objects.obj12 = 0;
    objects.obj16 = 0;
    objects.obj17 = 0;
    objects.obj18 = 0;
    objects.obj19 = 0;
    objects.obj20 = 0;
    objects.obj21 = 0;
    objects.obj22 = 0;
    deletePageFlowState(1);
}

void tick_screen_config() {
    void *flowState = getFlowState(0, 1);
    (void)flowState;
    {
        bool new_val = evalBooleanProperty(flowState, 3, 3, "Failed to evaluate Disabled state");
        bool cur_val = lv_obj_has_state(objects.wifi_btn, LV_STATE_DISABLED);
        if (new_val != cur_val) {
            tick_value_change_obj = objects.wifi_btn;
            if (new_val) lv_obj_add_state(objects.wifi_btn, LV_STATE_DISABLED);
            else lv_obj_clear_state(objects.wifi_btn, LV_STATE_DISABLED);
            tick_value_change_obj = NULL;
        }
    }
    {
        const char *new_val = evalTextProperty(flowState, 4, 3, "Failed to evaluate Text in Label widget");
        const char *cur_val = lv_label_get_text(objects.obj13);
        if (strcmp(new_val, cur_val) != 0) {
            tick_value_change_obj = objects.obj13;
            lv_label_set_text(objects.obj13, new_val);
            tick_value_change_obj = NULL;
        }
    }
    {
        const char *new_val = evalTextProperty(flowState, 5, 3, "Failed to evaluate Text in Label widget");
        const char *cur_val = lv_label_get_text(objects.obj14);
        if (strcmp(new_val, cur_val) != 0) {
            tick_value_change_obj = objects.obj14;
            lv_label_set_text(objects.obj14, new_val);
            tick_value_change_obj = NULL;
        }
    }
    {
        const char *new_val = evalTextProperty(flowState, 8, 3, "Failed to evaluate Text in Label widget");
        const char *cur_val = lv_label_get_text(objects.obj15);
        if (strcmp(new_val, cur_val) != 0) {
            tick_value_change_obj = objects.obj15;
            lv_label_set_text(objects.obj15, new_val);
            tick_value_change_obj = NULL;
        }
    }
    {
        int32_t new_val = evalIntegerProperty(flowState, 10, 3, "Failed to evaluate Value in Slider widget");
        int32_t cur_val = lv_slider_get_value(objects.obj12);
        if (new_val != cur_val) {
            tick_value_change_obj = objects.obj12;
            lv_slider_set_value(objects.obj12, new_val, LV_ANIM_ON);
            tick_value_change_obj = NULL;
        }
    }
    {
        const char *new_val = evalTextProperty(flowState, 13, 3, "Failed to evaluate Text in Label widget");
        const char *cur_val = lv_label_get_text(objects.obj16);
        if (strcmp(new_val, cur_val) != 0) {
            tick_value_change_obj = objects.obj16;
            lv_label_set_text(objects.obj16, new_val);
            tick_value_change_obj = NULL;
        }
    }
    {
        const char *new_val = evalTextProperty(flowState, 15, 3, "Failed to evaluate Text in Label widget");
        const char *cur_val = lv_label_get_text(objects.obj17);
        if (strcmp(new_val, cur_val) != 0) {
            tick_value_change_obj = objects.obj17;
            lv_label_set_text(objects.obj17, new_val);
            tick_value_change_obj = NULL;
        }
    }
    {
        const char *new_val = evalTextProperty(flowState, 17, 3, "Failed to evaluate Text in Label widget");
        const char *cur_val = lv_label_get_text(objects.obj18);
        if (strcmp(new_val, cur_val) != 0) {
            tick_value_change_obj = objects.obj18;
            lv_label_set_text(objects.obj18, new_val);
            tick_value_change_obj = NULL;
        }
    }
    {
        const char *new_val = evalTextProperty(flowState, 22, 3, "Failed to evaluate Text in Label widget");
        const char *cur_val = lv_label_get_text(objects.obj19);
        if (strcmp(new_val, cur_val) != 0) {
            tick_value_change_obj = objects.obj19;
            lv_label_set_text(objects.obj19, new_val);
            tick_value_change_obj = NULL;
        }
    }
    {
        const char *new_val = evalTextProperty(flowState, 24, 3, "Failed to evaluate Text in Label widget");
        const char *cur_val = lv_label_get_text(objects.obj20);
        if (strcmp(new_val, cur_val) != 0) {
            tick_value_change_obj = objects.obj20;
            lv_label_set_text(objects.obj20, new_val);
            tick_value_change_obj = NULL;
        }
    }
    {
        const char *new_val = evalTextProperty(flowState, 29, 3, "Failed to evaluate Text in Label widget");
        const char *cur_val = lv_label_get_text(objects.obj21);
        if (strcmp(new_val, cur_val) != 0) {
            tick_value_change_obj = objects.obj21;
            lv_label_set_text(objects.obj21, new_val);
            tick_value_change_obj = NULL;
        }
    }
    {
        const char *new_val = evalTextProperty(flowState, 31, 3, "Failed to evaluate Text in Label widget");
        const char *cur_val = lv_label_get_text(objects.obj22);
        if (strcmp(new_val, cur_val) != 0) {
            tick_value_change_obj = objects.obj22;
            lv_label_set_text(objects.obj22, new_val);
            tick_value_change_obj = NULL;
        }
    }
}

void change_color_theme(uint32_t theme_index) {
    lv_style_set_bg_color(get_style_device_config_MAIN_DEFAULT(), lv_color_hex(theme_colors[theme_index][1]));
    
    lv_style_set_border_color(get_style_device_config_MAIN_DEFAULT(), lv_color_hex(theme_colors[theme_index][1]));
    
    if (objects.wifi_btn) lv_obj_set_style_bg_color(objects.wifi_btn, lv_color_hex(theme_colors[theme_index][1]), LV_PART_MAIN | LV_STATE_DEFAULT);
    
    if (objects.obj12) lv_obj_set_style_bg_color(objects.obj12, lv_color_hex(theme_colors[theme_index][1]), LV_PART_INDICATOR | LV_STATE_DEFAULT);
    
    if (objects.obj12) lv_obj_set_style_bg_color(objects.obj12, lv_color_hex(theme_colors[theme_index][1]), LV_PART_KNOB | LV_STATE_DEFAULT);
    
    if (objects.main) lv_obj_invalidate(objects.main);
    if (objects.config) lv_obj_invalidate(objects.config);
}

extern void add_style(lv_obj_t *obj, int32_t styleIndex);
extern void remove_style(lv_obj_t *obj, int32_t styleIndex);

static const char *screen_names[] = { "Main", "Config" };
static const char *object_names[] = { "main", "config", "obj0", "go_config", "go_main", "wifi_btn", "inverter_container", "ac_watts_arc", "obj1", "inv_error", "solar_container", "soc", "batt_indicator_image", "batt_alarm", "soc_unknown_container", "soc_container", "obj2", "pv_power", "image_solar", "yield", "image_sun", "solar_error", "obj3", "obj4", "obj5", "obj6", "obj7", "obj8", "obj9", "obj10", "obj11", "obj12", "obj13", "obj14", "obj15", "obj16", "obj17", "obj18", "obj19", "obj20", "obj21", "obj22" };
static const char *style_names[] = { "Labels", "Arcs", "Images", "Labels_Error", "Device_Config" };
static const char *theme_names[] = { "Default" };

uint32_t theme_colors[1][2] = {
    { 0xff000000, 0xff4789d0 },
};


typedef void (*create_screen_func_t)();
create_screen_func_t create_screen_funcs[] = {
    create_screen_main,
    create_screen_config,
};
void create_screen(int screen_index) {
    create_screen_funcs[screen_index]();
}
void create_screen_by_id(enum ScreensEnum screenId) {
    create_screen_funcs[screenId - 1]();
}

typedef void (*delete_screen_func_t)();
delete_screen_func_t delete_screen_funcs[] = {
    delete_screen_main,
    delete_screen_config,
};
void delete_screen(int screen_index) {
    delete_screen_funcs[screen_index]();
}
void delete_screen_by_id(enum ScreensEnum screenId) {
    delete_screen_funcs[screenId - 1]();
}

typedef void (*tick_screen_func_t)();
tick_screen_func_t tick_screen_funcs[] = {
    tick_screen_main,
    tick_screen_config,
};
void tick_screen(int screen_index) {
    tick_screen_funcs[screen_index]();
}
void tick_screen_by_id(enum ScreensEnum screenId) {
    tick_screen_funcs[screenId - 1]();
}

void create_screens() {
    eez_flow_init_styles(add_style, remove_style);
    
    eez_flow_init_screen_names(screen_names, sizeof(screen_names) / sizeof(const char *));
    eez_flow_init_object_names(object_names, sizeof(object_names) / sizeof(const char *));
    eez_flow_init_style_names(style_names, sizeof(style_names) / sizeof(const char *));
    eez_flow_init_themes(theme_names, sizeof(theme_names) / sizeof(const char *), change_color_theme);
    
    eez_flow_set_create_screen_func(create_screen);
    eez_flow_set_delete_screen_func(delete_screen);
    
    lv_disp_t *dispp = lv_disp_get_default();
    lv_theme_t *theme = lv_theme_default_init(dispp, lv_palette_main(LV_PALETTE_BLUE), lv_palette_main(LV_PALETTE_RED), false, LV_FONT_DEFAULT);
    lv_disp_set_theme(dispp, theme);
    
    create_screen_main();
    create_screen_config();
}
