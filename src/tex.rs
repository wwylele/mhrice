use crate::bitfield::*;
use crate::file_ext::*;
use crate::gpu::*;
use anyhow::*;
use std::convert::{TryFrom, TryInto};
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

/*

Let's talk about Switch's texture layout (only covers 2D texture here)

On top of pixels, the smallest unit is a packet.
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

On top of blocks, the next unit is a super block, which contains W*H blocks,
where W (super_width) and H (super_height) are configurable and stored in the .tex file
(as log of 2). In a super block, blocks fill in the y direction first, then in x direction.
So the layout in a super block would be like

B[0]         B[H]        ...       B[(W-1)*H]
B[1]         B[H+1]      ...       B[(W-1)*H+1]
B[2]         B[H+2]      ...       B[(W-1)*H+2]
...          ...         ...       ...
B[H-1]       B[2*H-1]    ...       B[W*H-1]

Finally, super blocks fill the texture.
Super blocks fill in the x direction first, then in y direction.
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
        super_width: usize,
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
        let super_block_width = block_width * super_width;
        let super_block_height = block_height * super_height;
        let hyper_width = (width + super_block_width - 1) / super_block_width;
        let hyper_height = (height + super_block_height - 1) / super_block_height;

        for hyper_y in 0..hyper_height {
            for hyper_x in 0..hyper_width {
                for super_x in 0..super_width {
                    for super_y in 0..super_height {
                        if data.is_empty() {
                            return;
                        }
                        let block = step(&mut data, BLOCK_LEN);
                        Self::decode_block(block, |x, y, v| {
                            writer(
                                x + block_width * super_x + super_block_width * hyper_x,
                                y + block_height * super_y + super_block_height * hyper_y,
                                v,
                            )
                        })
                    }
                }
            }
        }
    }
}

struct Astc<const W: usize, const H: usize>;

impl<const W: usize, const H: usize> TexCodec for Astc<W, H> {
    const PACKET_WIDTH: usize = W;
    const PACKET_HEIGHT: usize = H;
    type T = [u8; 4];

    fn decode<F: FnMut(usize, usize, Self::T)>(packet: &[u8; 16], mut writer: F) {
        astc_decode::astc_decode_block(
            packet,
            astc_decode::Footprint::new(W as u32, H as u32),
            |x, y, v| writer(x as usize, y as usize, v),
        );
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

struct Bc3Unorm;

impl TexCodec for Bc3Unorm {
    const PACKET_WIDTH: usize = 4;
    const PACKET_HEIGHT: usize = 4;
    type T = [u8; 4];

    fn decode<F: FnMut(usize, usize, Self::T)>(packet: &[u8; 16], mut writer: F) {
        let mut color_buf = [[[0; 3]; 4]; 4];
        let mut alpha_buf = [[0; 4]; 4];
        Bc4Unorm::decode_half(packet[0..8].try_into().unwrap(), |x, y, v| {
            alpha_buf[x][y] = v[0]
        });
        Bc1Unorm::decode_half(packet[8..16].try_into().unwrap(), |x, y, v| {
            color_buf[x][y] = [v[0], v[1], v[2]]
        });
        for x in 0..4 {
            for y in 0..4 {
                let color = color_buf[x][y];
                writer(x, y, [color[0], color[1], color[2], alpha_buf[x][y]])
            }
        }
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

struct Bc5Unorm;

impl TexCodec for Bc5Unorm {
    const PACKET_WIDTH: usize = 4;
    const PACKET_HEIGHT: usize = 4;
    type T = [u8; 4];

    fn decode<F: FnMut(usize, usize, Self::T)>(packet: &[u8; 16], mut writer: F) {
        let mut red_buf = [[0; 4]; 4];
        let mut green_buf = [[0; 4]; 4];
        Bc4Unorm::decode_half(packet[0..8].try_into().unwrap(), |x, y, v| {
            red_buf[x][y] = v[0]
        });
        Bc4Unorm::decode_half(packet[8..16].try_into().unwrap(), |x, y, v| {
            green_buf[x][y] = v[0]
        });
        for x in 0..4 {
            for y in 0..4 {
                writer(x, y, [red_buf[x][y], green_buf[x][y], 0, 255])
            }
        }
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

struct R8G8B8A8Unorm;

impl TexCodec for R8G8B8A8Unorm {
    const PACKET_WIDTH: usize = 4;
    const PACKET_HEIGHT: usize = 1;
    type T = [u8; 4];

    fn decode<F: FnMut(usize, usize, Self::T)>(packet: &[u8; 16], mut writer: F) {
        writer(0, 0, packet[0..4].try_into().unwrap());
        writer(1, 0, packet[4..8].try_into().unwrap());
        writer(2, 0, packet[8..12].try_into().unwrap());
        writer(3, 0, packet[12..16].try_into().unwrap());
    }
}

struct R8Unorm;

impl TexCodec for R8Unorm {
    const PACKET_WIDTH: usize = 16;
    const PACKET_HEIGHT: usize = 1;
    type T = [u8; 4];

    fn decode<F: FnMut(usize, usize, Self::T)>(packet: &[u8; 16], mut writer: F) {
        for (i, &c) in packet.iter().enumerate() {
            writer(i, 0, [c, c, c, 255])
        }
    }
}

struct R8G8Unorm;

impl TexCodec for R8G8Unorm {
    const PACKET_WIDTH: usize = 8;
    const PACKET_HEIGHT: usize = 1;
    type T = [u8; 4];

    fn decode<F: FnMut(usize, usize, Self::T)>(packet: &[u8; 16], mut writer: F) {
        for i in 0..8 {
            let r = packet[i + 1];
            let g = packet[i * 2 + 1];
            writer(i, 0, [r, g, 0, 255])
        }
    }
}

pub struct Tex {
    format: u32,
    width: u16,
    height: u16,
    depth: u16,
    textures: Vec<Vec<Vec<u8>>>,
    log_super_width: u8,
    log_super_height: u8,
    #[allow(dead_code)]
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
        let log_super_width = file.read_u8()?;
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
                        if file
                            .read_exact(&mut buffer[0..usize::try_from(t.len)?])
                            .is_err()
                        {
                            // Some texture is seen with a few bytes missing. Don't know why
                            eprintln!("Incomplete texture data")
                        }
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
            log_super_width,
            log_super_height,
            log_super_depth,
        })
    }

    pub fn to_rgba(&self, index: usize, mipmap: usize) -> anyhow::Result<RgbaImage> {
        if self.depth != 1 {
            bail!("Volume texture")
        }
        let texture = &self.textures[index][mipmap];
        let width = usize::try_from(self.width >> mipmap)?;
        let height = usize::try_from(self.height >> mipmap)?;
        let super_width = 1 << self.log_super_width;
        let super_height = 1 << self.log_super_height;

        let mut data = vec![0; width * height * 4];
        let writer = |x, y, v: [u8; 4]| {
            let i = (x + y * (width)) * 4;
            data[i..][..4].copy_from_slice(&v);
        };
        let decoder = match self.format {
            0x1C | 0x1D => R8G8B8A8Unorm::decode_image,
            0x31 => R8G8Unorm::decode_image,
            0x3D => R8Unorm::decode_image,
            0x47 | 0x48 => Bc1Unorm::decode_image,
            0x4D | 0x4E => Bc3Unorm::decode_image,
            0x50 => Bc4Unorm::decode_image,
            0x53 => Bc5Unorm::decode_image,
            0x62 | 0x63 => Bc7Unorm::decode_image,
            0x402 | 0x403 => Astc::<4, 4>::decode_image,
            0x405 | 0x406 => Astc::<5, 4>::decode_image,
            0x408 | 0x409 => Astc::<5, 5>::decode_image,
            0x40B | 0x40C => Astc::<6, 5>::decode_image,
            0x40E | 0x40F => Astc::<6, 6>::decode_image,
            0x411 | 0x412 => Astc::<8, 5>::decode_image,
            0x414 | 0x415 => Astc::<8, 6>::decode_image,
            0x417 | 0x418 => Astc::<8, 8>::decode_image,
            0x41A | 0x41B => Astc::<10, 5>::decode_image,
            0x41D | 0x41E => Astc::<10, 6>::decode_image,
            0x420 | 0x421 => Astc::<10, 8>::decode_image,
            0x423 | 0x424 => Astc::<10, 10>::decode_image,
            0x426 | 0x427 => Astc::<12, 10>::decode_image,
            0x429 | 0x42A => Astc::<12, 12>::decode_image,
            x => bail!("unsupported format {:08X}", x),
        };
        decoder(&texture, width, height, super_width, super_height, writer);
        Ok(RgbaImage::new(
            data,
            u32::try_from(width)?,
            u32::try_from(height)?,
        ))
    }

    pub fn save_png(&self, index: usize, mipmap: usize, output: &Path) -> anyhow::Result<()> {
        self.to_rgba(index, mipmap)?.save_png(output)?;

        Ok(())
    }
}
