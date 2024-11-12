//extern crate crex;

use core::ffi::{c_char, c_int, c_ulong};

#[cfg_attr(feature = "export", crex::export(test_internal))]
extern "C" {
    pub fn test();
    pub fn test_args(a: c_int, b: c_char);
    pub fn test_ret() -> c_ulong;
}
