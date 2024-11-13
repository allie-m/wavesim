use crate::{MTLBuffer, MTLDevice, MTLRegion, MTLResource};
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use savannah::{handle, release, retain};
use std::ops::Range;
use std::os::raw::c_void;

/// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor?language=objc).
pub struct MTLTextureDescriptor(pub *mut Object, bool);
impl Clone for MTLTextureDescriptor {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) }, true)
    }
}
impl Drop for MTLTextureDescriptor {
    fn drop(&mut self) {
        if self.1 {
            unsafe { release(self.0) }
        }
    }
}

impl MTLTextureDescriptor {
    pub unsafe fn new() -> MTLTextureDescriptor {
        MTLTextureDescriptor(msg_send![class!(MTLTextureDescriptor), new], true)
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1515511-texture2ddescriptorwithpixelform?language=objc).
    /// [MTLPixelFormat docs](https://developer.apple.com/documentation/metal/mtlpixelformat?language=objc).
    pub unsafe fn texture_2d_descriptor_with_pixel_format(
        size: (u64, u64),
        mipmapped: bool,
        format: u64,
    ) -> MTLTextureDescriptor {
        MTLTextureDescriptor(
            msg_send![
                class!(MTLTextureDescriptor),
                texture2DDescriptorWithPixelFormat:format
                width:size.0 height:size.1 mipmapped:mipmapped
            ],
            false,
        )
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1516090-texturecubedescriptorwithpixelfo?language=objc).
    /// [MTLPixelFormat docs](https://developer.apple.com/documentation/metal/mtlpixelformat?language=objc).
    pub unsafe fn texture_cube_descriptor_with_pixel_format(
        size: u64,
        mipmapped: bool,
        format: u64,
    ) -> MTLTextureDescriptor {
        MTLTextureDescriptor(
            msg_send![
                class!(MTLTextureDescriptor),
                textureCubeDescriptorWithPixelFormat:format
                size:size mipmapped:mipmapped
            ],
            false,
        )
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/2966642-texturebufferdescriptorwithpixel?language=objc).
    /// [MTLPixelFormat docs](https://developer.apple.com/documentation/metal/mtlpixelformat?language=objc).
    /// [MTLResourceOptions docs](https://developer.apple.com/documentation/metal/mtlresourceoptions?language=objc).
    /// [MTLTextureUsage docs](https://developer.apple.com/documentation/metal/mtltextureusage?language=objc).
    pub unsafe fn texture_buffer_descriptor_with_pixel_format(
        width: u64,
        format: u64,
        options: u64,
        usage: u64,
    ) -> MTLTextureDescriptor {
        MTLTextureDescriptor(
            msg_send![
                class!(MTLTextureDescriptor),
                textureBufferDescriptorWithPixelFormat:format
                width:width
                resourceOptions:options
                usage:usage
            ],
            false,
        )
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1516228-texturetype?language=objc).
    /// [MTLTextureType docs](https://developer.apple.com/documentation/metal/mtltexturetype?language=objc).
    pub unsafe fn set_texture_type(&self, texture_type: u64) {
        let _: () = msg_send![self.0, setTextureType: texture_type];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1516228-texturetype?language=objc).
    /// [MTLTextureType docs](https://developer.apple.com/documentation/metal/mtltexturetype?language=objc).
    pub unsafe fn get_texture_type(&self) -> u64 {
        msg_send![self.0, textureType]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1515450-pixelformat?language=objc).
    /// [MTLPixelFormat docs](https://developer.apple.com/documentation/metal/mtlpixelformat?language=objc).
    pub unsafe fn set_pixel_format(&self, format: u64) {
        let _: () = msg_send![self.0, setPixelFormat: format];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1515450-pixelformat?language=objc).
    /// [MTLPixelFormat docs](https://developer.apple.com/documentation/metal/mtlpixelformat?language=objc).
    pub unsafe fn get_pixel_format(&self) -> u64 {
        msg_send![self.0, pixelFormat]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1515649-width?language=objc).
    pub unsafe fn set_width(&self, width: u64) {
        let _: () = msg_send![self.0, setWidth: width];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1515649-width?language=objc).
    pub unsafe fn get_width(&self) -> u64 {
        msg_send![self.0, width]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1516000-height?language=objc).
    pub unsafe fn set_height(&self, height: u64) {
        let _: () = msg_send![self.0, setHeight: height];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1516000-height?language=objc).
    pub unsafe fn get_height(&self) -> u64 {
        msg_send![self.0, height]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1516298-depth?language=objc).
    pub unsafe fn set_depth(&self, depth: u64) {
        let _: () = msg_send![self.0, setDepth: depth];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1516298-depth?language=objc).
    pub unsafe fn get_depth(&self) -> u64 {
        msg_send![self.0, depth]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1516300-mipmaplevelcount?language=objc).
    pub unsafe fn set_mipmap_level_count(&self, count: u64) {
        let _: () = msg_send![self.0, setMipmapLevelCount: count];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1516300-mipmaplevelcount?language=objc).
    pub unsafe fn get_mipmap_level_count(&self) -> u64 {
        msg_send![self.0, mipmapLevelCount]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1516260-samplecount?language=objc).
    pub unsafe fn set_sample_count(&self, count: u64) {
        let _: () = msg_send![self.0, setSampleCount: count];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1516260-samplecount?language=objc).
    pub unsafe fn get_sample_count(&self) -> u64 {
        msg_send![self.0, sampleCount]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1515331-arraylength?language=objc).
    pub unsafe fn set_array_length(&self, length: u64) {
        let _: () = msg_send![self.0, setArrayLength: length];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1515331-arraylength?language=objc).
    pub unsafe fn get_array_length(&self) -> u64 {
        msg_send![self.0, arrayLength]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1515776-resourceoptions?language=objc).
    /// [MTLResourceOptions docs](https://developer.apple.com/documentation/metal/mtlresourceoptions?language=objc).
    pub unsafe fn set_resource_options(&self, options: u64) {
        let _: () = msg_send![self.0, setResourceOptions: options];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1515776-resourceoptions?language=objc).
    /// [MTLResourceOptions docs](https://developer.apple.com/documentation/metal/mtlresourceoptions?language=objc).
    pub unsafe fn get_resource_options(&self) -> u64 {
        msg_send![self.0, resourceOptions]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1515375-cpucachemode?language=objc).
    /// [MTLCPUCacheMode docs](https://developer.apple.com/documentation/metal/mtlcpucachemode?language=objc).
    pub unsafe fn set_cpu_cache_mode(&self, mode: u64) {
        let _: () = msg_send![self.0, setCpuCacheMode: mode];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1515375-cpucachemode?language=objc).
    /// [MTLCPUCacheMode docs](https://developer.apple.com/documentation/metal/mtlcpucachemode?language=objc).
    pub unsafe fn get_cpu_cache_mode(&self) -> u64 {
        msg_send![self.0, cpuCacheMode]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1516262-storagemode?language=objc).
    /// [MTLStorageMode docs](https://developer.apple.com/documentation/metal/mtlstoragemode?language=objc).
    pub unsafe fn set_storage_mode(&self, mode: u64) {
        let _: () = msg_send![self.0, setStorageMode: mode];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1516262-storagemode?language=objc).
    /// [MTLStorageMode docs](https://developer.apple.com/documentation/metal/mtlstoragemode?language=objc).
    pub unsafe fn get_storage_mode(&self) -> u64 {
        msg_send![self.0, storageMode]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/3131697-hazardtrackingmode?language=objc).
    /// [MTLHazardTracking docs](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode?language=objc).
    pub unsafe fn set_hazard_tracking_mode(&self, mode: u64) {
        let _: () = msg_send![self.0, setHazardTrackingMode: mode];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/3131697-hazardtrackingmode?language=objc).
    /// [MTLHazardTracking docs](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode?language=objc).
    pub unsafe fn get_hazard_tracking_mode(&self) -> u64 {
        msg_send![self.0, hazardTrackingMode]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/2966641-allowgpuoptimizedcontents?language=objc).
    pub unsafe fn set_allow_gpu_optimized_contents(&self, allow: bool) {
        let _: () = msg_send![self.0, setAllowGPUOptimizedContents: allow];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/2966641-allowgpuoptimizedcontents?language=objc).
    pub unsafe fn get_allow_gpu_optimized_contents(&self) -> bool {
        msg_send![self.0, allowGPUOptimizedContents]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1515783-usage?language=objc).
    /// [MTLTextureUsage docs](https://developer.apple.com/documentation/metal/mtltextureusage?language=objc).
    pub unsafe fn set_usage(&self, usage: u64) {
        let _: () = msg_send![self.0, setUsage: usage];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1515783-usage?language=objc).
    /// [MTLTextureUsage docs](https://developer.apple.com/documentation/metal/mtltextureusage?language=objc).
    pub unsafe fn get_usage(&self) -> u64 {
        msg_send![self.0, usage]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/3114305-swizzle?language=objc).
    /// [MTLTextureSwizzleChannels docs](https://developer.apple.com/documentation/metal/mtltextureswizzlechannels?language=objc).
    /// [MTLTextureSwizzle docs](https://developer.apple.com/documentation/metal/mtltextureswizzle?language=objc).
    pub unsafe fn set_swizzle(&self, swizzle: (u8, u8, u8, u8)) {
        let _: () = msg_send![self.0, setSwizzle: swizzle];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexturedescriptor/3114305-swizzle?language=objc).
    /// [MTLTextureSwizzleChannels docs](https://developer.apple.com/documentation/metal/mtltextureswizzlechannels?language=objc).
    /// [MTLTextureSwizzle docs](https://developer.apple.com/documentation/metal/mtltextureswizzle?language=objc).
    pub unsafe fn get_swizzle(&self) -> (u8, u8, u8, u8) {
        msg_send![self.0, swizzle]
    }
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture?language=objc).
pub struct MTLTexture(pub *mut Object);
handle!(MTLTexture);

