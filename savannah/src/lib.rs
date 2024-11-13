//! Lightweight utilities for writing bindings to Objective-C libraries.

mod util;
mod version;
pub use util::*;
pub use version::*;

#[macro_use]
extern crate objc;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    /// The version of the machine using this.
    pub static ref CURRENT_OS_VERSION: Version = get_os_version();
}

/// Implements a few traits on the provided wrapper around a `*mut Object` such that the wrapper
/// maintains a strong reference to the underlying object for the lifetime of the wrapper.
///
/// The provided handle must be a tuple struct with a first parameter that
/// is a `*mut objc::runtime::Object` without an implementation of `Drop` or `Clone`.
#[macro_export]
macro_rules! handle {
    ($obj:ty) => {
        impl Drop for $obj {
            fn drop(&mut self) {
                unsafe { release(self.0) }
            }
        }
        impl Clone for $obj {
            fn clone(&self) -> Self {
                Self(unsafe { retain(self.0) })
            }
        }
    };
}
