use objc::runtime::Object;

/// Releases the reference to the provided object.
pub unsafe fn release(obj: *mut Object) {
    let _: () = msg_send![obj, release];
}

/// Makes the reference to the provided object strong.
pub unsafe fn retain(obj: *mut Object) -> *mut Object {
    msg_send![obj, retain]
}
