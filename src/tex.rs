use crate::bitfield::*;
use crate::file_ext::*;
use crate::gpu::*;
use anyhow::*;
use std::convert::TryFrom;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

/*

Let's talk about Switch's texture layout (only covers 2D texture here)

On top of pixels, the smalliest unit is a packet.
A packet is always 16 bytes, and represents a small rectangle area of pixels.
The size of this rectangle depends on the texture format.
For example:
 - for RGBA8, a packet is a 4x1 rectangle. Each pixel uses 4 bytes.
 - for ASTC6x6, which also use each 16 bytes as one decoding unit, a packet is a 6x6 square,

On top of packets, the next unit is a block, which always contains 4x8 packets,
and the packet layout in one block is like this:

0  2  16 18
1  3  17 19
4  6  20 22
5  7  21 23
8  10 24 26
9  11 25 27
12 14 28 30
13 15 29 31

Blocks fill the texture in y direction first until hitting "super_height" (H),
and then move on to the next x and fill in y direction again... till filling all x, and reset x,
filling in y direction starting below the previous super row,... . The layout would be

B[0]    B[H]    ...   B[(k-1)H]
B[1]    B[H+1]        B[(k-1)H+1]
B[2]    B[H+2]        B[(k-1)H+2]
...                  ...
B[H-1]  B[2H-1] ...   B[kH-1]
B[kH]   ...
B[kH+1]
...
B[(k+1)H-1]

*/

const PACKET_LEN: usize = 16;
const BLOCK_LEN: usize = PACKET_LEN * 4 * 8;

fn step<'a>(data: &'_ mut &'a [u8], max_len: usize) -> &'a [u8] {
    let len = std::cmp::min(data.len(), max_len);
    let ret = &data[0..len];
    *data = &data[len..];
    ret
}

trait TexCodec {
    const PACKET_WIDTH: usize;
    const PACKET_HEIGHT: usize;
    type T;

    fn decode<F: FnMut(usize, usize, Self::T)>(
        packet: &[u8], /* PACKET_LEN or less */
        writer: F,
    );

    fn decode_block<F: FnMut(usize, usize, Self::T)>(
        mut block: &[u8], /* BLOCK_LEN or less */
        mut writer: F,
    ) {
        for i in 0..32 {
            if block.is_empty() {
                return;
            }
            let packet = step(&mut block, PACKET_LEN);
            let bx = ((i & 2) >> 1) | ((i & 16) >> 3);
            let by = (i & 1) | ((i & 4) >> 1) | ((i & 8) >> 1);
            Self::decode(packet, |x, y, v| {
                writer(x + bx * Self::PACKET_WIDTH, y + by * Self::PACKET_HEIGHT, v)
            })
        }
    }

    fn decode_image<F: FnMut(usize, usize, Self::T)>(
        mut data: &[u8],
        width: usize,
        height: usize,
        super_height: usize,
        mut writer: F,
    ) {
        let mut writer = |x, y, v| {
            if x >= width || y >= height {
                return;
            }
            writer(x, y, v)
        };

        let block_width = Self::PACKET_WIDTH * 4;
        let block_height = Self::PACKET_HEIGHT * 8;
        let super_width = (width + block_width - 1) / block_width;
        let hyper_height = super_height * block_height;

        for hyper_y in 0.. {
            for super_x in 0..super_width {
                for super_y in 0..super_height {
                    if data.is_empty() {
                        return;
                    }
                    let block = step(&mut data, BLOCK_LEN);
                    Self::decode_block(block, |x, y, v| {
                        writer(
                            x + block_width * super_x,
                            y + block_height * super_y + hyper_height * hyper_y,
                            v,
                        )
                    })
                }
            }
        }
    }
}

struct Atsc6x6;

impl TexCodec for Atsc6x6 {
    const PACKET_WIDTH: usize = 6;
    const PACKET_HEIGHT: usize = 6;
    type T = (u8, u8, u8, u8);

    fn decode<F: FnMut(usize, usize, Self::T)>(
        packet: &[u8], /* PACKET_LEN or less */
        writer: F,
    ) {
        let mut in_buf = [0; 16];
        in_buf[0..packet.len()].copy_from_slice(packet);
        atsc_decompress_block(&in_buf, 6, 6, writer);
    }
}

pub struct Tex {
    format: u32,
    width: u16,
    height: u16,
    depth: u16,
    textures: Vec<Vec<Vec<u8>>>,
    log_super_height: u8,
    log_super_depth: u8,
}

impl Tex {
    pub fn new<F: Read + Seek>(mut file: F) -> Result<Tex> {
        if &file.read_magic()? != b"TEX\0" {
            bail!("Wrong magic for TEX");
        }
        if file.read_u32()? != 0x1c {
            bail!("Wrong version for TEX");
        }
        let width = file.read_u16()?;
        let height = file.read_u16()?;
        let depth = file.read_u16()?;
        let (texture_count, mipmap_count) = file.read_u16()?.bit_split((12, 4));

        let format = file.read_u32()?;
        let x = file.read_u32()?;
        if x != 1 {
            bail!("Expected 1")
        }
        let _b = file.read_u32()?;
        let _c = file.read_u32()?;
        let (log_super_height, log_super_depth) = file.read_u8()?.bit_split((4, 4));
        let _d = file.read_u8()?;
        let x = file.read_u16()?;
        if x != 0 {
            bail!("Expected 0")
        }
        let x = file.read_u16()?;
        if x != 7 {
            bail!("Expected 7")
        }
        let x = file.read_u16()?;
        if x != 1 {
            bail!("Expected 1")
        }

        struct TextureInfo {
            offset: u64,
            len: u32,
            _len_padded: u32,
        }

        let texture_infos = (0..texture_count)
            .map(|_| {
                (0..mipmap_count)
                    .map(|_| {
                        Ok(TextureInfo {
                            offset: file.read_u64()?,
                            len: file.read_u32()?,
                            _len_padded: file.read_u32()?,
                        })
                    })
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<_>>>()?;

        file.seek_noop(texture_infos[0][0].offset)?;

        let textures = texture_infos
            .into_iter()
            .map(|v| {
                v.into_iter()
                    .map(|t| {
                        let mut buffer = vec![0; usize::try_from(t.len)?];
                        file.seek(SeekFrom::Start(t.offset))?;
                        file.read_exact(&mut buffer)?;
                        Ok(buffer)
                    })
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Tex {
            format,
            width,
            height,
            depth,
            textures,
            log_super_height,
            log_super_depth,
        })
    }

    pub fn save_png(&self, index: usize, mipmap: usize, output: &Path) -> anyhow::Result<()> {
        if self.depth != 1 {
            bail!("Volume texture")
        }
        let texture = &self.textures[index][mipmap];
        let width = self.width >> mipmap;
        let height = self.height >> mipmap;

        match self.format {
            // Astc6x6UnormSrgb
            0x40F => {
                let mut data = vec![0; width as usize * height as usize * 4];
                let writer = |x, y, v: (u8, u8, u8, u8)| {
                    let i = (x + y * (width as usize)) * 4;
                    data[i] = v.0;
                    data[i + 1] = v.1;
                    data[i + 2] = v.2;
                    data[i + 3] = v.3;
                };
                Atsc6x6::decode_image(
                    &texture,
                    width as usize,
                    height as usize,
                    1 << self.log_super_height,
                    writer,
                );

                RgbaImage::new(data, width as u32, height as u32).save_png(output)?
            }

            x => bail!("unsupported format {:08X}", x),
        }

        Ok(())
    }
}
