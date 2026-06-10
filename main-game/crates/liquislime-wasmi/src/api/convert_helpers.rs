pub fn pack_u32s(left: u32, right: u32) -> u64 {
    ((left as u64) << 32) | (right as u64)
}

pub fn unpack_u32s(packed: u64) -> (u32, u32) {
    let left = (packed >> 32) as u32;
    let right = (packed & 0xFFFFFFFF) as u32;
    (left, right)
}
