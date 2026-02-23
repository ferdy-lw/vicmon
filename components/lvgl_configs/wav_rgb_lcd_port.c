/*
 * SPDX-FileCopyrightText: 2022-2025 Espressif Systems (Shanghai) CO LTD
 *
 * SPDX-License-Identifier: Apache-2.0
 */

#include "esp_err.h"
#include "esp_log.h"
#include "esp_check.h"
#include "esp_idf_version.h"
#include "driver/gpio.h"
#include "driver/i2c.h"
#include "driver/i2c_master.h"
#include "esp_lcd_panel_io.h"
#include "esp_lcd_panel_ops.h"
#include "esp_lcd_panel_rgb.h"
#include "esp_lvgl_port.h"
#include "lvgl.h"
#include "esp_lcd_touch_gt911.h"
#include "wav_rgb_lcd_port.h"


#define ESP_PANEL_USE_1024_600_LCD (0)                // 0: 800x480, 1: 1024x600

#if ESP_PANEL_USE_1024_600_LCD
#define EXAMPLE_LCD_H_RES   (1024)
#define EXAMPLE_LCD_V_RES   (600)
#define EXAMPLE_LCD_PIXEL_CLOCK_HZ (21 * 1000 * 1000)
#else
#define EXAMPLE_LCD_H_RES   (800)
#define EXAMPLE_LCD_V_RES   (480)
#define EXAMPLE_LCD_PIXEL_CLOCK_HZ (16 * 1000 * 1000)
#endif

/* https://docs.espressif.com/projects/esp-idf/en/v5.4.3/esp32s3/api-reference/peripherals/lcd/rgb_lcd.html */
/* LCD settings */
#define EXAMPLE_LCD_LVGL_FULL_REFRESH           (0)
#define EXAMPLE_LCD_LVGL_DIRECT_MODE            (0) // YES
#define EXAMPLE_LCD_LVGL_AVOID_TEAR             (0) // YES
#define EXAMPLE_LCD_DRAW_BUFF_DOUBLE           (1)// Not(0) Using(1) double buffer
#define EXAMPLE_LCD_DRAW_BUFF_HEIGHT            (100) // buffer size H_RES* BUFF_HEIGHT, when not using full refresh or direct
#define EXAMPLE_LCD_RGB_BUFFER_NUMS             (1) // (1) single buffer (2) double buffer - number of frame buffers in the panel (SPIRAM)
#define EXAMPLE_LCD_RGB_BOUNCE_BUFFER_MODE      (1) // YES // use a bounce buffer for the panel (DMA), when using BB only need one FB
#define EXAMPLE_LCD_RGB_BOUNCE_BUFFER_HEIGHT    (10) //(10)  // bounce buffer size H_RES * BUFFER_HEIGHT * bpp / 8 (must: V_RES % BUFFER_HEIGHT == 0)

/* LCD pins */
#define EXAMPLE_LCD_GPIO_VSYNC     (GPIO_NUM_3)
#define EXAMPLE_LCD_GPIO_HSYNC     (GPIO_NUM_46)
#define EXAMPLE_LCD_GPIO_DE        (GPIO_NUM_5)
#define EXAMPLE_LCD_GPIO_PCLK      (GPIO_NUM_7)
#define EXAMPLE_LCD_GPIO_DISP      (GPIO_NUM_NC)
#define EXAMPLE_LCD_GPIO_DATA0     (GPIO_NUM_14)
#define EXAMPLE_LCD_GPIO_DATA1     (GPIO_NUM_38)
#define EXAMPLE_LCD_GPIO_DATA2     (GPIO_NUM_18)
#define EXAMPLE_LCD_GPIO_DATA3     (GPIO_NUM_17)
#define EXAMPLE_LCD_GPIO_DATA4     (GPIO_NUM_10)
#define EXAMPLE_LCD_GPIO_DATA5     (GPIO_NUM_39)
#define EXAMPLE_LCD_GPIO_DATA6     (GPIO_NUM_0)
#define EXAMPLE_LCD_GPIO_DATA7     (GPIO_NUM_45)
#define EXAMPLE_LCD_GPIO_DATA8     (GPIO_NUM_48)
#define EXAMPLE_LCD_GPIO_DATA9     (GPIO_NUM_47)
#define EXAMPLE_LCD_GPIO_DATA10    (GPIO_NUM_21)
#define EXAMPLE_LCD_GPIO_DATA11    (GPIO_NUM_1)
#define EXAMPLE_LCD_GPIO_DATA12    (GPIO_NUM_2)
#define EXAMPLE_LCD_GPIO_DATA13    (GPIO_NUM_42)
#define EXAMPLE_LCD_GPIO_DATA14    (GPIO_NUM_41)
#define EXAMPLE_LCD_GPIO_DATA15    (GPIO_NUM_40)

