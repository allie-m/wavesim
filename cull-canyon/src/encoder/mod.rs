mod argument;
mod compute;
mod render;
pub use argument::*;
pub use compute::*;
pub use render::*;

use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};

pub trait MTLCommandEncoder {
    #[doc(hidden)]
    fn get_obj(&self) -> *mut Object;
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcommandencoder/1458038-endencoding?language=objc).
    unsafe fn end_encoding(&self) {
        let _: () = msg_send![self.get_obj(), endEncoding];
    }
}
