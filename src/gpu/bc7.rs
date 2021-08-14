const WEIGHTS2: [u32; 4] = [0, 21, 43, 64];
const WEIGHTS3: [u32; 8] = [0, 9, 18, 27, 37, 46, 55, 64];
const WEIGHTS4: [u32; 16] = [0, 4, 9, 13, 17, 21, 26, 30, 34, 38, 43, 47, 51, 55, 60, 64];

#[rustfmt::skip]
const PARTITION2: [usize; 64 * 16] = [
    0,0,1,1,0,0,1,1,0,0,1,1,0,0,1,1,        0,0,0,1,0,0,0,1,0,0,0,1,0,0,0,1,        0,1,1,1,0,1,1,1,0,1,1,1,0,1,1,1,        0,0,0,1,0,0,1,1,0,0,1,1,0,1,1,1,        0,0,0,0,0,0,0,1,0,0,0,1,0,0,1,1,        0,0,1,1,0,1,1,1,0,1,1,1,1,1,1,1,        0,0,0,1,0,0,1,1,0,1,1,1,1,1,1,1,        0,0,0,0,0,0,0,1,0,0,1,1,0,1,1,1,
    0,0,0,0,0,0,0,0,0,0,0,1,0,0,1,1,        0,0,1,1,0,1,1,1,1,1,1,1,1,1,1,1,        0,0,0,0,0,0,0,1,0,1,1,1,1,1,1,1,        0,0,0,0,0,0,0,0,0,0,0,1,0,1,1,1,        0,0,0,1,0,1,1,1,1,1,1,1,1,1,1,1,        0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,        0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,        0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,
    0,0,0,0,1,0,0,0,1,1,1,0,1,1,1,1,        0,1,1,1,0,0,0,1,0,0,0,0,0,0,0,0,        0,0,0,0,0,0,0,0,1,0,0,0,1,1,1,0,        0,1,1,1,0,0,1,1,0,0,0,1,0,0,0,0,        0,0,1,1,0,0,0,1,0,0,0,0,0,0,0,0,        0,0,0,0,1,0,0,0,1,1,0,0,1,1,1,0,        0,0,0,0,0,0,0,0,1,0,0,0,1,1,0,0,        0,1,1,1,0,0,1,1,0,0,1,1,0,0,0,1,
    0,0,1,1,0,0,0,1,0,0,0,1,0,0,0,0,        0,0,0,0,1,0,0,0,1,0,0,0,1,1,0,0,        0,1,1,0,0,1,1,0,0,1,1,0,0,1,1,0,        0,0,1,1,0,1,1,0,0,1,1,0,1,1,0,0,        0,0,0,1,0,1,1,1,1,1,1,0,1,0,0,0,        0,0,0,0,1,1,1,1,1,1,1,1,0,0,0,0,        0,1,1,1,0,0,0,1,1,0,0,0,1,1,1,0,        0,0,1,1,1,0,0,1,1,0,0,1,1,1,0,0,
    0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,        0,0,0,0,1,1,1,1,0,0,0,0,1,1,1,1,        0,1,0,1,1,0,1,0,0,1,0,1,1,0,1,0,        0,0,1,1,0,0,1,1,1,1,0,0,1,1,0,0,        0,0,1,1,1,1,0,0,0,0,1,1,1,1,0,0,        0,1,0,1,0,1,0,1,1,0,1,0,1,0,1,0,        0,1,1,0,1,0,0,1,0,1,1,0,1,0,0,1,        0,1,0,1,1,0,1,0,1,0,1,0,0,1,0,1,
    0,1,1,1,0,0,1,1,1,1,0,0,1,1,1,0,        0,0,0,1,0,0,1,1,1,1,0,0,1,0,0,0,        0,0,1,1,0,0,1,0,0,1,0,0,1,1,0,0,        0,0,1,1,1,0,1,1,1,1,0,1,1,1,0,0,        0,1,1,0,1,0,0,1,1,0,0,1,0,1,1,0,        0,0,1,1,1,1,0,0,1,1,0,0,0,0,1,1,        0,1,1,0,0,1,1,0,1,0,0,1,1,0,0,1,        0,0,0,0,0,1,1,0,0,1,1,0,0,0,0,0,
    0,1,0,0,1,1,1,0,0,1,0,0,0,0,0,0,        0,0,1,0,0,1,1,1,0,0,1,0,0,0,0,0,        0,0,0,0,0,0,1,0,0,1,1,1,0,0,1,0,        0,0,0,0,0,1,0,0,1,1,1,0,0,1,0,0,        0,1,1,0,1,1,0,0,1,0,0,1,0,0,1,1,        0,0,1,1,0,1,1,0,1,1,0,0,1,0,0,1,        0,1,1,0,0,0,1,1,1,0,0,1,1,1,0,0,        0,0,1,1,1,0,0,1,1,1,0,0,0,1,1,0,
    0,1,1,0,1,1,0,0,1,1,0,0,1,0,0,1,        0,1,1,0,0,0,1,1,0,0,1,1,1,0,0,1,        0,1,1,1,1,1,1,0,1,0,0,0,0,0,0,1,        0,0,0,1,1,0,0,0,1,1,1,0,0,1,1,1,        0,0,0,0,1,1,1,1,0,0,1,1,0,0,1,1,        0,0,1,1,0,0,1,1,1,1,1,1,0,0,0,0,        0,0,1,0,0,0,1,0,1,1,1,0,1,1,1,0,        0,1,0,0,0,1,0,0,0,1,1,1,0,1,1,1
];

