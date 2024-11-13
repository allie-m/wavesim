use crate::MTLCommandBuffer;
use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};
use savannah::{handle, release, retain};

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlcommandqueue?language=objc).
pub struct MTLCommandQueue(pub *mut Object);
handle!(MTLCommandQueue);

impl MTLCommandQueue {
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcommandqueue/1508686-commandbuffer?language=objc).
    pub unsafe fn new_command_buffer(&self) -> MTLCommandBuffer {
        MTLCommandBuffer(retain(msg_send![self.0, commandBuffer]))
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcommandqueue/1508684-commandbufferwithunretainedrefer?language=objc).
    pub unsafe fn new_command_buffer_with_unretained_references(&self) -> MTLCommandBuffer {
        MTLCommandBuffer(retain(msg_send![
            self.0,
            commandBufferWithUnretainedReferences
        ]))
    }
}