/* Touch settings */
#define I2C_MASTER_SCL_IO 9         /*!< GPIO number used for I2C master clock */
#define I2C_MASTER_SDA_IO 8         /*!< GPIO number used for I2C master data  */
#define I2C_MASTER_NUM 0            /*!< I2C master i2c port number, the number of i2c peripheral interfaces available will depend on the chip */
#define I2C_MASTER_FREQ_HZ 400000   /*!< I2C master clock frequency */
#define I2C_MASTER_TX_BUF_DISABLE 0 /*!< I2C master doesn't need buffer */
#define I2C_MASTER_RX_BUF_DISABLE 0 /*!< I2C master doesn't need buffer */
#define I2C_MASTER_TIMEOUT_MS 1000

#define GPIO_INPUT_IO_4 4
#define GPIO_INPUT_PIN_SEL 1ULL << GPIO_INPUT_IO_4
/* LCD touch pins */
#define EXAMPLE_TOUCH_I2C_NUM       (0)
#define EXAMPLE_TOUCH_I2C_CLK_HZ    (400000)
#define EXAMPLE_TOUCH_I2C_SCL       (GPIO_NUM_9)
#define EXAMPLE_TOUCH_I2C_SDA       (GPIO_NUM_8)

#if ESP_PANEL_USE_1024_600_LCD
#define EXAMPLE_LCD_PANEL_RGB_TIMING()  \
    {                                               \
        .pclk_hz = EXAMPLE_LCD_PIXEL_CLOCK_HZ,       \
        .h_res = EXAMPLE_LCD_H_RES,                 \
        .v_res = EXAMPLE_LCD_V_RES,                 \
        .hsync_pulse_width = 30,                    \
        .hsync_back_porch = 145,                     \
        .hsync_front_porch = 170,                    \
        .vsync_pulse_width = 2,                    \
        .vsync_back_porch = 23,                     \
        .vsync_front_porch = 12,                    \
        .flags.pclk_active_neg = true,              \
    }
#else
#define EXAMPLE_LCD_PANEL_RGB_TIMING()  \
    {                                               \
        .pclk_hz = EXAMPLE_LCD_PIXEL_CLOCK_HZ,       \
        .h_res = EXAMPLE_LCD_H_RES,                 \
        .v_res = EXAMPLE_LCD_V_RES,                 \
        .hsync_pulse_width = 4,                    \
        .hsync_back_porch = 8,                     \
        .hsync_front_porch = 8,                    \
        .vsync_pulse_width = 4,                    \
        .vsync_back_porch = 8,                     \
        .vsync_front_porch = 8,                    \
        .flags.pclk_active_neg = true,              \
    }
#endif

static const char *TAG = "RGB_LCD";

/**********************
* Function definitions
***********************/
static esp_err_t i2c_master_init(void);
static void gpio_init(void);
static void waveshare_esp32_s3_touch_reset();


/* LCD IO and panel */
static esp_lcd_panel_handle_t lcd_panel = NULL;
static esp_lcd_touch_handle_t touch_handle = NULL;

