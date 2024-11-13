use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use savannah::{handle, release, retain};

/// [Metal docs](https://developer.apple.com/documentation/metal/mtldepthstencildescriptor?language=objc).
pub struct MTLDepthStencilDescriptor(pub *mut Object);
handle!(MTLDepthStencilDescriptor);

impl MTLDepthStencilDescriptor {
    pub unsafe fn new() -> MTLDepthStencilDescriptor {
        MTLDepthStencilDescriptor(msg_send![class!(MTLDepthStencilDescriptor), new])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldepthstencildescriptor/1462463-depthcomparefunction?language=objc).
    /// [MTLCompareFunction docs](https://developer.apple.com/documentation/metal/mtlcomparefunction?language=objc).
    pub unsafe fn set_depth_compare_function(&self, function: u64) {
        let _: () = msg_send![self.0, setDepthCompareFunction: function];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldepthstencildescriptor/1462501-depthwriteenabled?language=objc).
    pub unsafe fn set_depth_write_enabled(&self, enabled: bool) {
        let _: () = msg_send![self.0, setDepthWriteEnabled: enabled];
    }
    // TODO add my stencil behavior
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtldepthstencilstate?language=objc).
pub struct MTLDepthStencilState(pub *mut Object);
handle!(MTLDepthStencilState);
