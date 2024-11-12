#![cfg_attr(feature = "variadic", feature(c_variadic))]

extern crate test_internal;

// We need the types of the function parameters.
use test_internal::import::*;

// Imports the public members of the internal library.
// The functions defined in the extern block will be shadowed!
pub use test_internal::*;

// This will correctly reexport functions from the internal library.
// Remove this line to verify that there are linking errors otherwise.
export_test_internal!();
