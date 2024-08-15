use ccc::{ip_byte_bin_16, ip_byte_bin_120, ip_byte_bin_200};

pub fn asymmetric_binary_dot_product_16(x: &[u64], y: &[u64]) -> u32 {
    unsafe { ip_byte_bin_16(x.as_ptr().cast(), y.as_ptr().cast()) }
}

pub fn asymmetric_binary_dot_product_120(x: &[u64], y: &[u64]) -> u32 {
    unsafe { ip_byte_bin_120(x.as_ptr().cast(), y.as_ptr().cast()) }
}

pub fn asymmetric_binary_dot_product_200(x: &[u64], y: &[u64]) -> u32 {
    unsafe { ip_byte_bin_200(x.as_ptr().cast(), y.as_ptr().cast()) }
}