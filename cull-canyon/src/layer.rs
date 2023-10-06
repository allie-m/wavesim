use crate::MTLDevice;
use crate::MTLTexture;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use savannah::{handle, release, retain};

/// Takes a raw window handle and sets its layer to be a CAMetalLayer.
///
/// # Panics
///
/// Panics if the `handle.get_raw_window_handle()` is not of the `MacOS` variant.
///
/// # Docs
///
/// [NSView docs](https://developer.apple.com/documentation/appkit/nsview?language=objc).
pub unsafe fn set_layer_for_raw_window_handle<H: HasRawWindowHandle>(
    layer: CAMetalLayer,
    handle: &H,
) {
    match handle.raw_window_handle() {
        RawWindowHandle::MacOS(handle) => {
            let _: () = msg_send![handle.ns_view as *mut Object, setWantsLayer:true];
            let _: () = msg_send![handle.ns_view as *mut Object, setLayer:layer.0];
        }
        _ => panic!("This isn't a macOS surface."),
    }
}

/// [Metal docs](https://developer.apple.com/documentation/quartzcore/cametaldrawable?language=objc).
pub struct CAMetalDrawable(pub *mut Object);
handle!(CAMetalDrawable);

impl CAMetalDrawable {
    /// [Metal docs](https://developer.apple.com/documentation/quartzcore/cametaldrawable/1478159-texture?language=objc).
    pub unsafe fn get_texture(&self) -> MTLTexture {
        MTLTexture({
            let k: *mut Object = msg_send![self.0, texture];
            let _: () = msg_send![k, retain];
            k
        })
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtldrawable/1470284-present?language=objc).
    pub unsafe fn present(&self) {
        let _: () = msg_send![self.0, present];
    }
}

/// [Metal docs](https://developer.apple.com/documentation/quartzcore/cametallayer?language=objc).
pub struct CAMetalLayer(pub *mut Object);
handle!(CAMetalLayer);

impl CAMetalLayer {
    pub unsafe fn new() -> CAMetalLayer {
        CAMetalLayer(msg_send![class!(CAMetalLayer), new])
    }
    /// [Metal docs](https://developer.apple.com/documentation/quartzcore/cametallayer/1478163-device?language=objc).
    pub unsafe fn set_device(&self, device: MTLDevice) {
        let _: () = msg_send![self.0, setDevice:device.0];
    }
    /// [Metal docs](https://developer.apple.com/documentation/quartzcore/cametallayer/3175021-preferreddevice?language=objc).
    pub unsafe fn get_preferred_device(&self) -> MTLDevice {
        MTLDevice(msg_send![self.0, preferredDevice])
    }
    /// [Metal docs](https://developer.apple.com/documentation/quartzcore/cametallayer/1478155-pixelformat?language=objc).
    /// [MTLPixelFormat docs](https://developer.apple.com/documentation/metal/mtlpixelformat?language=objc).
    pub unsafe fn set_pixel_format(&self, format: u64) {
        let _: () = msg_send![self.0, setPixelFormat: format];
    }
    /// [Metal docs](https://developer.apple.com/documentation/quartzcore/cametallayer/1478168-framebufferonly?language=objc).
    pub unsafe fn set_framebuffer_only(&self, only: bool) {
        let _: () = msg_send![self.0, setFramebufferOnly: only];
    }
    /// [Metal docs](https://developer.apple.com/documentation/quartzcore/cametallayer/1478174-drawablesize?language=objc).
    pub unsafe fn set_drawable_size(&self, width: f64, height: f64) {
        let _: () = msg_send![self.0, setDrawableSize: (width, height)];
    }
    /// [Metal docs](https://developer.apple.com/documentation/quartzcore/cametallayer/1478157-presentswithtransaction?language=objc).
    pub unsafe fn set_presents_with_transaction(&self, set: bool) {
        let _: () = msg_send![self.0, setPresentsWithTransaction: set];
    }
    /// [Metal docs](https://developer.apple.com/documentation/quartzcore/cametallayer/2887087-displaysyncenabled?language=objc).
    pub unsafe fn set_display_sync_enabled(&self, enabled: bool) {
        let _: () = msg_send![self.0, setDisplaySyncEnabled: enabled];
    }
    /// [Metal docs](https://developer.apple.com/documentation/quartzcore/cametallayer/1478161-wantsextendeddynamicrangecontent?language=objc).
    pub unsafe fn set_wants_extended_dynamic_range_content(&self, set: bool) {
        let _: () = msg_send![self.0, setWantsExtendedDynamicRangeContent: set];
    }
    /// [Metal docs](https://developer.apple.com/documentation/quartzcore/cametallayer/1478172-nextdrawable?language=objc).
    pub unsafe fn next_drawable(&self) -> Option<CAMetalDrawable> {
        let k: *mut Object = msg_send![self.0, nextDrawable];
        match k.is_null() {
            true => None,
            false => Some(CAMetalDrawable(retain(k))),
        }
    }
    /// [Metal docs](https://developer.apple.com/documentation/quartzcore/cametallayer/2938720-maximumdrawablecount?language=objc).
    pub unsafe fn set_maximum_drawable_count(&self, count: u64) {
        let _: () = msg_send![self.0, setMaximumDrawableCount: count];
    }
    /// [Metal docs](https://developer.apple.com/documentation/quartzcore/cametallayer/2887086-allowsnextdrawabletimeout?language=objc).
    pub unsafe fn set_allows_next_drawable_timeout(&self, allows: bool) {
        let _: () = msg_send![self.0, setAllowsNextDrawableTimeout: allows];
    }
    /// [Metal docs](https://developer.apple.com/documentation/quartzcore/calayer/1410746-contentsscale?language=objc).
    pub unsafe fn set_contents_scale(&self, scale: f64) {
        let _: () = msg_send![self.0, setContentsScale: scale];
    }
}
