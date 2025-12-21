#file(GLOB_RECURSE SOURCES ${CMAKE_CURRENT_LIST_DIR}/*.c)

#idf_component_register(SRCS ${SOURCES} 
#    INCLUDE_DIRS ${CMAKE_CURRENT_LIST_DIR} 
#    REQUIRES "lvgl" "esp_lcd_touch")

idf_component_register(
    SRCS "waveshare_rgb_lcd_port.c" "lvgl_port.c"
    INCLUDE_DIRS ".")

idf_component_get_property(lvgl_lib lvgl__lvgl COMPONENT_LIB)
target_compile_options(${lvgl_lib} PRIVATE -Wno-format)