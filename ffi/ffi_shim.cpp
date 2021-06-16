#include <cstdint>
#include "bc7decomp.h"

using namespace bc7decomp;

void fill_error(std::uint8_t block_width, std::uint8_t block_height,
        std::uint8_t* out_buffer) {

    for (std::uint8_t i = 0; i < block_width * block_height; ++i) {
        *(out_buffer++) = 0xFF;
        *(out_buffer++) = 0;
        *(out_buffer++) = 0xFF;
        *(out_buffer++) = 0xFF;
    }
}
extern "C" {
    void bc7_decompress_block_ffi(const std::uint8_t* input_buffer, std::uint8_t* out_buffer) {
        color_rgba pixels[16];
        if (unpack_bc7(input_buffer, pixels)) {
            for (int i = 0; i < 16; ++i) {
                out_buffer[i * 4] = pixels[i].r;
                out_buffer[i * 4 + 1] = pixels[i].g;
                out_buffer[i * 4 + 2] = pixels[i].b;
                out_buffer[i * 4 + 3] = pixels[i].a;
            }
        } else {
            fill_error(4, 4, out_buffer);
        }
    }
}