impl MTLResource for MTLTexture {
    fn get_obj(&self) -> *mut Object {
        self.0
    }
}

impl MTLTexture {
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/1515679-replaceregion?language=objc).
    pub unsafe fn replace_region_arrayed(
        &self,
        region: (u64, u64, u64, u64),
        mipmap_level: u64,
        slice: u64,
        bytes: *mut c_void,
        bytes_per_row: u64,
        bytes_per_image: u64,
    ) {
        let _: () = msg_send![
            self.0,
            replaceRegion:region
            mipmapLevel:mipmap_level
            slice:slice
            withBytes:bytes
            bytesPerRow:bytes_per_row
            bytesPerImage:bytes_per_image
        ];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/1515464-replaceregion?language=objc).
    pub unsafe fn replace_region(
        &self,
        region: (u64, u64, u64, u64),
        mipmap_level: u64,
        bytes: *mut c_void,
        bytes_per_row: u64,
    ) {
        let k = MTLRegion {
            x: region.0,
            y: region.1,
            z: 0,
            width: region.2,
            height: region.3,
            depth: 1,
        };
        let _: () = msg_send![
            self.0,
            replaceRegion:k
            mipmapLevel:mipmap_level
            withBytes:bytes
            bytesPerRow:bytes_per_row
        ];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/1516318-getbytes?language=objc).
    pub unsafe fn get_bytes_arrayed(
        &self,
        bytes: *mut c_void,
        bytes_per_row: u64,
        bytes_per_image: u64,
        region: (u64, u64, u64, u64, u64, u64),
        mipmap_level: u64,
        slice: u64,
    ) {
        let _: () = msg_send![
            self.0,
            getBytes:bytes
            bytesPerRow:bytes_per_row
            bytesPerImage:bytes_per_image
            fromRegion:region
            mipmapLevel:mipmap_level
            slice:slice
        ];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/1515751-getbytes?language=objc).
    pub unsafe fn get_bytes(
        &self,
        bytes: *mut c_void,
        bytes_per_row: u64,
        region: (u64, u64, u64, u64),
        mipmap_level: u64,
    ) {
        let k = MTLRegion {
            x: region.0,
            y: region.1,
            z: 0,
            width: region.2,
            height: region.3,
            depth: 1,
        };
        let _: () = msg_send![
            self.0,
            getBytes:bytes
            bytesPerRow:bytes_per_row
            fromRegion:k
            mipmapLevel:mipmap_level
        ];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/1515598-newtextureviewwithpixelformat?language=objc).
    /// [MTLPixelFormat docs](https://developer.apple.com/documentation/metal/mtlpixelformat?language=objc).
    pub unsafe fn new_texture_view_with_pixel_format(&self, format: u64) -> MTLTexture {
        MTLTexture(msg_send![self.0, newTextureViewWithPixelFormat: format])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/1515409-newtextureviewwithpixelformat?language=objc).
    /// [MTLPixelFormat docs](https://developer.apple.com/documentation/metal/mtlpixelformat?language=objc).
    /// [MTLTextureType docs](https://developer.apple.com/documentation/metal/mtltexturetype?language=objc).
    pub unsafe fn new_texture_view_with_pixel_format_arrayed(
        &self,
        format: u64,
        texture_type: u64,
        levels: Range<u64>,
        slices: Range<u64>,
    ) -> MTLTexture {
        MTLTexture(msg_send![
            self.0,
            newTextureViewWithPixelFormat:format
            textureType:texture_type
            levels:(levels.start, levels.end)
            slices:(slices.start, slices.end)
        ])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/1515409-newtextureviewwithpixelformat?language=objc).
    /// [MTLPixelFormat docs](https://developer.apple.com/documentation/metal/mtlpixelformat?language=objc).
    /// [MTLTextureType docs](https://developer.apple.com/documentation/metal/mtltexturetype?language=objc).
    /// [MTLTextureSwizzleChannels docs](https://developer.apple.com/documentation/metal/mtltextureswizzlechannels?language=objc).
    /// [MTLTextureSwizzle docs](https://developer.apple.com/documentation/metal/mtltextureswizzle?language=objc).
    pub unsafe fn new_texture_view_with_pixel_format_arrayed_swizzle(
        &self,
        format: u64,
        texture_type: u64,
        levels: Range<u64>,
        slices: Range<u64>,
        swizzle: (u64, u64, u64, u64),
    ) -> MTLTexture {
        MTLTexture(msg_send![
            self.0,
            newTextureViewWithPixelFormat:format
            textureType:texture_type
            levels:(levels.start, levels.end)
            slices:(slices.start, slices.end)
            swizzle:swizzle
        ])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/1515517-texturetype?language=objc).
    /// [MTLTextureType docs](https://developer.apple.com/documentation/metal/mtltexturetype?language=objc).
    pub unsafe fn get_texture_type(&self) -> u64 {
        msg_send![self.0, textureType]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/1515344-pixelformat?language=objc).
    /// [MTLPixelFormat docs](https://developer.apple.com/documentation/metal/mtlpixelformat?language=objc).
    pub unsafe fn get_pixel_format(&self) -> u64 {
        msg_send![self.0, pixelFormat]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/1515339-width?language=objc).
    pub unsafe fn get_width(&self) -> u64 {
        msg_send![self.0, width]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/1515938-height?language=objc).
    pub unsafe fn get_height(&self) -> u64 {
        msg_send![self.0, height]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/1515942-depth?language=objc).
    pub unsafe fn get_depth(&self) -> u64 {
        msg_send![self.0, depth]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/1515677-mipmaplevelcount?language=objc).
    pub unsafe fn get_mipmap_level_count(&self) -> u64 {
        msg_send![self.0, mipmapLevelCount]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/1515382-arraylength?language=objc).
    pub unsafe fn get_array_length(&self) -> u64 {
        msg_send![self.0, arrayLength]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/1515443-samplecount?language=objc).
    pub unsafe fn get_sample_count(&self) -> u64 {
        msg_send![self.0, sampleCount]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/1515749-framebufferonly?language=objc).
    pub unsafe fn is_framebuffer_only(&self) -> bool {
        msg_send![self.0, isFramebufferOnly]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/1515763-usage?language=objc).
    /// [MTLTextureUsage docs](https://developer.apple.com/documentation/metal/mtltextureusage?language=objc).
    pub unsafe fn get_usage(&self) -> u64 {
        msg_send![self.0, usage]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/2966640-allowgpuoptimizedcontents?language=objc).
    pub unsafe fn does_allow_gpu_optimized_contents(&self) -> bool {
        msg_send![self.0, allowGPUOptimizedContents]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/2998889-shareable?language=objc).
    pub unsafe fn is_shareable(&self) -> bool {
        msg_send![self.0, isShareable]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/3114304-swizzle?language=objc).
    /// [MTLTextureSwizzleChannels docs](https://developer.apple.com/documentation/metal/mtltextureswizzlechannels?language=objc).
    /// [MTLTextureSwizzle docs](https://developer.apple.com/documentation/metal/mtltextureswizzle?language=objc).
    pub unsafe fn get_swizzle(&self) -> (u64, u64, u64, u64) {
        msg_send![self.0, swizzle]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/1515372-parenttexture?language=objc).
    pub unsafe fn get_parent_texture(&self) -> Option<MTLTexture> {
        let t: *mut Object = msg_send![self.0, parentTexture];
        match t.is_null() {
            true => None,
            false => Some(MTLTexture(t)),
        }
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/1516265-parentrelativelevel?language=objc).
    pub unsafe fn get_parent_relative_level(&self) -> u64 {
        msg_send![self.0, parentRelativeLevel]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/1516221-parentrelativeslice?language=objc).
    pub unsafe fn get_parent_relative_slice(&self) -> u64 {
        msg_send![self.0, parentRelativeSlice]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/1619090-buffer?language=objc).
    pub unsafe fn get_buffer(&self) -> Option<MTLBuffer> {
        let b: *mut Object = msg_send![self.0, buffer];
        match b.is_null() {
            true => None,
            false => Some(MTLBuffer(b)),
        }
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/1619019-bufferoffset?language=objc).
    pub unsafe fn get_buffer_offset(&self) -> u64 {
        msg_send![self.0, bufferOffset]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/1619175-bufferbytesperrow?language=objc).
    pub unsafe fn get_buffer_bytes_per_row(&self) -> u64 {
        msg_send![self.0, bufferBytesPerRow]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/2981032-newsharedtexturehandle?language=objc).
    pub unsafe fn new_shared_texture_handle(&self) -> Option<MTLSharedTextureHandle> {
        let h: *mut Object = msg_send![self.0, newSharedTextureHandle];
        match h.is_null() {
            true => None,
            false => Some(MTLSharedTextureHandle(h)),
        }
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/2967449-newremotetextureviewfordevice?language=objc).
    pub unsafe fn new_remote_texture_view_for_device(&self, device: MTLDevice) -> MTLTexture {
        MTLTexture(msg_send![self.0, newRemoteTextureViewForDevice: device])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/2967451-remotestoragetexture?language=objc).
    pub unsafe fn get_remote_storage_texture(&self) -> Option<MTLTexture> {
        let s: *mut Object = msg_send![self.0, remoteStorageTexture];
        match s.is_null() {
            true => None,
            false => Some(MTLTexture(s)),
        }
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/3153124-issparse?language=objc).
    pub unsafe fn is_sparse(&self) -> bool {
        msg_send![self.0, isSparse]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/3043999-firstmipmapintail?language=objc).
    pub unsafe fn first_mipmap_in_tail(&self) -> u64 {
        msg_send![self.0, firstMipmapInTail]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtltexture/3044002-tailsizeinbytes?language=objc).
    pub unsafe fn tail_size_in_bytes(&self) -> u64 {
        msg_send![self.0, tailSizeInBytes]
    }
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlsharedtexturehandle?language=objc).
pub struct MTLSharedTextureHandle(pub *mut Object);
handle!(MTLSharedTextureHandle);
