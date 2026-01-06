#include "styles.h"
#include "images.h"
#include "fonts.h"

#include "ui.h"
#include "screens.h"

//
// Style: Labels
//

void init_style_labels_MAIN_DEFAULT(lv_style_t *style) {
    lv_style_set_text_color(style, lv_color_hex(0xfff2f2f2));
};

lv_style_t *get_style_labels_MAIN_DEFAULT() {
    static lv_style_t *style;
    if (!style) {
        style = lv_mem_alloc(sizeof(lv_style_t));
        lv_style_init(style);
        init_style_labels_MAIN_DEFAULT(style);
    }
    return style;
};

void add_style_labels(lv_obj_t *obj) {
    (void)obj;
    lv_obj_add_style(obj, get_style_labels_MAIN_DEFAULT(), LV_PART_MAIN | LV_STATE_DEFAULT);
};

void remove_style_labels(lv_obj_t *obj) {
    (void)obj;
    lv_obj_remove_style(obj, get_style_labels_MAIN_DEFAULT(), LV_PART_MAIN | LV_STATE_DEFAULT);
};

//
// Style: Arcs
//

void init_style_arcs_MAIN_DEFAULT(lv_style_t *style) {
    lv_style_set_arc_width(style, 12);
    lv_style_set_arc_color(style, lv_color_hex(0xff222629));
};

lv_style_t *get_style_arcs_MAIN_DEFAULT() {
    static lv_style_t *style;
    if (!style) {
        style = lv_mem_alloc(sizeof(lv_style_t));
        lv_style_init(style);
        init_style_arcs_MAIN_DEFAULT(style);
    }
    return style;
};

void init_style_arcs_KNOB_DEFAULT(lv_style_t *style) {
    lv_style_set_opa(style, 0);
};

lv_style_t *get_style_arcs_KNOB_DEFAULT() {
    static lv_style_t *style;
    if (!style) {
        style = lv_mem_alloc(sizeof(lv_style_t));
        lv_style_init(style);
        init_style_arcs_KNOB_DEFAULT(style);
    }
    return style;
};

void init_style_arcs_INDICATOR_DEFAULT(lv_style_t *style) {
    lv_style_set_arc_width(style, 12);
    lv_style_set_arc_color(style, lv_color_hex(0xff4789d0));
};

lv_style_t *get_style_arcs_INDICATOR_DEFAULT() {
    static lv_style_t *style;
    if (!style) {
        style = lv_mem_alloc(sizeof(lv_style_t));
        lv_style_init(style);
        init_style_arcs_INDICATOR_DEFAULT(style);
    }
    return style;
};

void add_style_arcs(lv_obj_t *obj) {
    (void)obj;
    lv_obj_add_style(obj, get_style_arcs_MAIN_DEFAULT(), LV_PART_MAIN | LV_STATE_DEFAULT);
    lv_obj_add_style(obj, get_style_arcs_KNOB_DEFAULT(), LV_PART_KNOB | LV_STATE_DEFAULT);
    lv_obj_add_style(obj, get_style_arcs_INDICATOR_DEFAULT(), LV_PART_INDICATOR | LV_STATE_DEFAULT);
};

void remove_style_arcs(lv_obj_t *obj) {
    (void)obj;
    lv_obj_remove_style(obj, get_style_arcs_MAIN_DEFAULT(), LV_PART_MAIN | LV_STATE_DEFAULT);
    lv_obj_remove_style(obj, get_style_arcs_KNOB_DEFAULT(), LV_PART_KNOB | LV_STATE_DEFAULT);
    lv_obj_remove_style(obj, get_style_arcs_INDICATOR_DEFAULT(), LV_PART_INDICATOR | LV_STATE_DEFAULT);
};

//
// Style: Images
//

void init_style_images_MAIN_DEFAULT(lv_style_t *style) {
    lv_style_set_img_recolor(style, lv_color_hex(0xfff2f2f2));
    lv_style_set_img_recolor_opa(style, 255);
};

lv_style_t *get_style_images_MAIN_DEFAULT() {
    static lv_style_t *style;
    if (!style) {
        style = lv_mem_alloc(sizeof(lv_style_t));
        lv_style_init(style);
        init_style_images_MAIN_DEFAULT(style);
    }
    return style;
};

