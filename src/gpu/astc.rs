// Ported from https://github.com/yuzu-emu/yuzu/blob/master/src/video_core/textures/astc.cpp
// Buggy

use once_cell::sync::Lazy;
use std::convert::TryInto;
use std::ops::Index;

struct InputBitStream<'a> {
    cur_byte: &'a [u8],
    total_bits: usize,
    next_bit: usize,
    bits_read: usize,
}

impl<'a> InputBitStream<'a> {
    fn new(data: &'a [u8], start_offset: usize) -> InputBitStream<'a> {
        InputBitStream {
            cur_byte: data,
            total_bits: data.len(),
            next_bit: start_offset % 8,
            bits_read: 0,
        }
    }

    fn get_bits_read(&self) -> usize {
        self.bits_read
    }

    fn read_bit(&mut self) -> bool {
        if self.bits_read >= self.total_bits * 8 {
            return false;
        }
        let bit = (self.cur_byte[0] >> self.next_bit) & 1 != 0;
        self.next_bit += 1;
        while self.next_bit >= 8 {
            self.next_bit -= 8;
            self.cur_byte = &self.cur_byte[1..];
        }
        self.bits_read += 1;
        bit
    }

    fn read_bits(&mut self, n_bits: u32) -> u32 {
        let mut ret = 0;
        for i in 0..n_bits {
            ret |= (self.read_bit() as u32) << i;
        }
        ret
    }
}

struct OutputBitStream<'a> {
    cur_byte: usize,
    ptr: &'a mut [u8],
    num_bits: usize,
    bits_written: usize,
    next_bit: usize,
}

impl<'a> OutputBitStream<'a> {
    fn new(ptr: &'a mut [u8]) -> OutputBitStream<'a> {
        let num_bits = ptr.len() * 8;
        OutputBitStream {
            cur_byte: 0,
            ptr,
            num_bits,
            bits_written: 0,
            next_bit: 0,
        }
    }

    fn write_bits(&mut self, val: u32, n_bits: u32) {
        for i in 0..n_bits {
            self.write_bit((val >> i) & 1 != 0);
        }
    }

    fn write_bit(&mut self, b: bool) {
        if self.bits_written >= self.num_bits {
            return;
        }

        let mask = 1 << self.next_bit;
        self.next_bit += 1;

        if b {
            self.ptr[self.cur_byte] |= mask
        } else {
            self.ptr[self.cur_byte] &= !mask;
        }

        // Next byte?
        if self.next_bit >= 8 {
            self.cur_byte += 1;
            self.next_bit = 0;
        }
    }
}

struct Bits(u32);

impl Bits {
    fn get(&self, pos: u32) -> u32 {
        (self.0 >> pos) & 1
    }

