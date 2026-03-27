#ifndef EEZ_LVGL_UI_STYLES_H
#define EEZ_LVGL_UI_STYLES_H

#include <lvgl.h>

#ifdef __cplusplus
extern "C" {
#endif

// Style: Labels
lv_style_t *get_style_labels_MAIN_DEFAULT();
void add_style_labels(lv_obj_t *obj);
void remove_style_labels(lv_obj_t *obj);

// Style: Arcs
lv_style_t *get_style_arcs_MAIN_DEFAULT();
lv_style_t *get_style_arcs_KNOB_DEFAULT();
lv_style_t *get_style_arcs_INDICATOR_DEFAULT();
void add_style_arcs(lv_obj_t *obj);
void remove_style_arcs(lv_obj_t *obj);

// Style: Images
lv_style_t *get_style_images_MAIN_DEFAULT();
void add_style_images(lv_obj_t *obj);
void remove_style_images(lv_obj_t *obj);

// Style: Labels_Error
lv_style_t *get_style_labels_error_MAIN_DEFAULT();
void add_style_labels_error(lv_obj_t *obj);
void remove_style_labels_error(lv_obj_t *obj);

// Style: Device_Config
lv_style_t *get_style_device_config_MAIN_DEFAULT();
void add_style_device_config(lv_obj_t *obj);
void remove_style_device_config(lv_obj_t *obj);

// Style: History_Details
lv_style_t *get_style_history_details_MAIN_DEFAULT();
void add_style_history_details(lv_obj_t *obj);
void remove_style_history_details(lv_obj_t *obj);

// Style: History_Details_Cont
lv_style_t *get_style_history_details_cont_MAIN_DEFAULT();
void add_style_history_details_cont(lv_obj_t *obj);
void remove_style_history_details_cont(lv_obj_t *obj);

#ifdef __cplusplus
}
#endif

#endif /*EEZ_LVGL_UI_STYLES_H*/