// We use the dynamic feature to select which crate we use.

#[cfg(not(feature = "dynamic"))]
pub use test_internal::*;

#[cfg(feature = "dynamic")]
pub use test_dylib::*;

pub fn info() {
    println!("Dynamic: {}", cfg!(feature = "dynamic"));
}