    fn range(&self, start: u32, end: u32) -> u32 {
        let mask = (1 << (end - start + 1)) - 1;
        (self.0 >> start) & mask
    }
}

#[derive(Debug, Clone, Copy)]
enum IntegerEncoding {
    JustBits,
    Qus32(u32),
    Trit(u32),
}

#[derive(Debug, Clone, Copy)]
struct IntegerEncodedValue {
    encoding: IntegerEncoding,
    num_bits: u32,
    bit_value: u32,
}

impl IntegerEncodedValue {
    // Returns the number of bits required to encode nVals values.
    fn get_bit_length(&self, n_vals: u32) -> u32 {
        let mut total_bits = self.num_bits * n_vals;
        match self.encoding {
            IntegerEncoding::JustBits => (),
            IntegerEncoding::Trit(_) => total_bits += (n_vals * 8 + 4) / 5,
            IntegerEncoding::Qus32(_) => total_bits += (n_vals * 7 + 2) / 3,
        }
        total_bits
    }
    fn match_encoding(&self, other: &IntegerEncodedValue) -> bool {
        use std::mem::discriminant;

        discriminant(&self.encoding) == discriminant(&other.encoding)
            && self.num_bits == other.num_bits
    }
}

#[allow(non_snake_case)]
fn decode_trit_block(
    bits: &mut InputBitStream,
    result: &mut Vec<IntegerEncodedValue>,
    bits_per_value: u32,
) {
    // Implement the algorithm in section C.2.12
    let mut m = [0u32; 5];
    let mut t = [0u32; 5];
    let mut T: u32;

    // Read the trit encoded block according to
    // table C.2.14
    m[0] = bits.read_bits(bits_per_value);
    T = bits.read_bits(2);
    m[1] = bits.read_bits(bits_per_value);
    T |= bits.read_bits(2) << 2;
    m[2] = bits.read_bits(bits_per_value);
    T |= (bits.read_bit() as u32) << 4;
    m[3] = bits.read_bits(bits_per_value);
    T |= bits.read_bits(2) << 5;
    m[4] = bits.read_bits(bits_per_value);
    T |= (bits.read_bit() as u32) << 7;

    let C: u32;

    let Tb = Bits(T);
    if Tb.range(2, 4) == 7 {
        C = (Tb.range(5, 7) << 2) | Tb.range(0, 1);
        t[3] = 2;
        t[4] = 2;
    } else {
        C = Tb.range(0, 4);
        if Tb.range(5, 6) == 3 {
            t[4] = 2;
            t[3] = Tb.get(7);
        } else {
            t[4] = Tb.get(7);
            t[3] = Tb.range(5, 6);
        }
    }

    let Cb = Bits(C);
    if Cb.range(0, 1) == 3 {
        t[2] = 2;
        t[1] = Cb.get(4);
        t[0] = (Cb.get(3) << 1) | (Cb.get(2) & !Cb.get(3));
    } else if Cb.range(2, 3) == 3 {
        t[2] = 2;
        t[1] = 2;
        t[0] = Cb.range(0, 1);
    } else {
        t[2] = Cb.get(4);
        t[1] = Cb.range(2, 3);
        t[0] = (Cb.get(1) << 1) | (Cb.get(0) & !Cb.get(1));
    }

    for i in 0..5 {
        let bit_value = m[i];
        let trit_value = t[i];
        result.push(IntegerEncodedValue {
            encoding: IntegerEncoding::Trit(trit_value),
            bit_value,
            num_bits: bits_per_value,
        })
    }
}

#[allow(non_snake_case)]
fn decode_qus32_block(
    bits: &mut InputBitStream,
    result: &mut Vec<IntegerEncodedValue>,
    bits_per_value: u32,
) {
    // Implement the algorithm in section C.2.12
    let mut m = [0u32; 3];
    let mut q = [0u32; 3];
    let mut Q: u32;

    // Read the trit encoded block according to
    // table C.2.15
    m[0] = bits.read_bits(bits_per_value);
    Q = bits.read_bits(3);
    m[1] = bits.read_bits(bits_per_value);
    Q |= bits.read_bits(2) << 3;
    m[2] = bits.read_bits(bits_per_value);
    Q |= bits.read_bits(2) << 5;

    let Qb = Bits(Q);
    if Qb.range(1, 2) == 3 && Qb.range(5, 6) == 0 {
        q[0] = 4;
        q[1] = 4;
        q[2] = (Qb.get(0) << 2) | ((Qb.get(4) & !Qb.get(0)) << 1) | (Qb.get(3) & !Qb.get(0));
    } else {
        let C;
        if Qb.range(1, 2) == 3 {
            q[2] = 4;
            C = (Qb.range(3, 4) << 3) | ((!Qb.range(5, 6) & 3) << 1) | Qb.get(0);
        } else {
            q[2] = Qb.range(5, 6);
            C = Qb.range(0, 4);
        }

        let Cb = Bits(C);
        if Cb.range(0, 2) == 5 {
            q[1] = 4;
            q[0] = Cb.range(3, 4);
        } else {
            q[1] = Cb.range(3, 4);
            q[0] = Cb.range(0, 2);
        }
    }

    for i in 0..3 {
        let bit_value = m[i];
        let qus32_value = q[i];
        result.push(IntegerEncodedValue {
            encoding: IntegerEncoding::Qus32(qus32_value),
            bit_value,
            num_bits: bits_per_value,
        })
    }
}

// Returns a new instance of this struct that corresponds to the
// can take no more than maxval values
fn create_encoding(mut max_val: u32) -> IntegerEncodedValue {
    while max_val > 0 {
        let check = max_val + 1;

        // Is max_val a power of two?
        if (check & (check - 1)) == 0 {
            return IntegerEncodedValue {
                encoding: IntegerEncoding::JustBits,
                num_bits: max_val.count_ones(),
                bit_value: 0,
            };
        }

        // Is max_val of the type 3*2^n - 1?
        if (check % 3 == 0) && ((check / 3) & ((check / 3) - 1)) == 0 {
            return IntegerEncodedValue {
                encoding: IntegerEncoding::Trit(0),
                num_bits: (check / 3 - 1).count_ones(),
                bit_value: 0,
            };
        }

        // Is max_val of the type 5*2^n - 1?
        if (check % 5 == 0) && !((check / 5) & ((check / 5) - 1)) == 0 {
            return IntegerEncodedValue {
                encoding: IntegerEncoding::Qus32(0),
                num_bits: (check / 5 - 1).count_ones(),
                bit_value: 0,
            };
        }

        // Apparently it can't be represented with a bounded integer sequence...
        // just iterate.
        max_val -= 1;
    }
    IntegerEncodedValue {
        encoding: IntegerEncoding::JustBits,
        num_bits: 0,
        bit_value: 0,
    }
}

static ENCODINGS_VALUES: Lazy<[IntegerEncodedValue; 256]> = Lazy::new(|| {
    (0..256)
        .map(|i| create_encoding(i))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
});

// Fills result with the values that are encoded in the given
// bitstream. We must know beforehand what the maximum possible
// value is, and how many values we're decoding.
fn decode_integer_sequence(
    result: &mut Vec<IntegerEncodedValue>,
    bits: &mut InputBitStream,
    max_range: u32,
    n_values: u32,
) {
    // Determine encoding parameters
    let mut val = ENCODINGS_VALUES[max_range as usize];

    // Start decoding
    let mut n_vals_decoded = 0;
    while n_vals_decoded < n_values {
        match val.encoding {
            IntegerEncoding::Qus32(_) => {
                decode_qus32_block(bits, result, val.num_bits);
                n_vals_decoded += 3;
            }

            IntegerEncoding::Trit(_) => {
                decode_trit_block(bits, result, val.num_bits);
                n_vals_decoded += 5;
            }
            IntegerEncoding::JustBits => {
                val.bit_value = bits.read_bits(val.num_bits);
                result.push(val);
                n_vals_decoded += 1;
            }
        }
    }
}

struct TexelWeightParams {
    width: u32,
    height: u32,
    is_dual_plane: bool,
    max_weight: u32,
    is_error: bool,
    void_extent_ldr: bool,
    void_extent_hdr: bool,
}

impl Default for TexelWeightParams {
    fn default() -> Self {
        TexelWeightParams {
            width: 0,
            height: 0,
            is_dual_plane: false,
            max_weight: 0,
            is_error: false,
            void_extent_ldr: false,
            void_extent_hdr: false,
        }
    }
}

impl TexelWeightParams {
    fn get_packed_bit_size(&self) -> u32 {
        // How many indices do we have?
        let mut nidxs = self.height * self.width;
        if self.is_dual_plane {
            nidxs *= 2;
        }

        ENCODINGS_VALUES[self.max_weight as usize].get_bit_length(nidxs)
    }

