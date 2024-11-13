use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};
// use savannah::{release, retain, handle};

pub trait MTLResource {
    #[doc(hidden)]
    fn get_obj(&self) -> *mut Object;
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlresource/1516127-cpucachemode?language=objc).
    /// [MTLCPUCacheMode docs](https://developer.apple.com/documentation/metal/mtlcpucachemode?language=objc).
    unsafe fn get_cpu_cache_mode(&self) -> u64 {
        msg_send![self.get_obj(), cpuCacheMode]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlresource/1515477-storagemode?language=objc).
    /// [MTLStorageMode docs](https://developer.apple.com/documentation/metal/mtlstoragemode?language=objc).
    unsafe fn get_storage_mode(&self) -> u64 {
        msg_send![self.get_obj(), storageMode]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlresource/3131693-hazardtrackingmode?language=objc).
    /// [MTLHazardTrackingMode docs](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode?language=objc).
    unsafe fn get_hazard_tracking_mode(&self) -> u64 {
        msg_send![self.get_obj(), hazardTrackingMode]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlresource/3131694-resourceoptions?language=objc).
    /// [MTLResourceOptions docs](https://developer.apple.com/documentation/metal/mtlresourceoptions?language=objc).
    unsafe fn get_resource_options(&self) -> u64 {
        msg_send![self.get_obj(), resourceOptions]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlresource/1515898-setpurgeablestate?language=objc).
    /// [MTLPurgeableState docs](https://developer.apple.com/documentation/metal/mtlpurgeablestate?language=objc).
    unsafe fn set_purgeable_state(&self, state: u64) {
        let _: () = msg_send![self.get_obj(), setPurgeableState: state];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlresource/3043406-heapoffset?language=objc).
    unsafe fn get_heap_offset(&self) -> u64 {
        msg_send![self.get_obj(), heapOffset]
    }
    // unsafe fn get_heap(&self) -> MTLHeap {} // TODO implement this later
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlresource/1771705-makealiasable?language=objc).
    unsafe fn make_aliasable(&self) {
        let _: () = msg_send![self.get_obj(), makeAliasable];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlresource/1771702-isaliasable?language=objc).
    unsafe fn is_aliasable(&self) -> bool {
        msg_send![self.get_obj(), isAliasable]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlresource/2915287-allocatedsize?language=objc).
    unsafe fn get_allocated_size(&self) -> u64 {
        msg_send![self.get_obj(), allocatedSize]
    }
}
