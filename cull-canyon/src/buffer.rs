use crate::MTLResource;
use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};
use savannah::{handle, release, retain};
use std::ops::Range;
use std::os::raw::c_void;

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlbuffer?language=objc).
pub struct MTLBuffer(pub *mut Object);
handle!(MTLBuffer);

impl MTLBuffer {
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlbuffer/1515716-contents?language=objc).
    pub unsafe fn get_contents(&self) -> *mut c_void {
        msg_send![self.0, contents]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlbuffer/1516121-didmodifyrange?language=objc).
    pub unsafe fn did_modify_range(&self, range: Range<u64>) {
        let _: () = msg_send![self.0, didModifyRange:(range.start, range.end)];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlbuffer/1515373-length?language=objc).
    pub unsafe fn get_length(&self) -> u64 {
        msg_send![self.0, length]
    }
}

impl MTLResource for MTLBuffer {
    fn get_obj(&self) -> *mut Object {
        self.0
    }
}