void add_style_images(lv_obj_t *obj) {
    (void)obj;
    lv_obj_add_style(obj, get_style_images_MAIN_DEFAULT(), LV_PART_MAIN | LV_STATE_DEFAULT);
};

void remove_style_images(lv_obj_t *obj) {
    (void)obj;
    lv_obj_remove_style(obj, get_style_images_MAIN_DEFAULT(), LV_PART_MAIN | LV_STATE_DEFAULT);
};

//
// Style: Labels_Error
//

void init_style_labels_error_MAIN_DEFAULT(lv_style_t *style) {
    lv_style_set_text_font(style, &ui_font_roboto_reg_32);
    lv_style_set_text_color(style, lv_color_hex(0xffe72e2e));
};

lv_style_t *get_style_labels_error_MAIN_DEFAULT() {
    static lv_style_t *style;
    if (!style) {
        style = lv_mem_alloc(sizeof(lv_style_t));
        lv_style_init(style);
        init_style_labels_error_MAIN_DEFAULT(style);
    }
    return style;
};

void add_style_labels_error(lv_obj_t *obj) {
    (void)obj;
    lv_obj_add_style(obj, get_style_labels_error_MAIN_DEFAULT(), LV_PART_MAIN | LV_STATE_DEFAULT);
};

void remove_style_labels_error(lv_obj_t *obj) {
    (void)obj;
    lv_obj_remove_style(obj, get_style_labels_error_MAIN_DEFAULT(), LV_PART_MAIN | LV_STATE_DEFAULT);
};

//
// Style: Device_Config
//

void init_style_device_config_MAIN_DEFAULT(lv_style_t *style) {
    lv_style_set_pad_top(style, 5);
    lv_style_set_pad_bottom(style, 5);
    lv_style_set_pad_left(style, 5);
    lv_style_set_pad_right(style, 5);
    lv_style_set_bg_color(style, lv_color_hex(theme_colors[eez_flow_get_selected_theme_index()][1]));
    lv_style_set_border_color(style, lv_color_hex(theme_colors[eez_flow_get_selected_theme_index()][1]));
    lv_style_set_border_width(style, 1);
    lv_style_set_shadow_width(style, 5);
    lv_style_set_shadow_spread(style, 2);
    lv_style_set_shadow_color(style, lv_color_hex(0xffd0cdcd));
    lv_style_set_text_color(style, lv_color_hex(0xfff2f2f2));
    lv_style_set_radius(style, 10);
    lv_style_set_bg_opa(style, 255);
};

lv_style_t *get_style_device_config_MAIN_DEFAULT() {
    static lv_style_t *style;
    if (!style) {
        style = lv_mem_alloc(sizeof(lv_style_t));
        lv_style_init(style);
        init_style_device_config_MAIN_DEFAULT(style);
    }
    return style;
};

void add_style_device_config(lv_obj_t *obj) {
    (void)obj;
    lv_obj_add_style(obj, get_style_device_config_MAIN_DEFAULT(), LV_PART_MAIN | LV_STATE_DEFAULT);
};

void remove_style_device_config(lv_obj_t *obj) {
    (void)obj;
    lv_obj_remove_style(obj, get_style_device_config_MAIN_DEFAULT(), LV_PART_MAIN | LV_STATE_DEFAULT);
};

//
//
//

void add_style(lv_obj_t *obj, int32_t styleIndex) {
    typedef void (*AddStyleFunc)(lv_obj_t *obj);
    static const AddStyleFunc add_style_funcs[] = {
        add_style_labels,
        add_style_arcs,
        add_style_images,
        add_style_labels_error,
        add_style_device_config,
    };
    add_style_funcs[styleIndex](obj);
}

void remove_style(lv_obj_t *obj, int32_t styleIndex) {
    typedef void (*RemoveStyleFunc)(lv_obj_t *obj);
    static const RemoveStyleFunc remove_style_funcs[] = {
        remove_style_labels,
        remove_style_arcs,
        remove_style_images,
        remove_style_labels_error,
        remove_style_device_config,
    };
    remove_style_funcs[styleIndex](obj);
}