#[rustfmt::skip]
const PARTITION3: [usize; 64 * 16] = [
    0,0,1,1,0,0,1,1,0,2,2,1,2,2,2,2,        0,0,0,1,0,0,1,1,2,2,1,1,2,2,2,1,        0,0,0,0,2,0,0,1,2,2,1,1,2,2,1,1,        0,2,2,2,0,0,2,2,0,0,1,1,0,1,1,1,        0,0,0,0,0,0,0,0,1,1,2,2,1,1,2,2,        0,0,1,1,0,0,1,1,0,0,2,2,0,0,2,2,        0,0,2,2,0,0,2,2,1,1,1,1,1,1,1,1,        0,0,1,1,0,0,1,1,2,2,1,1,2,2,1,1,
    0,0,0,0,0,0,0,0,1,1,1,1,2,2,2,2,        0,0,0,0,1,1,1,1,1,1,1,1,2,2,2,2,        0,0,0,0,1,1,1,1,2,2,2,2,2,2,2,2,        0,0,1,2,0,0,1,2,0,0,1,2,0,0,1,2,        0,1,1,2,0,1,1,2,0,1,1,2,0,1,1,2,        0,1,2,2,0,1,2,2,0,1,2,2,0,1,2,2,        0,0,1,1,0,1,1,2,1,1,2,2,1,2,2,2,        0,0,1,1,2,0,0,1,2,2,0,0,2,2,2,0,
    0,0,0,1,0,0,1,1,0,1,1,2,1,1,2,2,        0,1,1,1,0,0,1,1,2,0,0,1,2,2,0,0,        0,0,0,0,1,1,2,2,1,1,2,2,1,1,2,2,        0,0,2,2,0,0,2,2,0,0,2,2,1,1,1,1,        0,1,1,1,0,1,1,1,0,2,2,2,0,2,2,2,        0,0,0,1,0,0,0,1,2,2,2,1,2,2,2,1,        0,0,0,0,0,0,1,1,0,1,2,2,0,1,2,2,        0,0,0,0,1,1,0,0,2,2,1,0,2,2,1,0,
    0,1,2,2,0,1,2,2,0,0,1,1,0,0,0,0,        0,0,1,2,0,0,1,2,1,1,2,2,2,2,2,2,        0,1,1,0,1,2,2,1,1,2,2,1,0,1,1,0,        0,0,0,0,0,1,1,0,1,2,2,1,1,2,2,1,        0,0,2,2,1,1,0,2,1,1,0,2,0,0,2,2,        0,1,1,0,0,1,1,0,2,0,0,2,2,2,2,2,        0,0,1,1,0,1,2,2,0,1,2,2,0,0,1,1,        0,0,0,0,2,0,0,0,2,2,1,1,2,2,2,1,
    0,0,0,0,0,0,0,2,1,1,2,2,1,2,2,2,        0,2,2,2,0,0,2,2,0,0,1,2,0,0,1,1,        0,0,1,1,0,0,1,2,0,0,2,2,0,2,2,2,        0,1,2,0,0,1,2,0,0,1,2,0,0,1,2,0,        0,0,0,0,1,1,1,1,2,2,2,2,0,0,0,0,        0,1,2,0,1,2,0,1,2,0,1,2,0,1,2,0,        0,1,2,0,2,0,1,2,1,2,0,1,0,1,2,0,        0,0,1,1,2,2,0,0,1,1,2,2,0,0,1,1,
    0,0,1,1,1,1,2,2,2,2,0,0,0,0,1,1,        0,1,0,1,0,1,0,1,2,2,2,2,2,2,2,2,        0,0,0,0,0,0,0,0,2,1,2,1,2,1,2,1,        0,0,2,2,1,1,2,2,0,0,2,2,1,1,2,2,        0,0,2,2,0,0,1,1,0,0,2,2,0,0,1,1,        0,2,2,0,1,2,2,1,0,2,2,0,1,2,2,1,        0,1,0,1,2,2,2,2,2,2,2,2,0,1,0,1,        0,0,0,0,2,1,2,1,2,1,2,1,2,1,2,1,
    0,1,0,1,0,1,0,1,0,1,0,1,2,2,2,2,        0,2,2,2,0,1,1,1,0,2,2,2,0,1,1,1,        0,0,0,2,1,1,1,2,0,0,0,2,1,1,1,2,        0,0,0,0,2,1,1,2,2,1,1,2,2,1,1,2,        0,2,2,2,0,1,1,1,0,1,1,1,0,2,2,2,        0,0,0,2,1,1,1,2,1,1,1,2,0,0,0,2,        0,1,1,0,0,1,1,0,0,1,1,0,2,2,2,2,        0,0,0,0,0,0,0,0,2,1,1,2,2,1,1,2,
    0,1,1,0,0,1,1,0,2,2,2,2,2,2,2,2,        0,0,2,2,0,0,1,1,0,0,1,1,0,0,2,2,        0,0,2,2,1,1,2,2,1,1,2,2,0,0,2,2,        0,0,0,0,0,0,0,0,0,0,0,0,2,1,1,2,        0,0,0,2,0,0,0,1,0,0,0,2,0,0,0,1,        0,2,2,2,1,2,2,2,0,2,2,2,1,2,2,2,        0,1,0,1,2,2,2,2,2,2,2,2,2,2,2,2,        0,1,1,1,2,0,1,1,2,2,0,1,2,2,2,0,
];

