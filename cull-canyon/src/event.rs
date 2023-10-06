use objc::{msg_send, runtime::Object, sel, sel_impl};
use savannah::{handle, release, retain};

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlevent?language=objc).
pub struct MTLEvent(pub *mut Object);
handle!(MTLEvent);

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlsharedevent?language=objc).
pub struct MTLSharedEvent(pub *mut Object);
handle!(MTLSharedEvent);

impl MTLSharedEvent {
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlsharedevent/2966575-signaledvalue?language=objc).
    pub unsafe fn get_signaled_value(&self) -> u64 {
        msg_send![self.0, signaledValue]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlsharedevent/2966575-signaledvalue?language=objc).
    pub unsafe fn set_signaled_value(&self, value: u64) {
        let _: () = msg_send![self.0, setSignaledValue: value];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlsharedevent/2981025-newsharedeventhandle?language=objc).
    pub unsafe fn new_shared_event_handle(&self) -> MTLSharedEventHandle {
        MTLSharedEventHandle(msg_send![self.0, newSharedEventHandle])
    }
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlsharedeventhandle?language=objc).
pub struct MTLSharedEventHandle(pub *mut Object);
handle!(MTLSharedEventHandle);

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlfence?language=objc).
pub struct MTLFence(pub *mut Object);
handle!(MTLFence);
