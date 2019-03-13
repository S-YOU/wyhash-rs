const P0: u64 = 0x60be_e2be_e120_fc15;
const P1: u64 = 0xa3b1_9535_4a39_b70d;
const P2: u64 = 0x1b03_7387_12fa_d5c9;
const P3: u64 = 0xd985_068b_c543_9bd7;
const P4: u64 = 0x897f_236f_b004_a8e7;

#[inline]
fn wyhashmix(a: u64, b: u64) -> u64 {
    let r = u128::from(a) * u128::from(b ^ P0);
    ((r >> 64) ^ r) as u64
}

#[inline]
fn read64(data: &[u8]) -> u64 {
    u64::from(data[7]) << 56
        | u64::from(data[6]) << 48
        | u64::from(data[5]) << 40
        | u64::from(data[4]) << 32
        | u64::from(data[3]) << 24
        | u64::from(data[2]) << 16
        | u64::from(data[1]) << 8
        | u64::from(data[0])
}

#[inline]
fn read_rest(data: &[u8]) -> u64 {
    // This may be mathematically acceptable but the hashes would change as the byte sorting changes.
    // let mut result = 0;
    // for i in 0..data.len() {
    //     result |= u64::from(data[i]) << ((data.len() - i - 1) * 8);
    // }
    // result

    match data.len() {
        1 => u64::from(data[0]),
        2 => u64::from(data[1]) << 8 | u64::from(data[0]),
        3 => u64::from(data[1]) << 16 | u64::from(data[0]) << 8 | u64::from(data[2]),
        4 => {
            u64::from(data[3]) << 24
                | u64::from(data[2]) << 16
                | u64::from(data[1]) << 8
                | u64::from(data[0])
        }
        5 => {
            u64::from(data[3]) << 32
                | u64::from(data[2]) << 24
                | u64::from(data[1]) << 16
                | u64::from(data[0]) << 8
                | u64::from(data[4])
        }
        6 => {
            u64::from(data[3]) << 40
                | u64::from(data[2]) << 32
                | u64::from(data[1]) << 24
                | u64::from(data[0]) << 16
                | u64::from(data[5]) << 8
                | u64::from(data[4])
        }
        7 => {
            u64::from(data[3]) << 48
                | u64::from(data[2]) << 40
                | u64::from(data[1]) << 32
                | u64::from(data[0]) << 24
                | u64::from(data[5]) << 16
                | u64::from(data[4]) << 8
                | u64::from(data[6])
        }
        8 => read64(data),
        _ => panic!(),
    }
}

/// Generate a hash for the input data and seed
pub fn wyhash(bytes: &[u8], seed: u64) -> u64 {
    let seed = wyhash_core(bytes, seed);
    wyhash_finish(bytes.len() as u64, seed)
}

#[inline]
pub(crate) fn wyhash_core(bytes: &[u8], mut seed: u64) -> u64 {
    for i in 0..(bytes.len() / 32) {
        seed = wyhashmix(seed ^ P1, read64(&bytes[(i * 32)..]))
            ^ wyhashmix(seed ^ P2, read64(&bytes[(i * 32) + 8..]))
            ^ wyhashmix(seed ^ P3, read64(&bytes[(i * 32) + 16..]))
            ^ wyhashmix(seed ^ P4, read64(&bytes[(i * 32) + 24..]));
    }

    let rest = bytes.len() & 31;
    if rest != 0 {
        let start = bytes.len() & !31;
        match ((bytes.len() - 1) & 31) / 8 {
            0 => seed = wyhashmix(seed ^ P1, read_rest(&bytes[start..])),
            1 => {
                seed = wyhashmix(seed ^ P1, read64(&bytes[start..]))
                    ^ wyhashmix(seed ^ P2, read_rest(&bytes[start + 8..]))
            }

            2 => {
                seed = wyhashmix(seed ^ P1, read64(&bytes[start..]))
                    ^ wyhashmix(seed ^ P2, read64(&bytes[start + 8..]))
                    ^ wyhashmix(seed ^ P3, read_rest(&bytes[start + 16..]))
            }

            3 => {
                seed = wyhashmix(seed ^ P1, read64(&bytes[start..]))
                    ^ wyhashmix(seed ^ P2, read64(&bytes[start + 8..]))
                    ^ wyhashmix(seed ^ P3, read64(&bytes[start + 16..]))
                    ^ wyhashmix(seed ^ P4, read_rest(&bytes[start + 24..]))
            }
            _ => unreachable!(),
        }
    }
    seed
}

#[inline]
pub(crate) fn wyhash_finish(length: u64, seed: u64) -> u64 {
    wyhashmix(seed, length)
}

// TODO add PRNG

#[cfg(test)]
mod tests {
    use super::{read64, read_rest};

    #[test]
    fn read_rest_byte_sorting() {
        assert_eq!(0x01, read_rest(&[1]));
        assert_eq!(0x0201, read_rest(&[1, 2]));
        assert_eq!(0x020103, read_rest(&[1, 2, 3]));
        assert_eq!(0x04030201, read_rest(&[1, 2, 3, 4]));
        assert_eq!(0x0403020105, read_rest(&[1, 2, 3, 4, 5]));
        assert_eq!(0x040302010605, read_rest(&[1, 2, 3, 4, 5, 6]));
        assert_eq!(0x04030201060507, read_rest(&[1, 2, 3, 4, 5, 6, 7]));
        assert_eq!(0x0807060504030201, read_rest(&[1, 2, 3, 4, 5, 6, 7, 8]));
    }

    #[test]
    fn read64_byte_sorting() {
        assert_eq!(0x0807060504030201, read64(&[1, 2, 3, 4, 5, 6, 7, 8]));
    }
}