    fn get_num_weight_values(&self) -> u32 {
        let mut ret = self.width * self.height;
        if self.is_dual_plane {
            ret *= 2;
        }
        ret
    }
}

#[allow(non_snake_case)]
fn decode_block_info(strm: &mut InputBitStream) -> TexelWeightParams {
    let mut params = TexelWeightParams::default();

    // Read the entire block mode all at once
    let mode_bits = strm.read_bits(11) as u16;

    // Does this match the void extent block mode?
    if (mode_bits & 0x01FF) == 0x1FC {
        if mode_bits & 0x200 != 0 {
            params.void_extent_hdr = true;
        } else {
            params.void_extent_ldr = true;
        }

        // Next two bits must be one.
        if (mode_bits & 0x400) == 0 || !strm.read_bit() {
            params.is_error = true;
        }

        return params;
    }

    // First check if the last four bits are zero
    if (mode_bits & 0xF) == 0 {
        params.is_error = true;
        return params;
    }

    // If the last two bits are zero, then if bits
    // [6-8] are all ones, this is also reserved.
    if (mode_bits & 0x3) == 0 && (mode_bits & 0x1C0) == 0x1C0 {
        params.is_error = true;
        return params;
    }

    // Otherwise, there is no error... Figure out the layout
    // of the block mode. Layout is determined by a number
    // between 0 and 9 corresponding to table C.2.8 of the
    // ASTC spec.
    let layout;

    if (mode_bits & 0x1) != 0 || (mode_bits & 0x2) != 0 {
        // layout is in [0-4]
        if (mode_bits & 0x8) != 0 {
            // layout is in [2-4]
            if (mode_bits & 0x4) != 0 {
                // layout is in [3-4]
                if (mode_bits & 0x100) != 0 {
                    layout = 4;
                } else {
                    layout = 3;
                }
            } else {
                layout = 2;
            }
        } else {
            // layout is in [0-1]
            if (mode_bits & 0x4) != 0 {
                layout = 1;
            } else {
                layout = 0;
            }
        }
    } else {
        // layout is in [5-9]
        if (mode_bits & 0x100) != 0 {
            // layout is in [7-9]
            if (mode_bits & 0x80) != 0 {
                // layout is in [7-8]
                assert!((mode_bits & 0x40) == 0);
                if (mode_bits & 0x20) != 0 {
                    layout = 8;
                } else {
                    layout = 7;
                }
            } else {
                layout = 9;
            }
        } else {
            // layout is in [5-6]
            if (mode_bits & 0x80) != 0 {
                layout = 6;
            } else {
                layout = 5;
            }
        }
    }

    assert!(layout < 10);

    // Determine R
    let mut R = (mode_bits & 0x10) >> 4;
    if layout < 5 {
        R |= (mode_bits & 0x3) << 1;
    } else {
        R |= (mode_bits & 0xC) >> 1;
    }
    assert!(2 <= R && R <= 7);

    // Determine width & height
    match layout {
        0 => {
            let A = (mode_bits >> 5) & 0x3;
            let B = (mode_bits >> 7) & 0x3;
            params.width = B as u32 + 4;
            params.height = A as u32 + 2;
        }

        1 => {
            let A = (mode_bits >> 5) & 0x3;
            let B = (mode_bits >> 7) & 0x3;
            params.width = B as u32 + 8;
            params.height = A as u32 + 2;
        }

        2 => {
            let A = (mode_bits >> 5) & 0x3;
            let B = (mode_bits >> 7) & 0x3;
            params.width = A as u32 + 2;
            params.height = B as u32 + 8;
        }

        3 => {
            let A = (mode_bits >> 5) & 0x3;
            let B = (mode_bits >> 7) & 0x1;
            params.width = A as u32 + 2;
            params.height = B as u32 + 6;
        }

        4 => {
            let A = (mode_bits >> 5) & 0x3;
            let B = (mode_bits >> 7) & 0x1;
            params.width = B as u32 + 2;
            params.height = A as u32 + 2;
        }

        5 => {
            let A = (mode_bits >> 5) & 0x3;
            params.width = 12;
            params.height = A as u32 + 2;
        }

        6 => {
            let A = (mode_bits >> 5) & 0x3;
            params.width = A as u32 + 2;
            params.height = 12;
        }

        7 => {
            params.width = 6;
            params.height = 10;
        }

        8 => {
            params.width = 10;
            params.height = 6;
        }

        9 => {
            let A = (mode_bits >> 5) & 0x3;
            let B = (mode_bits >> 9) & 0x3;
            params.width = A as u32 + 6;
            params.height = B as u32 + 6;
        }

        _ => panic!("Don't know this layout..."),
    }

    // Determine whether or not we're using dual planes
    // and/or high precision layouts.
    let D = (layout != 9) && (mode_bits & 0x400) != 0;
    let H = (layout != 9) && (mode_bits & 0x200) != 0;

    if H {
        const MAX_WEIGHTS: [u32; 6] = [9, 11, 15, 19, 23, 31];
        params.max_weight = MAX_WEIGHTS[(R - 2) as usize];
    } else {
        const MAX_WEIGHTS: [u32; 6] = [1, 2, 3, 4, 5, 7];
        params.max_weight = MAX_WEIGHTS[(R - 2) as usize];
    }

    params.is_dual_plane = D;

    params
}

fn fill_void_extent_ldr<F: FnMut(usize, usize, (u8, u8, u8, u8))>(
    strm: &mut InputBitStream,
    writer: &mut F,
    block_width: u32,
    block_height: u32,
) {
    // Don't actually care about the void extent, just read the bits...
    for _ in 0..4 {
        strm.read_bits(13);
    }

    // Decode the RGBA components and renormalize them to the range [0, 255]
    let r = strm.read_bits(16) >> 8;
    let g = strm.read_bits(16) >> 8;
    let b = strm.read_bits(16) >> 8;
    let a = strm.read_bits(16) >> 8;

    for j in 0..block_height {
        for i in 0..block_width {
            writer(i as usize, j as usize, (r as u8, g as u8, b as u8, a as u8));
        }
    }
}

fn fill_error<F: FnMut(usize, usize, (u8, u8, u8, u8))>(
    writer: &mut F,
    block_width: u32,
    block_height: u32,
) {
    for j in 0..block_height {
        for i in 0..block_width {
            writer(i as usize, j as usize, (0xFF, 0, 0xFF, 0xFF));
        }
    }
}

// Replicates low num_bits such that [(to_bit - 1):(to_bit - 1 - fromBit)]
// is the same as [(num_bits - 1):0] and repeats all the way down.
fn replicate(val: u32, mut num_bits: u32, to_bit: u32) -> u32 {
    if num_bits == 0 {
        return 0;
    }
    if to_bit == 0 {
        return 0;
    }
    let v = val & ((1 << num_bits) - 1);
    let mut res = v;
    let mut reslen = num_bits;
    while reslen < to_bit {
        let mut comp = 0;
        if num_bits > to_bit - reslen {
            let newshift = to_bit - reslen;
            comp = num_bits - newshift;
            num_bits = newshift;
        }
        res = res << num_bits;
        res = res | (v >> comp);
        reslen += num_bits;
    }
    res
}

#[derive(Clone, Copy)]
struct Pixel {
    bit_depth: [u8; 4],
    color: [i16; 4],
}

impl Default for Pixel {
    fn default() -> Self {
        Pixel {
            bit_depth: [8, 8, 8, 8],
            color: [0, 0, 0, 0],
        }
    }
}

impl Pixel {
    fn new(a: i16, r: i16, g: i16, b: i16 /*, bit_depth: u8*/) -> Pixel {
        Pixel {
            color: [a, r, g, b],
            bit_depth: [8, 8, 8, 8],
        }
    }

