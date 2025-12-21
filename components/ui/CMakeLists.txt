#file(GLOB_RECURSE SOURCES ${CMAKE_CURRENT_LIST_DIR}/ui/*.c)
file(GLOB_RECURSE SOURCES "*.c" "*.cpp" "*.h")
#file(GLOB_RECURSE FLOW_SOURCES ${CMAKE_CURRENT_LIST_DIR}/ui/*.cpp)

#idf_component_register(SRCS ${SOURCES}
#    INCLUDE_DIRS ${CMAKE_CURRENT_LIST_DIR}
#    REQUIRES "lvgl" "esp_lcd_touch")

idf_component_get_property(lvgl_src lvgl__lvgl COMPONENT_DIR)

idf_component_register(
    SRCS ${SOURCES} ${FLOW_SOURCES}
    INCLUDE_DIRS ${CMAKE_CURRENT_LIST_DIR}/ui
    REQUIRES "lvgl")

idf_component_get_property(lvgl_lib lvgl__lvgl COMPONENT_LIB)
target_compile_options(${lvgl_lib} PRIVATE -Wno-format)
