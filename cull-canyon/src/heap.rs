use crate::{MTLBuffer, MTLTexture, MTLTextureDescriptor};
use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use savannah::{handle, release, retain};

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlheapdescriptor?language=objc).
pub struct MTLHeapDescriptor(pub *mut Object);
handle!(MTLHeapDescriptor);

impl MTLHeapDescriptor {
    pub unsafe fn new() -> MTLHeapDescriptor {
        MTLHeapDescriptor(msg_send![class!(MTLHeapDescriptor), new])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlheapdescriptor/3043389-type?language=objc).
    /// [MTLHeapType docs](https://developer.apple.com/documentation/metal/mtlheaptype?language=objc).
    pub unsafe fn set_type(&self, heap_type: u64) {
        let _: () = msg_send![self.0, setType: heap_type];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlheapdescriptor/1649567-storagemode?language=objc).
    /// [MTLStorageMode docs](https://developer.apple.com/documentation/metal/mtlstoragemode?language=objc).
    pub unsafe fn set_storage_mode(&self, mode: u64) {
        let _: () = msg_send![self.0, setStorageMode: mode];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlheapdescriptor/1649573-cpucachemode?language=objc).
    /// [MTLCPUCacheMode docs](https://developer.apple.com/documentation/metal/mtlcpucachemode?language=objc).
    pub unsafe fn set_cpu_cache_mode(&self, mode: u64) {
        let _: () = msg_send![self.0, setCpuCacheMode: mode];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlheapdescriptor/3131686-hazardtrackingmode?language=objc).
    /// [MTLHazardTrackingMode docs](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode?language=objc).
    pub unsafe fn set_hazard_tracking_mode(&self, mode: u64) {
        let _: () = msg_send![self.0, setHazardTrackingMode: mode];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlheapdescriptor/3131687-resourceoptions?language=objc).
    /// [MTLResourceOptions docs](https://developer.apple.com/documentation/metal/mtlresourceoptions?language=objc).
    pub unsafe fn set_resource_options(&self, options: u64) {
        let _: () = msg_send![self.0, setResourceOptions: options];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlheapdescriptor/1649568-size?language=objc).
    pub unsafe fn set_size(&self, size: u64) {
        let _: () = msg_send![self.0, setSize: size];
    }
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlheap?language=objc).
pub struct MTLHeap(pub *mut Object);
handle!(MTLHeap);

impl MTLHeap {
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlheapdescriptor/3043389-type?language=objc).
    /// [MTLHeapType docs](https://developer.apple.com/documentation/metal/mtlheaptype?language=objc).
    pub unsafe fn get_type(&self) -> u64 {
        msg_send![self.0, type]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlheapdescriptor/1649567-storagemode?language=objc).
    /// [MTLStorageMode docs](https://developer.apple.com/documentation/metal/mtlstoragemode?language=objc).
    pub unsafe fn get_storage_mode(&self) -> u64 {
        msg_send![self.0, storageMode]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlheapdescriptor/1649573-cpucachemode?language=objc).
    /// [MTLCPUCacheMode docs](https://developer.apple.com/documentation/metal/mtlcpucachemode?language=objc).
    pub unsafe fn get_cpu_cache_mode(&self) -> u64 {
        msg_send![self.0, cpuCacheMode]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlheapdescriptor/3131686-hazardtrackingmode?language=objc).
    /// [MTLHazardTrackingMode docs](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode?language=objc).
    pub unsafe fn get_hazard_tracking_mode(&self) -> u64 {
        msg_send![self.0, hazardTrackingMode]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlheapdescriptor/3131687-resourceoptions?language=objc).
    /// [MTLResourceOptions docs](https://developer.apple.com/documentation/metal/mtlresourceoptions?language=objc).
    pub unsafe fn get_resource_options(&self) -> u64 {
        msg_send![self.0, resourceOptions]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlheapdescriptor/1649568-size?language=objc).
    pub unsafe fn get_size(&self) -> u64 {
        msg_send![self.0, size]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlheap/2097557-usedsize?language=objc).
    pub unsafe fn get_used_size(&self) -> u64 {
        msg_send![self.0, usedSize]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlheap/2915348-currentallocatedsize?language=objc).
    pub unsafe fn get_current_allocated_size(&self) -> u64 {
        msg_send![self.0, currentAllocatedSize]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlheap/1771284-maxavailablesizewithalignment?language=objc).
    pub unsafe fn get_max_available_size_with_alignment(&self, alignment: u64) -> u64 {
        msg_send![self.0, maxAvailableSizeWithAlignment: alignment]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlheap/1771281-setpurgeablestate?language=objc).
    /// [MTLPurgeableState docs](https://developer.apple.com/documentation/metal/mtlpurgeablestate?language=objc).
    pub unsafe fn set_purgeable_state(&self, state: u64) {
        let _: () = msg_send![self.0, setPurgeableState: state];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlheap/1649571-newbufferwithlength?language=objc).
    /// [Other Metal docs](https://developer.apple.com/documentation/metal/mtlheap/3152522-newbufferwithlength?language=objc).
    /// [MTLResourceOptions docs](https://developer.apple.com/documentation/metal/mtlresourceoptions?language=objc).
    pub unsafe fn new_buffer_with_length(
        &self,
        length: u64,
        options: u64,
        offset: Option<u64>,
    ) -> Option<MTLBuffer> {
        let buffer: *mut Object = match offset {
            None => msg_send![self.0, newBufferWithLength:length options:options],
            Some(offset) => msg_send![
                self.0, newBufferWithLength:length options:options offset:offset
            ],
        };
        match buffer.is_null() {
            true => None,
            false => Some(MTLBuffer(buffer)),
        }
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlheap/1649574-newtexturewithdescriptor?language=objc).
    /// [Other Metal docs](https://developer.apple.com/documentation/metal/mtlheap/3152523-newtexturewithdescriptor?language=objc).
    pub unsafe fn new_texture_with_descriptor(
        &self,
        descriptor: MTLTextureDescriptor,
        offset: Option<u64>,
    ) -> Option<MTLTexture> {
        let texture: *mut Object = match offset {
            None => msg_send![self.0, newTextureWithDescriptor:descriptor.0],
            Some(offset) => msg_send![self.0, newTextureWithDescriptor:descriptor.0 offset:offset],
        };
        match texture.is_null() {
            true => None,
            false => Some(MTLTexture(texture)),
        }
    }
}
