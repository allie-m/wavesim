use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use savannah::{handle, release, retain};

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor?language=objc).
pub struct MTLIndirectCommandBufferDescriptor(pub *mut Object);
handle!(MTLIndirectCommandBufferDescriptor);

impl MTLIndirectCommandBufferDescriptor {
    pub unsafe fn new() -> MTLIndirectCommandBufferDescriptor {
        MTLIndirectCommandBufferDescriptor(msg_send![
            class!(MTLIndirectCommandBufferDescriptor),
            new
        ])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/2966582-commandtypes?language=objc).
    /// [MTLIndirectCommandType docs](https://developer.apple.com/documentation/metal/mtlindirectcommandtype?language=objc).
    pub unsafe fn set_command_types(&self, types: u64) {
        let _: () = msg_send![self.0, setCommandTypes: types];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/2967432-inheritbuffers?language=objc).
    pub unsafe fn set_inherit_buffers(&self, set: bool) {
        let _: () = msg_send![self.0, setInheritBuffers: set];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/2966584-inheritpipelinestate?language=objc).
    pub unsafe fn set_inherit_pipeline_state(&self, set: bool) {
        let _: () = msg_send![self.0, setInheritPipelineState: set];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/2976451-maxvertexbufferbindcount?language=objc).
    pub unsafe fn set_max_vertex_buffer_bind_count(&self, count: u64) {
        let _: () = msg_send![self.0, setMaxVertexBufferBindCount: count];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/2976449-maxfragmentbufferbindcount?language=objc).
    pub unsafe fn set_max_fragment_buffer_bind_count(&self, count: u64) {
        let _: () = msg_send![self.0, setMaxFragmentBufferBindCount: count];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/2976450-maxkernelbufferbindcount?language=objc).
    pub unsafe fn set_max_kernel_buffer_bind_count(&self, count: u64) {
        let _: () = msg_send![self.0, setMaxKernelBufferBindCount: count];
    }
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer?language=objc).
pub struct MTLIndirectCommandBuffer(pub *mut Object);
handle!(MTLIndirectCommandBuffer);

impl MTLIndirectCommandBuffer {
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer/2981028-size?language=objc).
    pub unsafe fn get_size(&self) -> u64 {
        msg_send![self.0, size]
    }
    // TODO implement the rest of this behavior + indirect commands
}