#[rustfmt::skip]
const ANCHOR_SECOND: [usize; 64] = [
    15,15,15,15,15,15,15,15,        15,15,15,15,15,15,15,15,        15, 2, 8, 2, 2, 8, 8,15,        2, 8, 2, 2, 8, 8, 2, 2,        15,15, 6, 8, 2, 8,15,15,        2, 8, 2, 2, 2,15,15, 6,        6, 2, 6, 8,15,15, 2, 2,        15,15,15,15,15, 2, 2,15
];

#[rustfmt::skip]
const ANCHOR_THIRD1: [usize; 64] = [
    3, 3,15,15, 8, 3,15,15,        8, 8, 6, 6, 6, 5, 3, 3,        3, 3, 8,15, 3, 3, 6,10,        5, 8, 8, 6, 8, 5,15,15,        8,15, 3, 5, 6,10, 8,15,        15, 3,15, 5,15,15,15,15,        3,15, 5, 5, 5, 8, 5,10,        5,10, 8,13,15,12, 3, 3
];

#[rustfmt::skip]
const ANCHOR_THIRD2: [usize; 64] = [
    15, 8, 8, 3,15,15, 3, 8,        15,15,15,15,15,15,15, 8,        15, 8,15, 3,15, 8,15, 8,        3,15, 6,10,15,15,10, 8,        15, 3,15,10,10, 8, 9,10,        6,15, 8,15, 3, 6, 6, 8,        15, 3,15,15,15,15,15,15,        15,15,15,15, 3,15,15, 8
];

