use crate::bitfield::*;
use crate::file_ext::*;
use crate::gpu::*;
use anyhow::*;
use std::convert::{TryFrom, TryInto};
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

    fn decode<F: FnMut(usize, usize, Self::T)>(packet: &[u8; 16], writer: F);

    fn decode_block<F: FnMut(usize, usize, Self::T)>(
        mut block: &[u8], /* BLOCK_LEN or less */
        mut writer: F,
    ) {
        for i in 0..32 {
            if block.is_empty() {
                return;
            }
            let packet = step(&mut block, PACKET_LEN);
            let mut packet_buf = [0; 16];
            packet_buf[0..packet.len()].copy_from_slice(packet);
            let bx = ((i & 2) >> 1) | ((i & 16) >> 3);
            let by = (i & 1) | ((i & 4) >> 1) | ((i & 8) >> 1);
            Self::decode(&packet_buf, |x, y, v| {
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
    type T = [u8; 4];

    fn decode<F: FnMut(usize, usize, Self::T)>(packet: &[u8; 16], writer: F) {
        atsc_decompress_block(packet, 6, 6, writer);
    }
}

fn color5to8(value: u8) -> u8 {
    (value << 3) | (value >> 2)
}

fn color6to8(value: u8) -> u8 {
    (value << 2) | (value >> 4)
}

struct Bc1Unorm;

impl Bc1Unorm {
    fn decode_half<F: FnMut(usize, usize, [u8; 4])>(packet: &[u8; 8], mut writer: F) {
        let c0 = u16::from_le_bytes(packet[0..2].try_into().unwrap());
        let c1 = u16::from_le_bytes(packet[2..4].try_into().unwrap());
        let mut colors = [[0; 4]; 4];
        fn decode_color(c: u16) -> [u8; 4] {
            let (b, g, r) = c.bit_split((5, 6, 5));
            [
                color5to8(r as u8),
                color6to8(g as u8),
                color5to8(b as u8),
                0xFF,
            ]
        }
        colors[0] = decode_color(c0);
        colors[1] = decode_color(c1);
        if c0 > c1 {
            colors[2] = [
                ((2 * colors[0][0] as u32 + colors[1][0] as u32) / 3) as u8,
                ((2 * colors[0][1] as u32 + colors[1][1] as u32) / 3) as u8,
                ((2 * colors[0][2] as u32 + colors[1][2] as u32) / 3) as u8,
                0xFF,
            ];
            colors[3] = [
                ((2 * colors[1][0] as u32 + colors[0][0] as u32) / 3) as u8,
                ((2 * colors[1][1] as u32 + colors[0][1] as u32) / 3) as u8,
                ((2 * colors[1][2] as u32 + colors[0][2] as u32) / 3) as u8,
                0xFF,
            ];
        } else {
            colors[2] = [
                ((colors[0][0] as u32 + colors[1][0] as u32) / 2) as u8,
                ((colors[0][1] as u32 + colors[1][1] as u32) / 2) as u8,
                ((colors[0][2] as u32 + colors[1][2] as u32) / 2) as u8,
                0xFF,
            ];
            colors[3] = [0, 0, 0, 0];
        }
        for (y, &b) in packet[4..8].iter().enumerate() {
            let (b0, b1, b2, b3) = b.bit_split((2, 2, 2, 2));
            writer(0, y, colors[b0 as usize]);
            writer(1, y, colors[b1 as usize]);
            writer(2, y, colors[b2 as usize]);
            writer(3, y, colors[b3 as usize]);
        }
    }
}

impl TexCodec for Bc1Unorm {
    const PACKET_WIDTH: usize = 8;
    const PACKET_HEIGHT: usize = 4;
    type T = [u8; 4];

    fn decode<F: FnMut(usize, usize, Self::T)>(packet: &[u8; 16], mut writer: F) {
        Self::decode_half(packet[0..8].try_into().unwrap(), &mut writer);
        Self::decode_half(packet[8..16].try_into().unwrap(), |x, y, v| {
            writer(x + 4, y, v)
        });
    }
}

struct Bc4Unorm;

impl Bc4Unorm {
    fn decode_half<F: FnMut(usize, usize, [u8; 4])>(packet: &[u8; 8], mut writer: F) {
        let mut c = [0; 8];
        let c0 = packet[0];
        let c1 = packet[1];
        c[0] = c0;
        c[1] = c1;
        if c[0] > c[1] {
            for (i, cc) in c[2..8].iter_mut().enumerate() {
                let f0 = 6 - i as u32;
                let f1 = i as u32 + 1;
                *cc = ((f0 * c0 as u32 + f1 * c1 as u32) / 7) as u8;
            }
        } else {
            for (i, cc) in c[2..6].iter_mut().enumerate() {
                let f0 = 4 - i as u32;
                let f1 = i as u32 + 1;
                *cc = ((f0 * c0 as u32 + f1 * c1 as u32) / 5) as u8;
            }
            c[6] = 0;
            c[7] = 255;
        }
        let mut buf = [0; 4];
        for super_y in 0..2 {
            buf[0..3].copy_from_slice(&packet[2 + super_y * 3..][..3]);
            let mut a = u32::from_le_bytes(buf);
            for y in 0..2 {
                for x in 0..4 {
                    let color = c[(a & 7) as usize];
                    writer(x, y + super_y * 2, [color, color, color, 255]);
                    a >>= 3;
                }
            }
        }
    }
}

impl TexCodec for Bc4Unorm {
    const PACKET_WIDTH: usize = 8;
    const PACKET_HEIGHT: usize = 4;
    type T = [u8; 4];

    fn decode<F: FnMut(usize, usize, Self::T)>(packet: &[u8; 16], mut writer: F) {
        Self::decode_half(packet[0..8].try_into().unwrap(), &mut writer);
        Self::decode_half(packet[8..16].try_into().unwrap(), |x, y, v| {
            writer(x + 4, y, v)
        });
    }
}

struct Bc7Unorm;

impl TexCodec for Bc7Unorm {
    const PACKET_WIDTH: usize = 4;
    const PACKET_HEIGHT: usize = 4;
    type T = [u8; 4];

    fn decode<F: FnMut(usize, usize, Self::T)>(packet: &[u8; 16], writer: F) {
        bc7_decompress_block(packet, writer);
    }
}

pub struct Tex {
    pub format: u32,
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
            len_padded: u32,
        }

        let texture_infos = (0..texture_count)
            .map(|_| {
                (0..mipmap_count)
                    .map(|_| {
                        Ok(TextureInfo {
                            offset: file.read_u64()?,
                            len: file.read_u32()?,
                            len_padded: file.read_u32()?,
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
                        if t.len_padded < t.len {
                            bail!("Padded len should be larger than len");
                        }
                        let mut buffer = vec![0; usize::try_from(t.len_padded)?];
                        file.seek(SeekFrom::Start(t.offset))?;
                        file.read_exact(&mut buffer[0..usize::try_from(t.len)?])?;
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
        let width = usize::try_from(self.width >> mipmap)?;
        let height = usize::try_from(self.height >> mipmap)?;
        let super_height = 1 << self.log_super_height;

        let mut data = vec![0; width * height * 4];
        let writer = |x, y, v: [u8; 4]| {
            let i = (x + y * (width)) * 4;
            data[i..][..4].copy_from_slice(&v);
        };
        let decoder = match self.format {
            0x47 | 0x48 => Bc1Unorm::decode_image,
            0x50 => Bc4Unorm::decode_image,
            0x62 | 0x63 => Bc7Unorm::decode_image,
            0x40F => Atsc6x6::decode_image,
            x => bail!("unsupported format {:08X}", x),
        };
        decoder(&texture, width, height, super_height, writer);
        RgbaImage::new(data, u32::try_from(width)?, u32::try_from(height)?).save_png(output)?;

        Ok(())
    }
}
