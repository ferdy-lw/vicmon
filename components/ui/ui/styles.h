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



#ifdef __cplusplus
}
#endif

#endif /*EEZ_LVGL_UI_STYLES_H*/