    // Changes the depth of each pixel. This scales the values to
    // the appropriate bit depth by either truncating the least
    // significant bits when going from larger to smaller bit depth
    // or by repeating the most significant bits when going from
    // smaller to larger bit depths.
    fn change_bit_depth(&mut self) {
        for i in 0..4 {
            *self.component_mut(i) =
                Self::change_bit_depth_comp(self.component(i), self.bit_depth[i as usize]);
            self.bit_depth[i as usize] = 8;
        }
    }

    fn convert_channel_to_float(channel: i16, bit_depth: u8) -> f32 {
        let denominator = ((1 << bit_depth) - 1) as f32;
        return channel as f32 / denominator;
    }

    // Changes the bit depth of a single component. See the comment
    // above for how we do this.
    fn change_bit_depth_comp(val: i16, old_depth: u8) -> i16 {
        assert!(old_depth <= 8);

        if old_depth == 8 {
            // Do nothing
            val
        } else if old_depth == 0 {
            (1 << 8) - 1
        } else if 8 > old_depth {
            replicate(val as u32, old_depth as u32, 8) as i16
        } else {
            // old_depth > newDepth
            let bits_wasted = old_depth - 8;
            let mut v = val as u16;
            v = (v + (1 << (bits_wasted - 1))) >> bits_wasted;
            v = std::cmp::min(v, (1 << 8) - 1);
            v as i16
        }
    }

    fn a_mut(&mut self) -> &mut i16 {
        &mut self.color[0]
    }
    fn a(&self) -> i16 {
        self.color[0]
    }
    fn r_mut(&mut self) -> &mut i16 {
        &mut self.color[1]
    }
    fn r(&self) -> i16 {
        self.color[1]
    }
    fn g_mut(&mut self) -> &mut i16 {
        &mut self.color[2]
    }
    fn g(&self) -> i16 {
        self.color[2]
    }
    fn b_mut(&mut self) -> &mut i16 {
        &mut self.color[3]
    }
    fn b(&self) -> i16 {
        self.color[3]
    }
    fn component_mut(&mut self, idx: u32) -> &mut i16 {
        &mut self.color[idx as usize]
    }
    fn component(&self, idx: u32) -> i16 {
        self.color[idx as usize]
    }

    // Take all of the components, transform them to their 8-bit variants
    fn pack(&self) -> (u8, u8, u8, u8) {
        let mut eight_bit = self.clone();
        eight_bit.change_bit_depth();

        (
            eight_bit.r() as u8,
            eight_bit.g() as u8,
            eight_bit.b() as u8,
            eight_bit.a() as u8,
        )
    }

    // Clamps the pixel to the range [0,255]
    fn clamp_byte(&mut self) {
        for i in 0..4 {
            self.color[i] = self.color[i].clamp(0, 255);
        }
    }