/* LVGL display and touch */
lv_display_t *lvgl_disp = NULL;
lv_indev_t *lvgl_touch_indev = NULL;

esp_err_t app_lcd_init(void)
{
    esp_err_t ret = ESP_OK;

    /* LCD initialization */
    ESP_LOGI(TAG, "Initialize RGB panel");
    esp_lcd_rgb_panel_config_t panel_conf = {
        .clk_src = LCD_CLK_SRC_DEFAULT,
#if ESP_IDF_VERSION < ESP_IDF_VERSION_VAL(5, 3, 0)
        .psram_trans_align = 64,
#else
        .dma_burst_size = 64,
#endif
        .data_width = 16,
#if ESP_IDF_VERSION >= ESP_IDF_VERSION_VAL(6,0,0)
        .in_color_format = LCD_COLOR_FMT_RGB565,
#else
        .bits_per_pixel = 16,
#endif
        .de_gpio_num = EXAMPLE_LCD_GPIO_DE,
        .pclk_gpio_num = EXAMPLE_LCD_GPIO_PCLK,
        .vsync_gpio_num = EXAMPLE_LCD_GPIO_VSYNC,
        .hsync_gpio_num = EXAMPLE_LCD_GPIO_HSYNC,
        .disp_gpio_num = EXAMPLE_LCD_GPIO_DISP,
        .data_gpio_nums = {
            EXAMPLE_LCD_GPIO_DATA0,
            EXAMPLE_LCD_GPIO_DATA1,
            EXAMPLE_LCD_GPIO_DATA2,
            EXAMPLE_LCD_GPIO_DATA3,
            EXAMPLE_LCD_GPIO_DATA4,
            EXAMPLE_LCD_GPIO_DATA5,
            EXAMPLE_LCD_GPIO_DATA6,
            EXAMPLE_LCD_GPIO_DATA7,
            EXAMPLE_LCD_GPIO_DATA8,
            EXAMPLE_LCD_GPIO_DATA9,
            EXAMPLE_LCD_GPIO_DATA10,
            EXAMPLE_LCD_GPIO_DATA11,
            EXAMPLE_LCD_GPIO_DATA12,
            EXAMPLE_LCD_GPIO_DATA13,
            EXAMPLE_LCD_GPIO_DATA14,
            EXAMPLE_LCD_GPIO_DATA15,
        },
        .timings = EXAMPLE_LCD_PANEL_RGB_TIMING(),
        .flags.fb_in_psram = true,
        .num_fbs = EXAMPLE_LCD_RGB_BUFFER_NUMS,
#if EXAMPLE_LCD_RGB_BOUNCE_BUFFER_MODE
        .bounce_buffer_size_px = EXAMPLE_LCD_H_RES * EXAMPLE_LCD_RGB_BOUNCE_BUFFER_HEIGHT,
#endif
    };
    ESP_GOTO_ON_ERROR(esp_lcd_new_rgb_panel(&panel_conf, &lcd_panel), err, TAG, "RGB init failed");
    ESP_GOTO_ON_ERROR(esp_lcd_panel_init(lcd_panel), err, TAG, "LCD init failed");

    return ret;

err:
    if (lcd_panel) {
        esp_lcd_panel_del(lcd_panel);
    }
    return ret;
}

