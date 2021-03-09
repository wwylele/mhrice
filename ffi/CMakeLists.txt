cmake_minimum_required(VERSION 3.13.0)
project(ffi-shim)


option(OPTION_ASTC_TESTS "" OFF)
add_subdirectory(astc-codec)

add_library(ffi-shim ffi_shim.cpp bc7/bc7decomp.cpp)
target_link_libraries(ffi-shim footprint astc_utils astc-codec)

install(TARGETS ffi-shim footprint astc_utils astc-codec DESTINATION lib)
