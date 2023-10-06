mod indirect;
pub use indirect::*;

use crate::MTLComputeCommandEncoder;
use crate::{CAMetalDrawable, MTLRenderCommandEncoder, MTLRenderPassDescriptor};
use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};
use savannah::{handle, release, retain};

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlcommandbuffer?language=objc).
pub struct MTLCommandBuffer(pub *mut Object);
handle!(MTLCommandBuffer);

impl MTLCommandBuffer {
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcommandbuffer/1442999-rendercommandencoderwithdescript?language=objc).
    pub unsafe fn new_render_command_encoder_with_descriptor(
        &self,
        descriptor: MTLRenderPassDescriptor,
    ) -> MTLRenderCommandEncoder {
        MTLRenderCommandEncoder(retain(
            msg_send![self.0, renderCommandEncoderWithDescriptor:descriptor.0],
        ))
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcommandbuffer/1443044-computecommandencoder?language=objc).
    pub unsafe fn new_compute_command_encoder(&self) -> MTLComputeCommandEncoder {
        MTLComputeCommandEncoder(retain(msg_send![self.0, computeCommandEncoder]))
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcommandbuffer/2966541-computecommandencoderwithdispatc?language=objc).
    /// [MTLDispatchType docs](https://developer.apple.com/documentation/metal/mtldispatchtype?language=objc).
    pub unsafe fn new_compute_command_encoder_with_dispatch_type(
        &self,
        dispatch_type: u64,
    ) -> MTLComputeCommandEncoder {
        MTLComputeCommandEncoder(retain(msg_send![
            self.0,
            computeCommandEncoderWithDispatchType: dispatch_type
        ]))
    }
    // TODO add the blit encoder
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcommandbuffer/1443019-enqueue?language=objc).
    pub unsafe fn enqueue(&self) {
        let _: () = msg_send![self.0, enqueue];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcommandbuffer/1443003-commit?language=objc).
    pub unsafe fn commit(&self) {
        let _: () = msg_send![self.0, commit];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcommandbuffer/1443036-waituntilscheduled?language=objc).
    pub unsafe fn wait_until_scheduled(&self) {
        let _: () = msg_send![self.0, waitUntilScheduled];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcommandbuffer/1443039-waituntilcompleted?language=objc).
    pub unsafe fn wait_until_completed(&self) {
        let _: () = msg_send![self.0, waitUntilCompleted];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcommandbuffer/1443029-presentdrawable?language=objc).
    pub unsafe fn present_drawable(&self, drawable: CAMetalDrawable) {
        let _: () = msg_send![self.0, presentDrawable:drawable.0];
    }
}