esp_err_t app_touch_init(void)
{
    ESP_LOGI(TAG, "Initialize I2C bus");   // Log the initialization of the I2C bus
    i2c_master_init();                     // Initialize the I2C master
    ESP_LOGI(TAG, "Initialize GPIO");      // Log GPIO initialization
    gpio_init();                           // Initialize GPIO pins
    ESP_LOGI(TAG, "Initialize Touch LCD"); // Log touch LCD initialization
    waveshare_esp32_s3_touch_reset();      // Reset the touch panel

    /* Initialize touch HW */
    const esp_lcd_touch_config_t tp_cfg = {
        .x_max = EXAMPLE_LCD_H_RES,
        .y_max = EXAMPLE_LCD_V_RES,
        .rst_gpio_num = GPIO_NUM_NC,
        .int_gpio_num = GPIO_NUM_NC,
        .levels = {
            .reset = 0,
            .interrupt = 0,
        },
        .flags = {
            .swap_xy = 0,
            .mirror_x = 0,
            .mirror_y = 0,
        },
    };
    esp_lcd_panel_io_handle_t tp_io_handle = NULL;
    esp_lcd_panel_io_i2c_config_t tp_io_config = ESP_LCD_TOUCH_IO_I2C_GT911_CONFIG();

    ESP_LOGI(TAG, "Initialize I2C panel IO");          // Log I2C panel I/O initialization
    ESP_RETURN_ON_ERROR(esp_lcd_new_panel_io_i2c((esp_lcd_i2c_bus_handle_t)I2C_MASTER_NUM, &tp_io_config, &tp_io_handle), TAG, "");

    ESP_LOGI(TAG, "Initialize touch controller GT911"); // Log touch controller initialization
    return esp_lcd_touch_new_i2c_gt911(tp_io_handle, &tp_cfg, &touch_handle);
}

esp_err_t app_lvgl_init(void)
{
    /* Initialize LVGL */
    const lvgl_port_cfg_t lvgl_cfg = {
        .task_priority = 4,         /* LVGL task priority */
        .task_stack = 7168, //6144,         /* LVGL task stack size */
        .task_affinity = -1,        /* LVGL task pinned to core (-1 is no affinity) */
        .task_max_sleep_ms = 500,   /* Maximum sleep in LVGL task */
        .timer_period_ms = 5        /* LVGL timer tick period in ms */
    };
    ESP_RETURN_ON_ERROR(lvgl_port_init(&lvgl_cfg), TAG, "LVGL port initialization failed");


    uint32_t buff_size = EXAMPLE_LCD_H_RES * EXAMPLE_LCD_DRAW_BUFF_HEIGHT;
#if EXAMPLE_LCD_LVGL_FULL_REFRESH || EXAMPLE_LCD_LVGL_DIRECT_MODE
    buff_size = EXAMPLE_LCD_H_RES * EXAMPLE_LCD_V_RES;
#endif

    /* Add LCD screen */
    ESP_LOGD(TAG, "Add LCD screen");
    const lvgl_port_display_cfg_t disp_cfg = {
        .panel_handle = lcd_panel,
        .buffer_size = buff_size,
        .double_buffer = EXAMPLE_LCD_DRAW_BUFF_DOUBLE,
        .hres = EXAMPLE_LCD_H_RES,
        .vres = EXAMPLE_LCD_V_RES,
        .monochrome = false,
#if LVGL_VERSION_MAJOR >= 9
        .color_format = LV_COLOR_FORMAT_RGB565,
#endif
        .rotation = {
            .swap_xy = false,
            .mirror_x = false,
            .mirror_y = false,
        },
        .flags = {
            .sw_rotate = true,
            .buff_dma = false,
            .buff_spiram = true,
#if EXAMPLE_LCD_LVGL_FULL_REFRESH
            .full_refresh = true,
#elif EXAMPLE_LCD_LVGL_DIRECT_MODE
            .direct_mode = true,
#endif
#if LVGL_VERSION_MAJOR >= 9
            .swap_bytes = false,
#endif
        }
    };
    const lvgl_port_display_rgb_cfg_t rgb_cfg = {
        .flags = {
#if EXAMPLE_LCD_RGB_BOUNCE_BUFFER_MODE
            .bb_mode = true,
#else
            .bb_mode = false,
#endif
#if EXAMPLE_LCD_LVGL_AVOID_TEAR
            .avoid_tearing = true,
#else
            .avoid_tearing = false,
#endif
        }
    };
    lvgl_disp = lvgl_port_add_disp_rgb(&disp_cfg, &rgb_cfg);

    /* Add touch input (for selected screen) */
    const lvgl_port_touch_cfg_t touch_cfg = {
        .disp = lvgl_disp,
        .handle = touch_handle,
    };
    lvgl_touch_indev = lvgl_port_add_touch(&touch_cfg);

    return ESP_OK;
}


