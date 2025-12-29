#include "images.h"

const ext_img_desc_t images[10] = {
    { "batt_60", &img_batt_60 },
    { "batt_80", &img_batt_80 },
    { "batt_90", &img_batt_90 },
    { "batt_full", &img_batt_full },
    { "batt_unknown", &img_batt_unknown },
    { "temp", &img_temp },
    { "power", &img_power },
    { "solar", &img_solar },
    { "sun", &img_sun },
    { "settings", &img_settings },
};
