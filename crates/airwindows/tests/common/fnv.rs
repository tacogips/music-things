#![allow(dead_code)]

pub fn fnv1a64(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325_u64, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
    })
}

pub fn f32_fnv(samples: &[f32]) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for sample in samples {
        for byte in sample.to_le_bytes() {
            hash = (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3);
        }
    }
    hash
}
