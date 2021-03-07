cmake_minimum_required(VERSION 3.1.0)
project(astc-shim)


option(OPTION_ASTC_TESTS "" OFF)
add_subdirectory(astc-codec)

add_library(astc-shim astc_shim.cpp)
target_link_libraries(astc-shim footprint astc_utils astc-codec)

install(TARGETS astc-shim DESTINATION lib)
