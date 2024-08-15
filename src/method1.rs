use ccc::ip_byte_bin;

pub fn asymmetric_binary_dot_product(x: &[u64], y: &[u64]) -> u32 {
    unsafe { ip_byte_bin(x.as_ptr().cast(), y.as_ptr().cast(), x.len() as u32) }
}
