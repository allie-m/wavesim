use crate::{
    nsstring_to_string, string_to_nsstring, MTLArgumentDescriptor, MTLArgumentEncoder, MTLBuffer,
    MTLCommandQueue, MTLCompileOptions, MTLComputePipelineDescriptor, MTLComputePipelineState,
    MTLDepthStencilDescriptor, MTLDepthStencilState, MTLEvent, MTLFence, MTLFunction, MTLHeap,
    MTLHeapDescriptor, MTLIndirectCommandBuffer, MTLIndirectCommandBufferDescriptor, MTLLibrary,
    MTLRenderPipelineDescriptor, MTLRenderPipelineState, MTLSamplerDescriptor, MTLSamplerState,
    MTLSharedEvent, MTLSharedEventHandle, MTLSharedTextureHandle, MTLTexture, MTLTextureDescriptor,
};
use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use savannah::{handle, release, retain};
use std::os::raw::c_void;
use std::ptr::null;

#[link(name = "Metal", kind = "framework")]
extern "C" {
    fn MTLCreateSystemDefaultDevice() -> *mut Object;
    fn MTLCopyAllDevices() -> *mut Object;
}

#[cfg_attr(
    any(target_os = "macos", target_os = "ios"),
    link(name = "System", kind = "dylib")
)]
#[cfg_attr(
    not(any(target_os = "macos", target_os = "ios")),
    link(name = "dispatch", kind = "dylib")
)]
#[allow(improper_ctypes)]
extern "C" {
    static _dispatch_main_q: *mut Object;

    fn dispatch_data_create(
        buffer: *const c_void,
        size: usize,
        queue: *mut Object,
        destructor: *const c_void,
    ) -> *mut Object;
    fn dispatch_release(object: *mut Object);
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice?language=objc).
pub struct MTLDevice(pub *mut Object);
handle!(MTLDevice);