/**
 * @brief I2C master initialization
 */
static esp_err_t i2c_master_init(void)
{
    int i2c_master_port = I2C_MASTER_NUM;

    i2c_config_t i2c_conf = {
        .mode = I2C_MODE_MASTER,
        .sda_io_num = I2C_MASTER_SDA_IO,
        .scl_io_num = I2C_MASTER_SCL_IO,
        .sda_pullup_en = GPIO_PULLUP_ENABLE,
        .scl_pullup_en = GPIO_PULLUP_ENABLE,
        .master.clk_speed = I2C_MASTER_FREQ_HZ,
    };

    // Configure I2C parameters
    i2c_param_config(i2c_master_port, &i2c_conf);

    // Install I2C driver
    return i2c_driver_install(i2c_master_port, i2c_conf.mode, 0, 0, 0);
}

// GPIO initialization
static void gpio_init(void)
{
    // Zero-initialize the config structure
    gpio_config_t io_conf = {};
    // Disable interrupt
    io_conf.intr_type = GPIO_INTR_DISABLE;
    // Bit mask of the pins, use GPIO4 here
    io_conf.pin_bit_mask = GPIO_INPUT_PIN_SEL;
    // Set as input mode
    io_conf.mode = GPIO_MODE_OUTPUT;

    gpio_config(&io_conf);
}

// Reset the touch screen
static void waveshare_esp32_s3_touch_reset()
{
    uint8_t write_buf = 0x01;
    i2c_master_write_to_device(I2C_MASTER_NUM, 0x24, &write_buf, 1, I2C_MASTER_TIMEOUT_MS / portTICK_PERIOD_MS);

    // Reset the touch screen. It is recommended to reset the touch screen before using it.
    write_buf = 0x2C;
    i2c_master_write_to_device(I2C_MASTER_NUM, 0x38, &write_buf, 1, I2C_MASTER_TIMEOUT_MS / portTICK_PERIOD_MS);
    esp_rom_delay_us(100 * 1000);
    gpio_set_level(GPIO_INPUT_IO_4, 0);
    esp_rom_delay_us(100 * 1000);
    write_buf = 0x2E;
    i2c_master_write_to_device(I2C_MASTER_NUM, 0x38, &write_buf, 1, I2C_MASTER_TIMEOUT_MS / portTICK_PERIOD_MS);
    esp_rom_delay_us(200 * 1000);
}

/******************************* Turn on the screen backlight **************************************/
esp_err_t waveshare_rgb_lcd_bl_on()
{
    // Configure CH422G to output mode
    uint8_t write_buf = 0x01;
    i2c_master_write_to_device(I2C_MASTER_NUM, 0x24, &write_buf, 1, I2C_MASTER_TIMEOUT_MS / portTICK_PERIOD_MS);

    // Pull the backlight pin high to light the screen backlight
    write_buf = 0x1E;
    i2c_master_write_to_device(I2C_MASTER_NUM, 0x38, &write_buf, 1, I2C_MASTER_TIMEOUT_MS / portTICK_PERIOD_MS);
    return ESP_OK;
}

/******************************* Turn off the screen backlight **************************************/
esp_err_t waveshare_rgb_lcd_bl_off()
{
    // Configure CH422G to output mode
    uint8_t write_buf = 0x01;
    i2c_master_write_to_device(I2C_MASTER_NUM, 0x24, &write_buf, 1, I2C_MASTER_TIMEOUT_MS / portTICK_PERIOD_MS);

    // Turn off the screen backlight by pulling the backlight pin low
    write_buf = 0x1A;
    i2c_master_write_to_device(I2C_MASTER_NUM, 0x38, &write_buf, 1, I2C_MASTER_TIMEOUT_MS / portTICK_PERIOD_MS);
    return ESP_OK;
}
