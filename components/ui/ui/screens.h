#ifndef EEZ_LVGL_UI_SCREENS_H
#define EEZ_LVGL_UI_SCREENS_H

#include <lvgl.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct _objects_t {
    lv_obj_t *main;
    lv_obj_t *history;
    lv_obj_t *config;
    lv_obj_t *obj0;
    lv_obj_t *go_config;
    lv_obj_t *go_history;
    lv_obj_t *go_main_hist;
    lv_obj_t *obj1;
    lv_obj_t *obj2;
    lv_obj_t *obj3;
    lv_obj_t *obj4;
    lv_obj_t *go_main;
    lv_obj_t *wifi_btn;
    lv_obj_t *inverter_container;
    lv_obj_t *ac_watts_arc;
    lv_obj_t *obj5;
    lv_obj_t *inv_error;
    lv_obj_t *solar_container;
    lv_obj_t *soc;
    lv_obj_t *batt_indicator_image;
    lv_obj_t *batt_alarm;
    lv_obj_t *soc_unknown_container;
    lv_obj_t *soc_container;
    lv_obj_t *obj6;
    lv_obj_t *pv_power;
    lv_obj_t *image_solar;
    lv_obj_t *yield;
    lv_obj_t *image_sun;
    lv_obj_t *solar_error;
    lv_obj_t *obj7;
    lv_obj_t *obj8;
    lv_obj_t *obj9;
    lv_obj_t *obj10;
    lv_obj_t *obj11;
    lv_obj_t *obj12;
    lv_obj_t *obj13;
    lv_obj_t *obj14;
    lv_obj_t *obj15;
    lv_obj_t *hist_det_lifetime;
    lv_obj_t *chart_history;
    lv_obj_t *chart_pmax;
    lv_obj_t *history_details;
    lv_obj_t *obj16;
    lv_obj_t *hist_det_day;
    lv_obj_t *obj17;
    lv_obj_t *hist_det_yield;
    lv_obj_t *obj18;
    lv_obj_t *hist_det_pmax;
    lv_obj_t *obj19;
    lv_obj_t *hist_det_vmax;
    lv_obj_t *obj20;
    lv_obj_t *hist_det_float;
    lv_obj_t *obj21;
    lv_obj_t *hist_det_abs;
    lv_obj_t *obj22;
    lv_obj_t *hist_det_bulk;
    lv_obj_t *obj23;
    lv_obj_t *hist_det_bat_max;
    lv_obj_t *hist_det_bat_min;
    lv_obj_t *obj24;
    lv_obj_t *hist_det_errors;
    lv_obj_t *obj25;
    lv_obj_t *history_loading;
    lv_obj_t *obj26;
    lv_obj_t *obj27;
    lv_obj_t *obj28;
    lv_obj_t *obj29;
    lv_obj_t *obj30;
    lv_obj_t *obj31;
    lv_obj_t *obj32;
    lv_obj_t *obj33;
    lv_obj_t *obj34;
    lv_obj_t *obj35;
    lv_obj_t *obj36;
    lv_obj_t *obj37;
    lv_obj_t *obj38;
    lv_obj_t *obj39;
    lv_obj_t *obj40;
} objects_t;

extern objects_t objects;

enum ScreensEnum {
    SCREEN_ID_MAIN = 1,
    SCREEN_ID_HISTORY = 2,
    SCREEN_ID_CONFIG = 3,
};

void create_screen_main();
void delete_screen_main();
void tick_screen_main();

void create_screen_history();
void delete_screen_history();
void tick_screen_history();

void create_screen_config();
void delete_screen_config();
void tick_screen_config();

enum Themes {
    THEME_ID_DEFAULT,
};
enum Colors {
    COLOR_ID_BACKGROUND,
    COLOR_ID_VICTRON,
};
void change_color_theme(uint32_t themeIndex);
extern uint32_t theme_colors[1][2];

void create_screen_by_id(enum ScreensEnum screenId);
void delete_screen_by_id(enum ScreensEnum screenId);
void tick_screen_by_id(enum ScreensEnum screenId);
void tick_screen(int screen_index);

void create_screens();


#ifdef __cplusplus
}
#endif

#endif /*EEZ_LVGL_UI_SCREENS_H*/