struct InputBitStream {
    data: u128,
    bits_read: u32,
}

impl InputBitStream {
    fn new(data: u128) -> InputBitStream {
        InputBitStream { data, bits_read: 0 }
    }

    fn get_bits_read(&self) -> u32 {
        self.bits_read
    }

    fn read_bits32(&mut self, n_bits: u32) -> u32 {
        debug_assert!(n_bits <= 32);
        self.bits_read += n_bits;
        debug_assert!(self.bits_read <= 128);
        let ret = self.data & ((1 << n_bits) - 1);
        self.data >>= n_bits;
        ret as u32
    }
}

fn bc7_dequant_pbit(val: u32, pbit: u32, val_bits: u32) -> u32 {
    debug_assert!(val < (1 << val_bits));
    debug_assert!(pbit < 2);
    debug_assert!((4..=8).contains(&val_bits));
    let total_bits = val_bits + 1;
    let mut val = (val << 1) | pbit;
    val <<= 8 - total_bits;
    val |= val >> total_bits;
    debug_assert!(val <= 255);
    val
}
fn bc7_dequant(mut val: u32, val_bits: u32) -> u32 {
    debug_assert!(val < (1 << val_bits));
    debug_assert!((4..=8).contains(&val_bits));
    val <<= 8 - val_bits;
    val |= val >> val_bits;
    debug_assert!(val <= 255);
    val
}

fn bc7_interp2(l: u32, h: u32, w: usize) -> u8 {
    ((l * (64 - WEIGHTS2[w]) + h * WEIGHTS2[w] + 32) >> 6) as u8
}
fn bc7_interp3(l: u32, h: u32, w: usize) -> u8 {
    ((l * (64 - WEIGHTS3[w]) + h * WEIGHTS3[w] + 32) >> 6) as u8
}
fn bc7_interp23(l: u32, h: u32, w: usize, bits: u32) -> u8 {
    debug_assert!(l <= 255 && h <= 255);
    match bits {
        2 => bc7_interp2(l, h, w),
        3 => bc7_interp3(l, h, w),
        _ => unreachable!(),
    }
}

fn unpack_bc7_mode0_2<F: FnMut(usize, usize, [u8; 4])>(mode: u32, block: u128, mut writer: F) {
    let weight_bits = if mode == 0 { 3 } else { 2 };
    let endpoint_bits = if mode == 0 { 4 } else { 5 };
    let pb = if mode == 0 { 6 } else { 0 };
    let weight_vals = 1 << weight_bits;

    let mut stream = InputBitStream::new(block);

    assert_eq!(stream.read_bits32(mode + 1), 1 << mode);

    let part = stream.read_bits32(if mode == 0 { 4 } else { 6 }) as usize;

    let mut endpoints = [[0; 6]; 3];

    for c in &mut endpoints {
        for e in c {
            *e = stream.read_bits32(endpoint_bits);
        }
    }

    let mut pbits = [0; 6];
    for p in &mut pbits[0..pb] {
        *p = stream.read_bits32(1);
    }

    let mut weights = [0; 16];
    for (i, w) in weights.iter_mut().enumerate() {
        let weight_bits = if i == 0 || i == ANCHOR_THIRD1[part] || i == ANCHOR_THIRD2[part] {
            weight_bits - 1
        } else {
            weight_bits
        };
        *w = stream.read_bits32(weight_bits);
    }

    debug_assert!(stream.get_bits_read() == 128);

    for c in &mut endpoints {
        for (e, p) in c.iter_mut().zip(pbits) {
            *e = if pb != 0 {
                bc7_dequant_pbit(*e, p, endpoint_bits)
            } else {
                bc7_dequant(*e, endpoint_bits)
            };
        }
    }

    let mut block_colors = [[[0, 0, 0, 255]; 8]; 3];
    for (s, se) in block_colors.iter_mut().enumerate() {
        for (i, see) in se[0..weight_vals].iter_mut().enumerate() {
            for (color, e) in see[0..3].iter_mut().zip(endpoints) {
                *color = bc7_interp23(e[s * 2], e[s * 2 + 1], i, weight_bits);
            }
        }
    }

    for y in 0..4 {
        for x in 0..4 {
            let i = x + y * 4;
            writer(
                x,
                y,
                block_colors[PARTITION3[part * 16 + i]][weights[i] as usize],
            )
        }
    }
}

