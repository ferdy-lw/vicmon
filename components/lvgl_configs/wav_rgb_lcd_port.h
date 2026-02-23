// #include "esp_lvgl_port.h"
typedef int esp_err_t;
struct _lv_display_t;
typedef struct _lv_display_t lv_display_t;
struct _lv_indev_t;
typedef struct _lv_indev_t lv_indev_t;

/* LVGL display and touch */
extern lv_display_t *lvgl_disp;
extern lv_indev_t *lvgl_touch_indev;

bool lvgl_port_lock(uint32_t timeout_ms);
void lvgl_port_unlock(void);

esp_err_t waveshare_rgb_lcd_bl_on();
esp_err_t waveshare_rgb_lcd_bl_off();

esp_err_t app_lcd_init(void);
esp_err_t app_touch_init(void);
esp_err_t app_lvgl_init(void);
