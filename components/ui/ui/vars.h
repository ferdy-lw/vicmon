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

extern float get_var_soc();
extern void set_var_soc(float value);
extern bool get_var_inverter_mode();
extern void set_var_inverter_mode(bool value);
extern int32_t get_var_solar_watts();
extern void set_var_solar_watts(int32_t value);
extern int32_t get_var_ac_watts();
extern void set_var_ac_watts(int32_t value);
extern float get_var_batt_volt();
extern void set_var_batt_volt(float value);
extern float get_var_batt_amp();
extern void set_var_batt_amp(float value);
extern int32_t get_var_batt_temp();
extern void set_var_batt_temp(int32_t value);


#ifdef __cplusplus
}
#endif

#endif /*EEZ_LVGL_UI_VARS_H*/