    fn make_opaque(&mut self) {
        *self.a_mut() = 255;
    }
}

#[allow(non_snake_case)]
fn decode_color_values(
    out: &mut [u32],
    data: &[u8],
    modes: &[u32],
    n_partitions: u32,
    n_bits_for_color_data: u32,
) {
    // First figure out how many color values we have
    let mut n_values = 0;
    for i in 0..n_partitions {
        n_values += ((modes[i as usize] >> 2) + 1) << 1;
    }

    // Then based on the number of values and the remaining number of bits,
    // figure out the max value for each of them...
    let mut range = 256;
    range -= 1;
    while range > 0 {
        let val = ENCODINGS_VALUES[range];
        let bit_length = val.get_bit_length(n_values);
        if bit_length <= n_bits_for_color_data {
            // Find the smallest possible range that matches the given encoding
            range -= 1;
            while range > 0 {
                let newval = ENCODINGS_VALUES[range];
                if !newval.match_encoding(&val) {
                    break;
                }
                range -= 1;
            }

            // Return to last matching range.
            range += 1;
            break;
        }
        range -= 1;
    }

    // We now have enough to decode our integer sequence.
    let mut decoded_color_values = vec![];

    let mut color_stream = InputBitStream::new(data, 0);
    decode_integer_sequence(
        &mut decoded_color_values,
        &mut color_stream,
        range as u32,
        n_values,
    );

    // Once we have the decoded values, we need to dequantize them to the 0-255 range
    // This procedure is outlined in ASTC spec C.2.13
    let mut old_idx = 0;
    for val in decoded_color_values {
        // Have we already decoded all that we need?
        if old_idx >= n_values {
            break;
        }

        let bitlen = val.num_bits;
        let bitval = val.bit_value;

        assert!(bitlen >= 1);

        // A is just the lsb replicated 9 times.
        let A = replicate(bitval & 1, 1, 9);
        let mut B = 0;
        let mut C = 0;
        let mut D = 0;

        match val.encoding {
            // replicate bits
            IntegerEncoding::JustBits => {
                out[old_idx as usize] = replicate(bitval, bitlen, 8);
                old_idx += 1;
            }

            // Use algorithm in C.2.13
            IntegerEncoding::Trit(trit_value) => {
                D = trit_value;

                match bitlen {
                    1 => {
                        C = 204;
                    }

                    2 => {
                        C = 93;
                        // B = b000b0bb0
                        let b = (bitval >> 1) & 1;
                        B = (b << 8) | (b << 4) | (b << 2) | (b << 1);
                    }

                    3 => {
                        C = 44;
                        // B = cb000cbcb
                        let cb = (bitval >> 1) & 3;
                        B = (cb << 7) | (cb << 2) | cb;
                    }

                    4 => {
                        C = 22;
                        // B = dcb000dcb
                        let dcb = (bitval >> 1) & 7;
                        B = (dcb << 6) | dcb;
                    }

                    5 => {
                        C = 11;
                        // B = edcb000ed
                        let edcb = (bitval >> 1) & 0xF;
                        B = (edcb << 5) | (edcb >> 2);
                    }

                    6 => {
                        C = 5;
                        // B = fedcb000f
                        let fedcb = (bitval >> 1) & 0x1F;
                        B = (fedcb << 4) | (fedcb >> 4);
                    }

                    _ => panic!("Unsupported trit encoding for color values!"),
                }
            }

            IntegerEncoding::Qus32(qus32_value) => {
                D = qus32_value;

                match bitlen {
                    1 => {
                        C = 113;
                    }

                    2 => {
                        C = 54;
                        // B = b0000bb00
                        let b = (bitval >> 1) & 1;
                        B = (b << 8) | (b << 3) | (b << 2);
                    }

                    3 => {
                        C = 26;
                        // B = cb0000cbc
                        let cb = (bitval >> 1) & 3;
                        B = (cb << 7) | (cb << 1) | (cb >> 1);
                    }

                    4 => {
                        C = 13;
                        // B = dcb0000dc
                        let dcb = (bitval >> 1) & 7;
                        B = (dcb << 6) | (dcb >> 1);
                    }

                    5 => {
                        C = 6;
                        // B = edcb0000e
                        let edcb = (bitval >> 1) & 0xF;
                        B = (edcb << 5) | (edcb >> 3);
                    }

                    _ => panic!("Unsupported quint encoding for color values!"),
                }
            }
        } // switch(val.encoding)

        if !matches!(val.encoding, IntegerEncoding::JustBits) {
            let mut T = D * C + B;
            T ^= A;
            T = (A & 0x80) | (T >> 2);
            out[old_idx as usize] = T;
            old_idx += 1;
        }
    }

    // Make sure that each of our values is in the proper range...
    for i in 0..n_values {
        assert!(out[i as usize] <= 255);
    }
}

#[allow(non_snake_case)]
fn unquantize_texel_weight(val: &IntegerEncodedValue) -> u32 {
    let bitval = val.bit_value;
    let bitlen = val.num_bits;

    let A = replicate(bitval & 1, 1, 7);
    let mut B = 0;
    let mut C = 0;
    let mut D = 0;

    let mut result = 0;
    match val.encoding {
        IntegerEncoding::JustBits => {
            result = replicate(bitval, bitlen, 6);
        }

        IntegerEncoding::Trit(trit_value) => {
            D = trit_value;
            assert!(D < 3);

            match bitlen {
                0 => {
                    result = [0, 32, 63][D as usize];
                }

                1 => {
                    C = 50;
                }

                2 => {
                    C = 23;
                    let b = (bitval >> 1) & 1;
                    B = (b << 6) | (b << 2) | b;
                }

                3 => {
                    C = 11;
                    let cb = (bitval >> 1) & 3;
                    B = (cb << 5) | cb;
                }

                _ => panic!("Invalid trit encoding for texel weight"),
            }
        }

        IntegerEncoding::Qus32(qus32_value) => {
            D = qus32_value;
            assert!(D < 5);

            match bitlen {
                0 => {
                    result = [0, 16, 32, 47, 63][D as usize];
                }

                1 => {
                    C = 28;
                }

                2 => {
                    C = 13;
                    let b = (bitval >> 1) & 1;
                    B = (b << 6) | (b << 1);
                }

                _ => panic!("Invalid quint encoding for texel weight"),
            }
        }
    }

    if !matches!(val.encoding, IntegerEncoding::JustBits) && bitlen > 0 {
        // Decode the value...
        result = D * C + B;
        result ^= A;
        result = (A & 0x20) | (result >> 2);
    }

    assert!(result < 64);

    // Change from [0,63] to [0,64]
    if result > 32 {
        result += 1;
    }

    result
}

fn unquantize_texel_weights(
    out: &mut [[u32; 144]; 2],
    weights: &[IntegerEncodedValue],
    params: &TexelWeightParams,
    block_width: u32,
    block_height: u32,
) {
    let mut weight_idx = 0u32;
    let mut unquantized = [[0; 144]; 2];

    let mut itr = weights.iter();
    while let Some(w) = itr.next() {
        unquantized[0][weight_idx as usize] = unquantize_texel_weight(w);

        if params.is_dual_plane {
            unquantized[1][weight_idx as usize] = unquantize_texel_weight(itr.next().unwrap());
            /*if itr == weights.end() {
                break;
            }*/
        }
        weight_idx += 1;
        if weight_idx >= params.width * params.height {
            break;
        }
    }

    // Do infill if necessary (Section C.2.18) ...
    let Ds = (1024 + (block_width / 2)) / (block_width - 1);
    let Dt = (1024 + (block_height / 2)) / (block_height - 1);

    let plane_scale = if params.is_dual_plane { 2 } else { 1 };
    for plane in 0..plane_scale {
        for t in 0..block_height {
            for s in 0..block_width {
                let cs = Ds * s;
                let ct = Dt * t;

                let gs = (cs * (params.width - 1) + 32) >> 6;
                let gt = (ct * (params.height - 1) + 32) >> 6;

                let js = gs >> 4;
                let fs = gs & 0xF;

                let jt = gt >> 4;
                let ft = gt & 0x0F;

                let w11 = (fs * ft + 8) >> 4;
                let w10 = ft - w11;
                let w01 = fs - w11;
                let w00 = 16 - fs - ft + w11;

                let v0 = js + jt * params.width;

                let mut p00 = 0;
                let mut p01 = 0;
                let mut p10 = 0;
                let mut p11 = 0;

                if v0 < (params.width * params.height) {
                    p00 = unquantized[plane][v0 as usize];
                }

                if v0 + 1 < (params.width * params.height) {
                    p01 = unquantized[plane][(v0 + 1) as usize];
                }

                if v0 + params.width < (params.width * params.height) {
                    p10 = unquantized[plane][(v0 + params.width) as usize];
                }

                if v0 + params.width + 1 < (params.width * params.height) {
                    p11 = unquantized[plane][(v0 + params.width + 1) as usize];
                }

                out[plane][(t * block_width + s) as usize] =
                    (p00 * w00 + p01 * w01 + p10 * w10 + p11 * w11 + 8) >> 4;
            }
        }
    }
}

// Transfers a bit as described in C.2.14
fn bit_transfer_signed(a: &mut i32, b: &mut i32) {
    *b >>= 1;
    *b |= *a & 0x80;
    *a >>= 1;
    *a &= 0x3F;
    if (*a & 0x20) != 0 {
        *a -= 0x40;
    }
}

// Adds more precision to the blue channel as described
// in C.2.14
fn blue_contract(a: i32, r: i32, g: i32, b: i32) -> Pixel {
    Pixel::new(
        a as i16,
        ((r + b) >> 1) as i16,
        ((g + b) >> 1) as i16,
        b as i16,
    )
}

// Partition selection functions as specified in
// C.2.21
fn hash52(mut p: u32) -> u32 {
    p ^= p >> 15;
    p -= p << 17;
    p += p << 7;
    p += p << 4;
    p ^= p >> 5;
    p += p << 16;
    p ^= p >> 7;
    p ^= p >> 3;
    p ^= p << 6;
    p ^= p >> 17;
    p
}

fn select_partition(
    mut seed: i32,
    mut x: i32,
    mut y: i32,
    mut z: i32,
    partition_count: i32,
    small_block: bool,
) -> u32 {
    if 1 == partition_count {
        return 0;
    }

    if small_block {
        x <<= 1;
        y <<= 1;
        z <<= 1;
    }

    seed += (partition_count - 1) * 1024;

    let rnum = hash52(seed as u32);
    let mut seed1 = (rnum & 0xF) as u8;
    let mut seed2 = ((rnum >> 4) & 0xF) as u8;
    let mut seed3 = ((rnum >> 8) & 0xF) as u8;
    let mut seed4 = ((rnum >> 12) & 0xF) as u8;
    let mut seed5 = ((rnum >> 16) & 0xF) as u8;
    let mut seed6 = ((rnum >> 20) & 0xF) as u8;
    let mut seed7 = ((rnum >> 24) & 0xF) as u8;
    let mut seed8 = ((rnum >> 28) & 0xF) as u8;
    let mut seed9 = ((rnum >> 18) & 0xF) as u8;
    let mut seed10 = ((rnum >> 22) & 0xF) as u8;
    let mut seed11 = ((rnum >> 26) & 0xF) as u8;
    let mut seed12 = (((rnum >> 30) | (rnum << 2)) & 0xF) as u8;

    seed1 = seed1 * seed1;
    seed2 = seed2 * seed2;
    seed3 = seed3 * seed3;
    seed4 = seed4 * seed4;
    seed5 = seed5 * seed5;
    seed6 = seed6 * seed6;
    seed7 = seed7 * seed7;
    seed8 = seed8 * seed8;
    seed9 = seed9 * seed9;
    seed10 = seed10 * seed10;
    seed11 = seed11 * seed11;
    seed12 = seed12 * seed12;

    let sh1: i32;
    let sh2: i32;
    let sh3: i32;
    if seed & 1 != 0 {
        sh1 = if seed & 2 != 0 { 4 } else { 5 };
        sh2 = if partition_count == 3 { 6 } else { 5 };
    } else {
        sh1 = if partition_count == 3 { 6 } else { 5 };
        sh2 = if seed & 2 != 0 { 4 } else { 5 };
    }
    sh3 = if seed & 0x10 != 0 { sh1 } else { sh2 };

    seed1 >>= sh1;
    seed2 >>= sh2;
    seed3 >>= sh1;
    seed4 >>= sh2;
    seed5 >>= sh1;
    seed6 >>= sh2;
    seed7 >>= sh1;
    seed8 >>= sh2;
    seed9 >>= sh3;
    seed10 >>= sh3;
    seed11 >>= sh3;
    seed12 >>= sh3;

    let mut a: i32 = seed1 as i32 * x + seed2 as i32 * y + seed11 as i32 * z + (rnum >> 14) as i32;
    let mut b: i32 = seed3 as i32 * x + seed4 as i32 * y + seed12 as i32 * z + (rnum >> 10) as i32;
    let mut c: i32 = seed5 as i32 * x + seed6 as i32 * y + seed9 as i32 * z + (rnum >> 6) as i32;
    let mut d: i32 = seed7 as i32 * x + seed8 as i32 * y + seed10 as i32 * z + (rnum >> 2) as i32;

    a &= 0x3F;
    b &= 0x3F;
    c &= 0x3F;
    d &= 0x3F;

    if partition_count < 4 {
        d = 0;
    }

    if partition_count < 3 {
        c = 0;
    }

    if a >= b && a >= c && a >= d {
        0
    } else if b >= c && b >= d {
        1
    } else if c >= d {
        2
    } else {
        3
    }
}

fn select_2d_partition(seed: i32, x: i32, y: i32, partition_count: i32, small_block: bool) -> u32 {
    select_partition(seed, x, y, 0, partition_count, small_block)
}

// Section C.2.14
fn compute_endpos32s(
    ep1: &mut Pixel,
    ep2: &mut Pixel,
    color_values: &mut &[u32],
    color_endpos32_mod: u32,
) {
    macro_rules! read_uint_values {
        ($N:expr) => {{
            let mut v = [0; $N];
            for i in 0..$N {
                v[i] = color_values[0];
                *color_values = &color_values[1..];
            }
            v
        }};
    }

    macro_rules! read_int_values {
        ($N:expr) => {{
            let mut v = [0; $N];
            for i in 0..$N {
                v[i] = color_values[0] as i32;
                *color_values = &color_values[1..];
            }
            v
        }};
    }

    macro_rules! bts {
        ($v:ident, $a:expr, $b: expr) => {{
            let mut a = $v[$a];
            let mut b = $v[$b];
            bit_transfer_signed(&mut a, &mut b);
            $v[$a] = a;
            $v[$b] = b;
        }};
    }

    match color_endpos32_mod {
        0 => {
            let v = read_uint_values!(2);
            *ep1 = Pixel::new(0xFF, v[0] as i16, v[0] as i16, v[0] as i16);
            *ep2 = Pixel::new(0xFF, v[1] as i16, v[1] as i16, v[1] as i16);
        }

        1 => {
            let v = read_uint_values!(2);
            let L0 = (v[0] >> 2) | (v[1] & 0xC0);
            let L1 = std::cmp::max(L0 + (v[1] & 0x3F), 0xFF);
            *ep1 = Pixel::new(0xFF, L0 as i16, L0 as i16, L0 as i16);
            *ep2 = Pixel::new(0xFF, L1 as i16, L1 as i16, L1 as i16);
        }

        4 => {
            let v = read_uint_values!(4);
            *ep1 = Pixel::new(v[2] as i16, v[0] as i16, v[0] as i16, v[0] as i16);
            *ep2 = Pixel::new(v[3] as i16, v[1] as i16, v[1] as i16, v[1] as i16);
        }

        5 => {
            let mut v = read_int_values!(4);
            bts!(v, 1, 0);
            bts!(v, 3, 2);
            *ep1 = Pixel::new(v[2] as i16, v[0] as i16, v[0] as i16, v[0] as i16);
            *ep2 = Pixel::new(
                (v[2] + v[3]) as i16,
                (v[0] + v[1]) as i16,
                (v[0] + v[1]) as i16,
                (v[0] + v[1]) as i16,
            );
            ep1.clamp_byte();
            ep2.clamp_byte();
        }

        6 => {
            let v = read_uint_values!(4);
            *ep1 = Pixel::new(
                0xFF,
                ((v[0] * v[3]) >> 8) as i16,
                ((v[1] * v[3]) >> 8) as i16,
                ((v[2] * v[3]) >> 8) as i16,
            );
            *ep2 = Pixel::new(0xFF, v[0] as i16, v[1] as i16, v[2] as i16);
        }

        8 => {
            let v = read_uint_values!(6);
            if v[1] + v[3] + v[5] >= v[0] + v[2] + v[4] {
                *ep1 = Pixel::new(0xFF, v[0] as i16, v[2] as i16, v[4] as i16);
                *ep2 = Pixel::new(0xFF, v[1] as i16, v[3] as i16, v[5] as i16);
            } else {
                *ep1 = blue_contract(0xFF, v[1] as i32, v[3] as i32, v[5] as i32);
                *ep2 = blue_contract(0xFF, v[0] as i32, v[2] as i32, v[4] as i32);
            }
        }

        9 => {
            let mut v = read_int_values!(6);
            bts!(v, 1, 0);
            bts!(v, 3, 2);
            bts!(v, 5, 4);
            if v[1] + v[3] + v[5] >= 0 {
                *ep1 = Pixel::new(0xFF, v[0] as i16, v[2] as i16, v[4] as i16);
                *ep2 = Pixel::new(
                    0xFF,
                    (v[0] + v[1]) as i16,
                    (v[2] + v[3]) as i16,
                    (v[4] + v[5]) as i16,
                );
            } else {
                *ep1 = blue_contract(0xFF, v[0] + v[1], v[2] + v[3], v[4] + v[5]);
                *ep2 = blue_contract(0xFF, v[0], v[2], v[4]);
            }
            ep1.clamp_byte();
            ep2.clamp_byte();
        }

        10 => {
            let v = read_uint_values!(6);
            *ep1 = Pixel::new(
                v[4] as i16,
                ((v[0] * v[3]) >> 8) as i16,
                ((v[1] * v[3]) >> 8) as i16,
                ((v[2] * v[3]) >> 8) as i16,
            );
            *ep2 = Pixel::new(v[5] as i16, v[0] as i16, v[1] as i16, v[2] as i16);
        }

        12 => {
            let v = read_uint_values!(8);
            if v[1] + v[3] + v[5] >= v[0] + v[2] + v[4] {
                *ep1 = Pixel::new(v[6] as i16, v[0] as i16, v[2] as i16, v[4] as i16);
                *ep2 = Pixel::new(v[7] as i16, v[1] as i16, v[3] as i16, v[5] as i16);
            } else {
                *ep1 = blue_contract(v[7] as i32, v[1] as i32, v[3] as i32, v[5] as i32);
                *ep2 = blue_contract(v[6] as i32, v[0] as i32, v[2] as i32, v[4] as i32);
            }
        }

        13 => {
            let mut v = read_int_values!(8);
            bts!(v, 1, 0);
            bts!(v, 3, 2);
            bts!(v, 5, 4);
            bts!(v, 7, 6);
            if v[1] + v[3] + v[5] >= 0 {
                *ep1 = Pixel::new(v[6] as i16, v[0] as i16, v[2] as i16, v[4] as i16);
                *ep2 = Pixel::new(
                    (v[7] + v[6]) as i16,
                    (v[0] + v[1]) as i16,
                    (v[2] + v[3]) as i16,
                    (v[4] + v[5]) as i16,
                );
            } else {
                *ep1 = blue_contract(v[6] + v[7], v[0] + v[1], v[2] + v[3], v[4] + v[5]);
                *ep2 = blue_contract(v[6], v[0], v[2], v[4]);
            }
            ep1.clamp_byte();
            ep2.clamp_byte();
        }

        _ => panic!("Unsupported color endpoint mode (is it HDR?)"),
    }
}

pub fn atsc_decompress_block<F: FnMut(usize, usize, (u8, u8, u8, u8))>(
    in_buf: &[u8; 16],
    block_width: u32,
    block_height: u32,
    mut writer: F,
) {
    let mut strm = InputBitStream::new(in_buf, 0);
    let weight_params = decode_block_info(&mut strm);

    // Was there an error?
    if weight_params.is_error {
        fill_error(&mut writer, block_width, block_height);
        return;
    }

    if weight_params.void_extent_ldr {
        fill_void_extent_ldr(&mut strm, &mut writer, block_width, block_height);
        return;
    }

    if weight_params.void_extent_hdr {
        fill_error(&mut writer, block_width, block_height);
        return;
    }

    if weight_params.width > block_width {
        fill_error(&mut writer, block_width, block_height);
        return;
    }

    if weight_params.height > block_height {
        fill_error(&mut writer, block_width, block_height);
        return;
    }

    // Read num partitions
    let n_partitions = strm.read_bits(2) + 1;
    assert!(n_partitions <= 4);

    if n_partitions == 4 && weight_params.is_dual_plane {
        fill_error(&mut writer, block_width, block_height);
        return;
    }

    // Based on the number of partitions, read the color endpos32 mode for
    // each partition.

    // Determine partitions, partition index, and color endpos32 modes
    let plane_idx;
    let partition_index;
    let mut color_endpos32_mod = [0, 0, 0, 0];

    // Define color data.
    let mut color_endpos32_data = [0; 16];
    let mut color_endpos32_stream = OutputBitStream::new(&mut color_endpos32_data);

    // Read extra config data...
    let mut base_cem = 0;
    if n_partitions == 1 {
        color_endpos32_mod[0] = strm.read_bits(4);
        partition_index = 0;
    } else {
        partition_index = strm.read_bits(10);
        base_cem = strm.read_bits(6);
    }
    let base_mode = base_cem & 3;

    // Remaining bits are color endpos32 data...
    let n_weight_bits = weight_params.get_packed_bit_size();
    let mut remaining_bits = 128 - n_weight_bits - strm.get_bits_read() as u32;

    // Consider extra bits prior to texel data...
    let mut extra_cem_bits = 0;
    if base_mode != 0 {
        match n_partitions {
            2 => extra_cem_bits += 2,
            3 => extra_cem_bits += 5,
            4 => extra_cem_bits += 8,
            _ => panic!(),
        }
    }
    remaining_bits -= extra_cem_bits;

    // Do we have a dual plane situation?
    let mut plane_selector_bits = 0;
    if weight_params.is_dual_plane {
        plane_selector_bits = 2;
    }
    remaining_bits -= plane_selector_bits;

    // Read color data...
    let color_data_bits = remaining_bits;
    while remaining_bits > 0 {
        let nb = std::cmp::min(remaining_bits, 8);
        let b = strm.read_bits(nb);
        color_endpos32_stream.write_bits(b, nb);
        remaining_bits -= nb;
    }

    // Read the plane selection bits
    plane_idx = strm.read_bits(plane_selector_bits);

    // Read the rest of the cem
    if base_mode != 0 {
        let extra_cem = strm.read_bits(extra_cem_bits);
        let mut cem = (extra_cem << 6) | base_cem;
        cem >>= 2;

        let mut C = [false; 4];
        for i in 0..n_partitions {
            C[i as usize] = (cem & 1) != 0;
            cem >>= 1;
        }

        let mut M = [0; 4];
        for i in 0..n_partitions {
            M[i as usize] = cem & 3;
            cem >>= 2;
            assert!(M[i as usize] <= 3);
        }

        for i in 0..n_partitions {
            color_endpos32_mod[i as usize] = base_mode;
            if !C[i as usize] {
                color_endpos32_mod[i as usize] -= 1;
            }
            color_endpos32_mod[i as usize] <<= 2;
            color_endpos32_mod[i as usize] |= M[i as usize];
        }
    } else if n_partitions > 1 {
        let cem = base_cem >> 2;
        for i in 0..n_partitions {
            color_endpos32_mod[i as usize] = cem;
        }
    }

    // Make sure everything up till here is sane.
    for i in 0..n_partitions {
        assert!(color_endpos32_mod[i as usize] < 16);
    }
    assert!(strm.get_bits_read() as u32 + weight_params.get_packed_bit_size() == 128);

    // Decode both color data and texel weight data
    let mut color_values = [0; 32]; // Four values, two endpos32s, four maximum paritions
    decode_color_values(
        &mut color_values,
        &color_endpos32_data,
        &color_endpos32_mod,
        n_partitions,
        color_data_bits,
    );

    let mut endpos32s = [[Pixel::default(); 2]; 4];
    let mut color_values_ptr = &color_values[..];
    for i in 0..n_partitions {
        let mut a = Pixel::default();
        let mut b = Pixel::default();
        compute_endpos32s(
            &mut a,
            &mut b,
            &mut color_values_ptr,
            color_endpos32_mod[i as usize],
        );
        endpos32s[i as usize][0] = a;
        endpos32s[i as usize][1] = b;
    }

    // Read the texel weight data..
    let mut texel_weight_data = in_buf.clone();

    // Reverse everything
    for i in 0..8 {
        let a = texel_weight_data[i].reverse_bits();
        let b = texel_weight_data[15 - i].reverse_bits();

        texel_weight_data[i] = b;
        texel_weight_data[15 - i] = a;
    }

    // Make sure that higher non-texel bits are set to zero
    let clear_byte_start = (weight_params.get_packed_bit_size() >> 3) + 1;
    if clear_byte_start > 0 && clear_byte_start <= texel_weight_data.len() as u32 {
        texel_weight_data[clear_byte_start as usize - 1] &=
            (1 << (weight_params.get_packed_bit_size() % 8)) - 1;
        texel_weight_data[clear_byte_start as usize..].fill(0);
    }

    let mut texel_weight_values = vec![];

    let mut weight_stream = InputBitStream::new(&texel_weight_data, 0);

    decode_integer_sequence(
        &mut texel_weight_values,
        &mut weight_stream,
        weight_params.max_weight,
        weight_params.get_num_weight_values(),
    );

    // Blocks can be at most 12x12, so we can have as many as 144 weights
    let mut weights = [[0; 144]; 2];
    unquantize_texel_weights(
        &mut weights,
        &texel_weight_values,
        &weight_params,
        block_width,
        block_height,
    );

    // Now that we have endpos32s and weights, we can s32erpolate and generate
    // the proper decoding...
    for j in 0..block_height {
        for i in 0..block_width {
            let partition = select_2d_partition(
                partition_index as i32,
                i as i32,
                j as i32,
                n_partitions as i32,
                (block_height * block_width) < 32,
            );
            assert!(partition < n_partitions);

            let mut p = Pixel::default();
            for c in 0..4 {
                let C0 = endpos32s[partition as usize][0].component(c);
                let C0 = replicate(C0 as u32, 8, 16);
                let C1 = endpos32s[partition as usize][1].component(c);
                let C1 = replicate(C1 as u32, 8, 16);

                let mut plane = 0;
                if weight_params.is_dual_plane && (((plane_idx + 1) & 3) == c) {
                    plane = 1;
                }

                let weight = weights[plane][(j * block_width + i) as usize];
                let C = (C0 * (64 - weight) + C1 * weight + 32) / 64;
                if C == 65535 {
                    *p.component_mut(c) = 255;
                } else {
                    let Cf = C as f64;
                    *p.component_mut(c) = (255.0 * (Cf / 65536.0) + 0.5) as i16;
                }
            }

            writer(i as usize, j as usize, p.pack());
        }
    }
}
