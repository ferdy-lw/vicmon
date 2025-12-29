#ifndef EEZ_LVGL_UI_VARS_H
#define EEZ_LVGL_UI_VARS_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// enum declarations



// Flow global variables

enum FlowGlobalVariables {
    FLOW_GLOBAL_VARIABLE_NONE
};

// Native global variables

extern bool get_var_inv_switch();
extern void set_var_inv_switch(bool value);
extern const char *get_var_inv_mode();
extern void set_var_inv_mode(const char *value);
extern const char *get_var_inv_error();
extern void set_var_inv_error(const char *value);
extern int32_t get_var_ac_watts();
extern void set_var_ac_watts(int32_t value);
extern float get_var_batt_soc();
extern void set_var_batt_soc(float value);
extern float get_var_batt_volt();
extern void set_var_batt_volt(float value);
extern float get_var_batt_amp();
extern void set_var_batt_amp(float value);
extern int32_t get_var_batt_temp();
extern void set_var_batt_temp(int32_t value);
extern const char *get_var_batt_alarm();
extern void set_var_batt_alarm(const char *value);
extern int32_t get_var_solar_watts();
extern void set_var_solar_watts(int32_t value);
extern int32_t get_var_solar_yield();
extern void set_var_solar_yield(int32_t value);
extern const char *get_var_solar_mode();
extern void set_var_solar_mode(const char *value);
extern const char *get_var_solar_error();
extern void set_var_solar_error(const char *value);
extern const char *get_var_ip_addr();
extern void set_var_ip_addr(const char *value);
extern int32_t get_var_backlight_delay();
extern void set_var_backlight_delay(int32_t value);


#ifdef __cplusplus
}
#endif

#endif /*EEZ_LVGL_UI_VARS_H*/