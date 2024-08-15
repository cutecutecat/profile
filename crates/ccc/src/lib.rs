#[cfg(target_arch = "x86_64")]
#[link(name = "assets", kind = "static")]
extern "C" {
    pub fn ip_byte_bin(x: *const u64, y: *const u64, len: u32) -> u32;
    pub fn ip_byte_bin_16(x: *const u64, y: *const u64) -> u32;
    pub fn ip_byte_bin_120(x: *const u64, y: *const u64) -> u32;
    pub fn ip_byte_bin_200(x: *const u64, y: *const u64) -> u32;
}