fn unpack_bc7_mode1_3_7<F: FnMut(usize, usize, [u8; 4])>(mode: u32, block: u128, mut writer: F) {
    let comps = if mode == 7 { 4 } else { 3 };
    let weight_bits = if mode == 1 { 3 } else { 2 };
    let endpoint_bits = match mode {
        7 => 5,
        1 => 6,
        3 => 7,
        _ => unreachable!(),
    };
    let pb = if mode == 1 { 2 } else { 4 };
    let shared_pbits = mode == 1;
    let weight_vals = 1 << weight_bits;

    let mut stream = InputBitStream::new(block);

    assert_eq!(stream.read_bits32(mode + 1), 1 << mode);

    let part = stream.read_bits32(6) as usize;

    let mut endpoints = [[0; 4]; 4];
    for c in &mut endpoints[0..comps] {
        for e in c {
            *e = stream.read_bits32(endpoint_bits);
        }
    }

    let mut pbits = [0; 4];
    for p in &mut pbits[0..pb] {
        *p = stream.read_bits32(1);
    }

    let mut weights = [0; 16];
    for (i, w) in weights.iter_mut().enumerate() {
        let weight_bits = if i == 0 || i == ANCHOR_SECOND[part] {
            weight_bits - 1
        } else {
            weight_bits
        };
        *w = stream.read_bits32(weight_bits);
    }

    debug_assert!(stream.get_bits_read() == 128);

    for c in &mut endpoints[0..comps] {
        for (e, ep) in c.iter_mut().enumerate() {
            *ep = bc7_dequant_pbit(
                *ep,
                pbits[if shared_pbits { e >> 1 } else { e }],
                endpoint_bits,
            );
        }
    }

    let mut block_colors = [[[0, 0, 0, 255]; 8]; 2];
    for (s, se) in block_colors.iter_mut().enumerate() {
        for (i, see) in se[0..weight_vals].iter_mut().enumerate() {
            for (color, e) in see[0..comps].iter_mut().zip(endpoints) {
                *color = bc7_interp23(e[s * 2], e[s * 2 + 1], i, weight_bits);
            }
        }
    }

    for y in 0..4 {
        for x in 0..4 {
            let i = x + y * 4;
            writer(
                x,
                y,
                block_colors[PARTITION2[part * 16 + i]][weights[i] as usize],
            )
        }
    }
}

fn unpack_bc7_mode4_5<F: FnMut(usize, usize, [u8; 4])>(mode: u32, block: u128, mut writer: F) {
    let weight_bits = 2;
    let a_weight_bits = if mode == 4 { 3 } else { 2 };
    let endpoint_bits = if mode == 4 { 5 } else { 7 };
    let a_endpoint_bits = if mode == 4 { 6 } else { 8 };

    let mut stream = InputBitStream::new(block);

    assert_eq!(stream.read_bits32(mode + 1), 1 << mode);

    let comp_rot = stream.read_bits32(2);
    let index_mode = if mode == 4 { stream.read_bits32(1) } else { 0 };

    let mut endpoints = [[0; 2]; 4];
    for (c, cc) in endpoints.iter_mut().enumerate() {
        for e in cc {
            *e = stream.read_bits32(if c == 3 {
                a_endpoint_bits
            } else {
                endpoint_bits
            });
        }
    }
    let weights_bits = if index_mode != 0 {
        [a_weight_bits, weight_bits]
    } else {
        [weight_bits, a_weight_bits]
    };

    let mut weights = [0; 16];
    let mut a_weights = [0; 16];
    let (first, second) = if index_mode != 0 {
        (&mut a_weights, &mut weights)
    } else {
        (&mut weights, &mut a_weights)
    };

    for (i, w) in first.iter_mut().enumerate() {
        let bit_decrease = if i == 0 { 1 } else { 0 };
        *w = stream.read_bits32(weight_bits - bit_decrease);
    }

    for (i, w) in second.iter_mut().enumerate() {
        let bit_decrease = if i == 0 { 1 } else { 0 };
        *w = stream.read_bits32(a_weight_bits - bit_decrease);
    }

    debug_assert!(stream.get_bits_read() == 128);

    for (c, cc) in endpoints.iter_mut().enumerate() {
        for e in cc {
            *e = bc7_dequant(
                *e,
                if c == 3 {
                    a_endpoint_bits
                } else {
                    endpoint_bits
                },
            );
        }
    }

    let mut block_colors = [[0; 4]; 8];
    for (i, b) in block_colors[0..1 << weights_bits[0]].iter_mut().enumerate() {
        for (color, e) in b.iter_mut().zip(endpoints) {
            *color = bc7_interp23(e[0], e[1], i, weights_bits[0]);
        }
    }

    for (i, b) in block_colors[0..1 << weights_bits[1]].iter_mut().enumerate() {
        b[3] = bc7_interp23(endpoints[3][0], endpoints[3][1], i, weights_bits[1]);
    }

    for y in 0..4 {
        for x in 0..4 {
            let i = x + y * 4;
            let mut color = block_colors[weights[i] as usize];
            color[3] = block_colors[a_weights[i] as usize][3];

            if comp_rot >= 1 {
                color.swap(3, (comp_rot - 1) as usize);
            }
            writer(x, y, color)
        }
    }
}

