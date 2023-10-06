use crate::{MTLBuffer, MTLComputePipelineState, MTLRenderPipelineState, MTLSamplerState};
use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};
use savannah::{handle, release, retain};
use std::os::raw::c_void;

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlargumentencoder?language=objc).
pub struct MTLArgumentEncoder(pub *mut Object);
handle!(MTLArgumentEncoder);

impl MTLArgumentEncoder {
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlargumentencoder/2915777-setargumentbuffer?language=objc).
    pub unsafe fn set_argument_buffer(&self, buffer: MTLBuffer, offset: u64) {
        let _: () = msg_send![self.0, setArgumentBuffer:buffer.0 offset:offset];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlargumentencoder/2915780-setargumentbuffer?language=objc).
    pub unsafe fn set_argument_buffer_with_start_offset(
        &self,
        buffer: MTLBuffer,
        start_offset: u64,
        array_element: u64,
    ) {
        let _: () = msg_send![
            self.0,
            setArgumentBuffer:buffer.0
            startOffset:start_offset
            arrayElement:array_element
        ];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlargumentencoder/2915784-encodedlength?language=objc).
    pub unsafe fn get_encoded_length(&self) -> u64 {
        msg_send![self.0, encodedLength]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlargumentencoder/2915785-setbuffer?language=objc).
    pub unsafe fn set_buffer(&self, buffer: MTLBuffer, offset: u64, index: u64) {
        let _: () = msg_send![self.0, setBuffer:buffer.0 offset:offset atIndex:index];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlargumentencoder/2915770-setsamplerstate?language=objc).
    pub unsafe fn set_sampler_state(&self, state: MTLSamplerState, index: u64) {
        let _: () = msg_send![self.0, setSamplerState:state.0 atIndex:index];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlargumentencoder/2966535-setrenderpipelinestate?language=objc).
    pub unsafe fn set_render_pipeline_state(&self, state: MTLRenderPipelineState, index: u64) {
        let _: () = msg_send![self.0, setRenderPipelineState:state.0 atIndex:index];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlargumentencoder/2966533-setcomputepipelinestate?language=objc).
    pub unsafe fn set_compute_pipeline_state(&self, state: MTLComputePipelineState, index: u64) {
        let _: () = msg_send![self.0, setComputePipelineState:state.0 atIndex:index];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlargumentencoder/2915771-constantdataatindex?language=objc).
    pub unsafe fn get_constant_data_at_index(&self, index: u64) -> *mut c_void {
        msg_send![self.0, constantDataAtIndex: index]
    }
}