impl MTLDevice {
    /// [Metal docs](https://developer.apple.com/documentation/metal/1433401-mtlcreatesystemdefaultdevice?language=objc).
    pub unsafe fn create_system_default_device() -> MTLDevice {
        MTLDevice(MTLCreateSystemDefaultDevice())
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/1433367-mtlcopyalldevices?language=objc).
    pub unsafe fn copy_all_devices() -> Vec<MTLDevice> {
        let devices = MTLCopyAllDevices();
        let length: u64 = msg_send![devices, count];
        (0..length)
            .map(|index| MTLDevice(retain(msg_send![devices, objectAtIndex: index])))
            .collect::<Vec<_>>()
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/1433359-name?language=objc).
    pub unsafe fn get_name(&self) -> &str {
        let string: *mut Object = msg_send![self.0, name];
        nsstring_to_string(string)
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/1433377-headless?language=objc).
    pub unsafe fn is_headless(&self) -> bool {
        msg_send![self.0, isHeadless]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/1433409-lowpower?language=objc).
    pub unsafe fn is_low_power(&self) -> bool {
        msg_send![self.0, isLowPower]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/2889851-removable?language=objc).
    pub unsafe fn is_removable(&self) -> bool {
        msg_send![self.0, isRemovable]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/2915737-registryid?language=objc).
    pub unsafe fn get_registry_id(&self) -> u64 {
        msg_send![self.0, registryID]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/3114005-location?language=objc).
    /// [MTLDeviceLocation docs](https://developer.apple.com/documentation/metal/mtldevicelocation?language=objc).
    pub unsafe fn get_location(&self) -> u64 {
        msg_send![self.0, location]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/3114006-locationnumber?language=objc).
    pub unsafe fn get_location_number(&self) -> u64 {
        msg_send![self.0, locationNumber]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/3114007-maxtransferrate?language=objc).
    pub unsafe fn get_max_transfer_rate(&self) -> u64 {
        msg_send![self.0, maxTransferRate]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/3229025-hasunifiedmemory?language=objc).
    pub unsafe fn has_unified_memory(&self) -> bool {
        msg_send![self.0, hasUnifiedMemory]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/2967424-peergroupid?language=objc).
    pub unsafe fn get_peer_group_id(&self) -> u64 {
        msg_send![self.0, peerGroupID]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/2967423-peercount?language=objc).
    pub unsafe fn get_peer_count(&self) -> u32 {
        msg_send![self.0, peerCount]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/2967425-peerindex?language=objc).
    pub unsafe fn get_peer_index(&self) -> u32 {
        msg_send![self.0, peerIndex]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/3143473-supportsfamily?language=objc).
    /// [MTLGPUFamily docs](https://developer.apple.com/documentation/metal/mtlgpufamily?language=objc).
    pub unsafe fn supports_family(&self, family: u64) -> bool {
        msg_send![self.0, supportsFamily: family]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/1433418-supportsfeatureset?language=objc).
    /// [MTLFeatureSet docs](https://developer.apple.com/documentation/metal/mtlfeatureset?language=objc).
    pub unsafe fn supports_feature_set(&self, set: u64) -> bool {
        msg_send![self.0, supportsFeatureSet: set]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/2369280-recommendedmaxworkingsetsize?language=objc).
    pub unsafe fn get_recommended_max_working_set_size(&self) -> u64 {
        msg_send![self.0, recommendedMaxWorkingSetSize]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/2915745-currentallocatedsize?language=objc).
    pub unsafe fn get_current_allocated_size(&self) -> u64 {
        msg_send![self.0, currentAllocatedSize]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/2877429-maxthreadgroupmemorylength?language=objc).
    pub unsafe fn get_max_threadgroup_memory_length(&self) -> u64 {
        msg_send![self.0, maxThreadgroupMemoryLength]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/1433393-maxthreadsperthreadgroup?language=objc).
    pub unsafe fn get_max_threads_per_threadgroup(&self) -> (u64, u64, u64) {
        msg_send![self.0, maxThreadsPerThreadgroup]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/2866117-programmablesamplepositionssuppo?language=objc).
    pub unsafe fn are_programmable_sample_positions_supported(&self) -> bool {
        msg_send![self.0, areProgrammableSamplePositionsSupported]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/2866120-getdefaultsamplepositions?language=objc).
    pub unsafe fn get_default_sample_positions(&self, count: u64) -> (f32, f32) {
        let mut positions: (f32, f32) = (0.0, 0.0);
        let positions = (&mut positions) as *mut (f32, f32);
        let _: () = msg_send![self.0, getDefaultSamplePositions:positions count:count];
        *positions
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/1433371-depth24stencil8pixelformatsuppor?language=objc).
    pub unsafe fn is_depth24_stencil8_pixel_format_supported(&self) -> bool {
        msg_send![self.0, isDepth24Stencil8PixelFormatSupported]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/3081738-barycentriccoordssupported?language=objc).
    pub unsafe fn are_barycentric_coords_supported(&self) -> bool {
        msg_send![self.0, areBarycentricCoordsSupported]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/3197984-supportsvertexamplificationcount?language=objc).
    pub unsafe fn supports_vertex_amplification_count(&self, count: u64) -> bool {
        msg_send![self.0, supportsVertexAmplificationCount: count]
    }

    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/1433388-newcommandqueue?language=objc).
    pub unsafe fn new_command_queue(&self) -> MTLCommandQueue {
        MTLCommandQueue(retain(msg_send![self.0, newCommandQueue]))
    }

    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/1433433-newcommandqueuewithmaxcommandbuf?language=objc).
    pub unsafe fn new_command_queue_with_max_command_buffers(
        &self,
        max_command_buffers: u64,
    ) -> MTLCommandQueue {
        MTLCommandQueue(retain(msg_send![
            self.0,
            newCommandQueueWithMaxCommandBufferCount: max_command_buffers
        ]))
    }

    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/2966565-newevent?language=objc).
    pub unsafe fn new_event(&self) -> MTLEvent {
        MTLEvent(retain(msg_send![self.0, newEvent]))
    }

    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/2966569-newsharedevent?language=objc).
    pub unsafe fn new_shared_event(&self) -> MTLSharedEvent {
        MTLSharedEvent(retain(msg_send![self.0, newSharedEvent]))
    }

    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/2981024-newsharedeventwithhandle?language=objc).
    pub unsafe fn new_shared_event_with_handle(
        &self,
        handle: MTLSharedEventHandle,
    ) -> MTLSharedEvent {
        MTLSharedEvent(retain(msg_send![self.0, newSharedEventWithHandle:handle.0]))
    }

    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/1649923-newfence?language=objc).
    pub unsafe fn new_fence(&self) -> MTLFence {
        MTLFence(retain(msg_send![self.0, newFence]))
    }

    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/1433391-newlibrarywithdata?language=objc).
    pub unsafe fn new_library_with_data(&self, data: &[u8]) -> Result<MTLLibrary, &str> {
        let mut err: *mut Object = std::ptr::null_mut();

        let dispatch_data = dispatch_data_create(
            data.as_ptr() as *const c_void,
            data.len() as usize,
            &_dispatch_main_q as *const _ as *mut Object,
            null(),
        );

        let lib: *mut Object = msg_send![self.0, newLibraryWithData:dispatch_data error:&mut err];
        dispatch_release(dispatch_data);

        if !err.is_null() {
            let info: *mut Object = msg_send![err, localizedDescription];
            Err(nsstring_to_string(info))
        } else {
            Ok(MTLLibrary(lib))
        }
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/1433431-newlibrarywithsource?language=objc).
    pub unsafe fn new_library_with_source(
        &self,
        source: &str,
        options: MTLCompileOptions,
    ) -> Result<MTLLibrary, &str> {
        let mut err: *mut Object = std::ptr::null_mut();
        let source = string_to_nsstring(source);
        let lib: *mut Object =
            msg_send![self.0, newLibraryWithSource:source options:options.0 error:&mut err];

        if !err.is_null() {
            let info: *mut Object = msg_send![err, localizedDescription];
            Err(nsstring_to_string(info))
        } else {
            Ok(MTLLibrary(lib))
        }
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/1433416-newlibrarywithfile?language=objc).
    pub unsafe fn new_library_with_file(&self, path: &str) -> Result<MTLLibrary, &str> {
        let mut err: *mut Object = std::ptr::null_mut();
        let file = string_to_nsstring(path);
        let lib: *mut Object = msg_send![self.0, newLibraryWithFile:file error:&mut err];
        if !err.is_null() {
            let info: *mut Object = msg_send![err, localizedDescription];
            Err(nsstring_to_string(info))
        } else {
            Ok(MTLLibrary(lib))
        }
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/1433369-newrenderpipelinestatewithdescri?language=objc).
    pub unsafe fn new_render_pipeline_state_with_descriptor(
        &self,
        descriptor: MTLRenderPipelineDescriptor,
    ) -> Result<MTLRenderPipelineState, &str> {
        let mut err: *mut Object = std::ptr::null_mut();
        let state =
            msg_send![self.0, newRenderPipelineStateWithDescriptor:descriptor.0 error:&mut err];
        if !err.is_null() {
            let info: *mut Object = msg_send![err, localizedDescription];
            Err(nsstring_to_string(info))
        } else {
            Ok(MTLRenderPipelineState(state))
        }
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/1433395-newcomputepipelinestatewithfunct?language=objc).
    pub unsafe fn new_compute_pipeline_state_with_function(
        &self,
        function: MTLFunction,
    ) -> Result<MTLComputePipelineState, &str> {
        let mut err: *mut Object = std::ptr::null_mut();
        let state =
            msg_send![self.0, newComputePipelineStateWithFunction:function.0 error:&mut err];
        if !err.is_null() {
            let info: *mut Object = msg_send![err, localizedDescription];
            Err(nsstring_to_string(info))
        } else {
            Ok(MTLComputePipelineState(state))
        }
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/1433353-newcomputepipelinestatewithdescr?language=objc).
    pub unsafe fn new_compute_pipeline_state_with_descriptor(
        &self,
        descriptor: MTLComputePipelineDescriptor,
    ) -> Result<MTLComputePipelineState, &str> {
        let mut err: *mut Object = std::ptr::null_mut();
        let state =
            msg_send![self.0, newComputePipelineStateWithDescriptor:descriptor.0 error:&mut err];
        if !err.is_null() {
            let info: *mut Object = msg_send![err, localizedDescription];
            Err(nsstring_to_string(info))
        } else {
            Ok(MTLComputePipelineState(state))
        }
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/2966563-maxbufferlength?language=objc).
    pub unsafe fn max_buffer_length(&self) -> u64 {
        msg_send![self.0, maxBufferLength]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/1433375-newbufferwithlength?language=objc).
    /// [MTLResourceOptions docs](https://developer.apple.com/documentation/metal/mtlresourceoptions?language=objc).
    pub unsafe fn new_buffer_with_length(&self, length: u64, options: u64) -> MTLBuffer {
        MTLBuffer(msg_send![self.0, newBufferWithLength:length options:options])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/1433429-newbufferwithbytes?language=objc).
    /// [MTLResourceOptions docs](https://developer.apple.com/documentation/metal/mtlresourceoptions?language=objc).
    pub unsafe fn new_buffer_with_bytes(
        &self,
        bytes: *const c_void,
        len: u64,
        options: u64,
    ) -> MTLBuffer {
        MTLBuffer(msg_send![self.0, newBufferWithBytes:bytes length:len options:options])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/1433355-supportstexturesamplecount?language=objc).
    pub unsafe fn supports_texture_sample_count(&self, count: u64) -> bool {
        msg_send![self.0, supportsTextureSampleCount: count]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/2866126-minimumlineartexturealignmentfor?language=objc).
    /// [MTLPixelFormat docs](https://developer.apple.com/documentation/metal/mtlpixelformat?language=objc).
    pub unsafe fn minimum_linear_texture_alignment_for_pixel_format(&self, format: u64) -> u64 {
        msg_send![self.0, minimumLinearTextureAlignmentForPixelFormat: format]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/2966564-minimumtexturebufferalignmentfor?language=objc).
    /// [MTLPixelFormat docs](https://developer.apple.com/documentation/metal/mtlpixelformat?language=objc).
    pub unsafe fn minimum_texture_buffer_alignment_for_pixel_format(&self, format: u64) -> u64 {
        msg_send![self.0, minimumTextureBufferAlignmentForPixelFormat: format]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/2887289-readwritetexturesupport?language=objc).
    /// [MTLReadWriteTextureTier](https://developer.apple.com/documentation/metal/mtlreadwritetexturetier?language=objc).
    pub unsafe fn read_write_texture_support(&self) -> u64 {
        msg_send![self.0, readWriteTextureSupport]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/1433425-newtexturewithdescriptor?language=objc).
    pub unsafe fn new_texture_with_descriptor(
        &self,
        descriptor: MTLTextureDescriptor,
    ) -> MTLTexture {
        MTLTexture(msg_send![self.0, newTextureWithDescriptor:descriptor.0])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/2967421-newsharedtexturewithdescriptor?language=objc).
    pub unsafe fn new_shared_texture_with_descriptor(
        &self,
        descriptor: MTLTextureDescriptor,
    ) -> MTLTexture {
        MTLTexture(msg_send![self.0, newSharedTextureWithDescriptor:descriptor.0])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/2967422-newsharedtexturewithhandle?language=objc).
    pub unsafe fn new_shared_texture_with_handle(
        &self,
        handle: MTLSharedTextureHandle,
    ) -> MTLTexture {
        MTLTexture(msg_send![self.0, newSharedTextureWithHandle:handle.0])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/1433408-newsamplerstatewithdescriptor?language=objc).
    pub unsafe fn new_sampler_state_with_descriptor(
        &self,
        descriptor: MTLSamplerDescriptor,
    ) -> MTLSamplerState {
        MTLSamplerState(msg_send![self.0, newSamplerStateWithDescriptor:descriptor.0])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/2915742-argumentbufferssupport?language=objc).
    /// [MTLArgumentBuffersTier docs](https://developer.apple.com/documentation/metal/mtlargumentbufferstier?language=objc).
    pub unsafe fn get_argument_buffers_support(&self) -> u64 {
        msg_send![self.0, argumentBuffersSupport]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/2977322-maxargumentbuffersamplercount?language=objc).
    pub unsafe fn get_max_argument_buffer_sampler_count(&self) -> u64 {
        msg_send![self.0, maxArgumentBufferSamplerCount]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/2915744-newargumentencoderwitharguments?language=objc).
    pub unsafe fn new_argument_encoder_with_arguments(
        &self,
        descriptors: &[MTLArgumentDescriptor],
    ) -> MTLArgumentEncoder {
        MTLArgumentEncoder({
            let arr: *mut Object = msg_send![class!(NSMutableArray), new];
            descriptors.iter().for_each(|desc| {
                let _: () = msg_send![arr, addObject:desc.0];
            });
            msg_send![self.0, newArgumentEncoderWithArguments: arr]
        })
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/2967420-newindirectcommandbufferwithdesc?language=objc).
    /// [MTLResourceOptions docs](https://developer.apple.com/documentation/metal/mtlresourceoptions?language=objc).
    pub unsafe fn new_indirect_command_buffer_with_descriptor(
        &self,
        descriptor: MTLIndirectCommandBufferDescriptor,
        max_command_count: u64,
        res_options: u64,
    ) -> MTLIndirectCommandBuffer {
        MTLIndirectCommandBuffer(retain(msg_send![
            self.0,
            newIndirectCommandBufferWithDescriptor:descriptor.0
            maxCommandCount:max_command_count
            options:res_options
        ]))
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/1649928-newheapwithdescriptor?language=objc).
    pub unsafe fn new_heap_with_descriptor(&self, descriptor: MTLHeapDescriptor) -> MTLHeap {
        MTLHeap(msg_send![self.0, newHeapWithDescriptor:descriptor.0])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldevice/1433412-newdepthstencilstatewithdescript?language=objc).
    pub unsafe fn new_depth_stencil_state(
        &self,
        descriptor: MTLDepthStencilDescriptor,
    ) -> MTLDepthStencilState {
        MTLDepthStencilState(retain(
            msg_send![self.0, newDepthStencilStateWithDescriptor:descriptor.0],
        ))
    }
    // TODO add the rest of the behavior
}
