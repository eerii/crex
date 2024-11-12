pub use internal::*;

mod internal {
    use core::ffi::{c_char, c_int, c_ulong};

    extern "C" {
        pub fn test();
        pub fn test_args(a: c_int, b: c_char);
        pub fn test_ret() -> c_ulong;
    }
}
