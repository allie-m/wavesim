use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use savannah::{handle, release, retain};

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor?language=objc).
pub struct MTLSamplerDescriptor(pub *mut Object);
handle!(MTLSamplerDescriptor);

impl MTLSamplerDescriptor {
    pub unsafe fn new() -> MTLSamplerDescriptor {
        MTLSamplerDescriptor(msg_send![class!(MTLSamplerDescriptor), new])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/1516289-normalizedcoordinates?language=objc).
    pub unsafe fn set_normalized_coords(&self, normalized: bool) {
        let _: () = msg_send![self.0, setNormalizedCoordinates: normalized];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/1515466-raddressmode?language=objc).
    /// [MTLSamplerAddressMode docs](https://developer.apple.com/documentation/metal/mtlsampleraddressmode?language=objc).
    pub unsafe fn set_r_address_mode(&self, address_mode: u64) {
        let _: () = msg_send![self.0, setRAddressMode: address_mode];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/1515779-saddressmode?language=objc).
    /// [MTLSamplerAddressMode docs](https://developer.apple.com/documentation/metal/mtlsampleraddressmode?language=objc).
    pub unsafe fn set_s_address_mode(&self, address_mode: u64) {
        let _: () = msg_send![self.0, setSAddressMode: address_mode];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/1515900-taddressmode?language=objc).
    /// [MTLSamplerAddressMode docs](https://developer.apple.com/documentation/metal/mtlsampleraddressmode?language=objc).
    pub unsafe fn set_t_address_mode(&self, address_mode: u64) {
        let _: () = msg_send![self.0, setTAddressMode: address_mode];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/2092299-bordercolor?language=objc).
    /// [MTLSamplerBorderColor docs](https://developer.apple.com/documentation/metal/mtlsamplerbordercolor?language=objc).
    pub unsafe fn set_border_color(&self, color: u64) {
        let _: () = msg_send![self.0, setBorderColor: color];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/1515792-minfilter?language=objc).
    /// [MTLSamplerMinMagFilter docs](https://developer.apple.com/documentation/metal/mtlsamplerminmagfilter?language=objc).
    pub unsafe fn set_min_filter(&self, filter: u64) {
        let _: () = msg_send![self.0, setMinFilter: filter];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/1515926-magfilter?language=objc).
    /// [MTLSamplerMinMagFilter docs](https://developer.apple.com/documentation/metal/mtlsamplerminmagfilter?language=objc).
    pub unsafe fn set_mag_filter(&self, filter: u64) {
        let _: () = msg_send![self.0, setMagFilter: filter];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/1515553-mipfilter?language=objc).
    /// [MTLSamplerMinMagFilter docs](https://developer.apple.com/documentation/metal/mtlsamplerminmagfilter?language=objc).
    pub unsafe fn set_mip_filter(&self, filter: u64) {
        let _: () = msg_send![self.0, setMipFilter: filter];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/1515629-lodminclamp?language=objc).
    pub unsafe fn set_lod_min_clamp(&self, clamp: f32) {
        let _: () = msg_send![self.0, setLodMinClamp: clamp];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/1516234-lodmaxclamp?language=objc).
    pub unsafe fn set_lod_max_clamp(&self, clamp: f32) {
        let _: () = msg_send![self.0, setLodMaxClamp: clamp];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/1615844-lodaverage?language=objc).
    pub unsafe fn set_lod_average_enabled(&self, average: bool) {
        let _: () = msg_send![self.0, setLodAverage: average];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/1516164-maxanisotropy?language=objc).
    pub unsafe fn set_max_anisotropy(&self, max: u64) {
        let _: () = msg_send![self.0, setMaxAnisotropy: max];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/1516001-comparefunction?language=objc).
    /// [MTLCompareFunction docs](https://developer.apple.com/documentation/metal/mtlcomparefunction?language=objc).
    pub unsafe fn set_compare_function(&self, function: u64) {
        let _: () = msg_send![self.0, setCompareFunction: function];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/2915782-supportargumentbuffers?language=objc).
    pub unsafe fn set_support_argument_buffers(&self, support: bool) {
        let _: () = msg_send![self.0, setSupportArgumentBuffers: support];
    }
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlsamplerstate?language=objc).
pub struct MTLSamplerState(pub *mut Object);
handle!(MTLSamplerState);