fn unpack_bc7_mode6<F: FnMut(usize, usize, [u8; 4])>(block: u128, mut writer: F) {
    let mut stream = InputBitStream::new(block);

    assert_eq!(stream.read_bits32(7), 1 << 6);

    let r0 = stream.read_bits32(7);
    let r1 = stream.read_bits32(7);
    let g0 = stream.read_bits32(7);
    let g1 = stream.read_bits32(7);
    let b0 = stream.read_bits32(7);
    let b1 = stream.read_bits32(7);
    let a0 = stream.read_bits32(7);
    let a1 = stream.read_bits32(7);
    let p0 = stream.read_bits32(1);

    let p1 = stream.read_bits32(1);

    let mut s = [0; 16];
    for (i, w) in s.iter_mut().enumerate() {
        let bits = if i == 0 { 3 } else { 4 };
        *w = stream.read_bits32(bits);
    }

    let r0 = (r0 << 1) | p0;
    let g0 = (g0 << 1) | p0;
    let b0 = (b0 << 1) | p0;
    let a0 = (a0 << 1) | p0;
    let r1 = (r1 << 1) | p1;
    let g1 = (g1 << 1) | p1;
    let b1 = (b1 << 1) | p1;
    let a1 = (a1 << 1) | p1;

    let mut vals = [[0; 4]; 16];
    for (val, w) in vals.iter_mut().zip(WEIGHTS4) {
        let iw = 64 - w;
        *val = [
            ((r0 * iw + r1 * w + 32) >> 6) as u8,
            ((g0 * iw + g1 * w + 32) >> 6) as u8,
            ((b0 * iw + b1 * w + 32) >> 6) as u8,
            ((a0 * iw + a1 * w + 32) >> 6) as u8,
        ];
    }

    for y in 0..4 {
        for x in 0..4 {
            let i = x + y * 4;
            writer(x, y, vals[s[i] as usize])
        }
    }
}

pub fn bc7_decompress_block<F: FnMut(usize, usize, [u8; 4])>(
    in_buf: &[u8; 16],
    mut writer: F,
) -> bool {
    let first_byte = in_buf[0];
    let block = u128::from_le_bytes(*in_buf);

    for mode in 0..=7 {
        if first_byte & (1 << mode) != 0 {
            match mode {
                0 | 2 => unpack_bc7_mode0_2(mode, block, writer),
                1 | 3 | 7 => unpack_bc7_mode1_3_7(mode, block, writer),
                4 | 5 => unpack_bc7_mode4_5(mode, block, writer),
                6 => unpack_bc7_mode6(block, writer),
                _ => unreachable!(),
            }
            return true;
        }
    }

    for y in 0..4 {
        for x in 0..4 {
            writer(x, y, [0xFF, 0, 0xFF, 0xFF])
        }
    }
    false
}
