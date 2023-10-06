use crate::{
    MTLBuffer, MTLCommandEncoder, MTLComputePipelineState, MTLFence, MTLHeap, MTLResource,
    MTLSamplerState, MTLSize, MTLTexture,
};
use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};
use savannah::{handle, release, retain};
use std::os::raw::c_void;

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder?language=objc).
pub struct MTLComputeCommandEncoder(pub *mut Object);
handle!(MTLComputeCommandEncoder);

impl MTLCommandEncoder for MTLComputeCommandEncoder {
    fn get_obj(&self) -> *mut Object {
        self.0
    }
}

impl MTLComputeCommandEncoder {
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/1443140-setcomputepipelinestate?language=objc).
    pub unsafe fn set_compute_pipeline_state(&self, state: MTLComputePipelineState) {
        let _: () = msg_send![self.0, setComputePipelineState:state.0];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/1443126-setbuffer?language=objc).
    pub unsafe fn set_buffer(&self, buffer: MTLBuffer, offset: u64, index: u64) {
        let _: () = msg_send![self.0, setBuffer:buffer.0 offset:offset atIndex:index];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/1443146-setbufferoffset?language=objc).
    pub unsafe fn set_buffer_offset(&self, offset: u64, index: u64) {
        let _: () = msg_send![self.0, setBufferOffset:offset atIndex:index];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/1443159-setbytes?language=objc).
    pub unsafe fn set_bytes(&self, bytes: *const c_void, length: u64, index: u64) {
        let _: () = msg_send![self.0, setBytes:bytes length:length atIndex:index];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/1443144-setsamplerstate?language=objc).
    pub unsafe fn set_sampler_state(&self, state: MTLSamplerState, index: u64) {
        let _: () = msg_send![self.0, setVertexSamplerState:state.0 atIndex:index];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/1443153-setsamplerstate?language=objc).
    pub unsafe fn set_sampler_state_lod_clamps(
        &self,
        state: MTLSamplerState,
        index: u64,
        lod_min_clamp: f32,
        lod_max_clamp: f32,
    ) {
        let _: () = msg_send![self.0,
            setVertexSamplerState:state.0
            lodMinClamp:lod_min_clamp
            lodMaxClamp:lod_max_clamp
            atIndex:index
        ];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/1443130-settexture?language=objc).
    pub unsafe fn set_texture(&self, texture: MTLTexture, index: u64) {
        let _: () = msg_send![self.0, setTexture:texture.0 atIndex:index];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/1443142-setthreadgroupmemorylength?language=objc).
    pub unsafe fn set_threadgroup_memory_length(&self, length: u64, index: u64) {
        let _: () = msg_send![self.0, setThreadgroupMemoryLength:length atIndex:index];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/1443138-dispatchthreadgroups?language=objc).
    pub unsafe fn dispatch_threadgroups(
        &self,
        threadgroups_per_grid: (u64, u64, u64),
        threads_per_threadgroup: (u64, u64, u64),
    ) {
        let a = MTLSize {
            w: threadgroups_per_grid.0,
            h: threadgroups_per_grid.1,
            d: threadgroups_per_grid.2,
        };
        let b = MTLSize {
            w: threads_per_threadgroup.0,
            h: threads_per_threadgroup.1,
            d: threads_per_threadgroup.2,
        };
        let _: () = msg_send![
            self.0,
            dispatchThreadgroups:a
            threadsPerThreadgroup:b
        ];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/2866532-dispatchthreads?language=objc).
    pub unsafe fn dispatch_threads(
        &self,
        threads_per_grid: (u64, u64, u64),
        threads_per_threadgroup: (u64, u64, u64),
    ) {
        let a = MTLSize {
            w: threads_per_grid.0,
            h: threads_per_grid.1,
            d: threads_per_grid.2,
        };
        let b = MTLSize {
            w: threads_per_threadgroup.0,
            h: threads_per_threadgroup.1,
            d: threads_per_threadgroup.2,
        };
        let _: () = msg_send![
            self.0,
            dispatchThreads:a
            threadsPerThreadgroup:b
        ];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/1443157-dispatchthreadgroupswithindirect?language=objc).
    pub unsafe fn dispatch_threadgroups_with_indirect_buffer(
        &self,
        buffer: MTLBuffer,
        offset: u64,
        threads_per_threadgroup: (u64, u64, u64),
    ) {
        let _: () = msg_send![
            self.0,
            dispatchThreadgroupsWithIndirectBuffer:buffer.0
            indirectBufferOffset:offset
            threadsPerThreadgroup:threads_per_threadgroup
        ];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/2866548-useresource?language=objc).
    /// [MTLResourceUsage docs](https://developer.apple.com/documentation/metal/mtlresourceusage?language=objc).
    pub unsafe fn use_resource<T: MTLResource>(&self, resource: T, usage: u64) {
        let _: () = msg_send![self.0, useResource:resource.get_obj() usage:usage];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/2866530-useheap?language=objc).
    pub unsafe fn use_heap(&self, heap: MTLHeap) {
        let _: () = msg_send![self.0, useHeap:heap.0];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/1649789-updatefence?language=objc).
    pub unsafe fn update_fence(&self, fence: MTLFence) {
        let _: () = msg_send![self.0, updateFence:fence.0];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/1649790-waitforfence?language=objc).
    pub unsafe fn wait_for_fence(&self, fence: MTLFence) {
        let _: () = msg_send![self.0, waitForFence:fence.0];
    }
    // TODO add the rest of the commands here
}
