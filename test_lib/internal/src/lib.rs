// Since the dylib crate will need the types that we use in the functions,
// and it would be error prone to copy them there, we wrap them in a module
// to export them. This way the crate's root isn't poluted.
pub mod import {
    pub use core::ffi::{c_int, c_ulong};
}
use import::*;

// We need to make sure that the dylib crate also exports public items from
// this crate that are not extern.
pub fn other_function() {
    println!("don't forget to export this as well!");
}
pub const OTHER_CONSTANT: u32 = 256;

// The export feature can be used to control if the macro is invoked.
#[cfg_attr(feature = "export", crex::export(test_internal))]
extern "C" {
    pub fn test();
    pub fn test_args(a: c_int, b: c_int);
    pub fn test_ret() -> c_ulong;
    #[cfg(feature = "variadic")]
    pub fn test_variadic(n: c_int, ...) -> c_int;
}
