//! Lightweight, unsafe bindings to the Metal API.
//!
//! # Docs
//!
//! Almost all documentation here links to [Metal documentation](https://developer.apple.com/documentation/metal?language=objc).
//!
//! # Memory Management
//!
//! Note here that all `cull-canyon` structs are wrappers around raw pointers to Objective-C
//! objects. They provide behaviors for interacting with that raw pointer, but also expose it. That
//! exposed raw pointer may have messages sent to it with `objc`'s `msg_send!`.
//!
//! All `cull-canyon` structs have an implementation of `clone` which copies the reference to the
//! underlying object into a new wrapper. Because Objective-C is reference counted, the underlying
//! Objective-C object will at least last for the lifetime of the longest-lasting struct which has a
//! reference to it.
//!
//! Most `cull-canyon` methods take structs by value because they are as lightweight as a pointer.
//!
//! Because these structs mostly have destructors, they do not have implementations of `Copy`,
//! and so must be manually cloned to avoid being moved.
//!
//! # Enums
//!
//! Enum types for Objective-C enums are not yet added. Functions or methods which take
//! or return an enum instead use its discriminant's type (most of the time a u64). You can
//! tell when a method or function needs or returns an enum value from the linked documentation.
//!
//! # Valid Usage
//!
//! The extent of the documentation for most methods or functions is a link to the underlying
//! Metal method or function which is called. There are no checks when an input is given to a
//! `cull-canyon` method.
//!
//! `cull-canyon` has only very minimal error handling. If a `cull-canyon` function or method
//! returns an `Option`, that is handling the case where the returned Objective-C reference is
//! null. If a `Result` is returned, that is handling the case where an error pointer needs to be
//! passed to the underlying Objective-C function or method. `cull-canyon` checks if it's null,
//! and if not, stringifies it and returns it in the result.
//!
//! Note here that all `cull-canyon` functions and methods are marked unsafe. This is because
//! none of these functions can be guaranteed to have consistent behavior without OS version and
//! OS checks.
//!
//! It is undefined behavior to call a method or function whose documentation links to Metal
//! documentation if:
//!
//! - the OS running the program does not support Metal.
//! - the OS version running the program does not support that method or function (specified
//! in the Metal documentation).
//! - values which do not correspond to any valid enum variant are given to methods or functions
//! which ask for a particular enum.
//! - the Metal documentation places restrictions on the input which that method or function can
//! take and those restrictions are violated.
//!
//! # OS Version Checking
//!
//! Note that, to check the version of the OS which you are running, and whether it is a valid
//! OS for running a Metal method or function, query the reexported `savannah` static property
//! `CURRENT_OS_VERSION`.

mod argument;
mod buffer;
mod commandbuffer;
mod commandqueue;
mod depthstencil;
mod device;
mod encoder;
mod event;
mod heap;
mod layer;
mod library;
mod pipeline;
mod resource;
mod sampler;
mod texture;

pub use argument::*;
pub use buffer::*;
pub use commandbuffer::*;
pub use commandqueue::*;
pub use depthstencil::*;
pub use device::*;
pub use encoder::*;
pub use event::*;
pub use heap::*;
pub use layer::*;
pub use library::*;
pub use pipeline::*;
pub use resource::*;
pub use sampler::*;
pub use texture::*;

pub use savannah::CURRENT_OS_VERSION;

#[repr(C)]
pub(crate) struct MTLSize {
    pub(crate) w: u64,
    pub(crate) h: u64,
    pub(crate) d: u64,
}

#[repr(C)]
pub(crate) struct MTLRegion {
    x: u64,
    y: u64,
    z: u64,
    width: u64,
    height: u64,
    depth: u64,
}

#[doc(hidden)]
#[macro_export]
macro_rules! __just_clone {
    ($struct_to_clone:ty) => {
        impl Clone for $struct_to_clone {
            fn clone(&self) -> Self {
                Self(unsafe { retain(self.0) })
            }
        }
    };
}

use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use std::os::raw::c_void;

pub(crate) unsafe fn string_to_nsstring(src: &str) -> *mut Object {
    let cls = class!(NSString);
    let bytes = src.as_ptr() as *const c_void;
    let obj: *mut objc::runtime::Object = msg_send![cls, alloc];
    let obj: *mut objc::runtime::Object = msg_send![
       obj,
       initWithBytes:bytes
       length:src.len()
       encoding:4
    ];
    obj
}

pub(crate) unsafe fn nsstring_to_string<'a>(nsstring: *mut Object) -> &'a str {
    let bytes: *const u8 = msg_send![nsstring, UTF8String];
    let len: u64 = msg_send![nsstring, length];
    let bytes = std::slice::from_raw_parts(bytes, len as usize);
    std::str::from_utf8(bytes).unwrap()
}
