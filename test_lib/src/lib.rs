#[cfg(not(feature = "dynamic"))]
pub use test_internal::*;

#[cfg(feature = "dynamic")]
pub use test_dylib::*;
