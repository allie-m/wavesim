use savannah::*;

#[macro_use]
extern crate objc;

use objc::runtime::Object;
use std::ptr::null_mut;
use std::os::raw::c_void;

struct Test(*mut Object);
handle![Test];

#[test]
fn null_handle() {
    Test(null_mut());

    println!("Have a handle.");
}

#[test]
fn os_version() {
    println!("You are running: {}", *CURRENT_OS_VERSION)
}

#[link(name = "Metal", kind = "framework")]
extern "C" {
    fn MTLCreateSystemDefaultDevice() -> *mut Object;
}

#[test]
fn call_metal_function(){
    unsafe {
        let ds = MTLCreateSystemDefaultDevice();
        let _handle = Test(ds);
        println!("So now I have a Metal device handle...");
    }
}

#[test]
fn new_string() {
    let hello_world = "Hey there world!";
    hello_world.len();
    let bytes = hello_world.as_ptr() as *const c_void;

    let class = class!(NSString);
    let nsstr = unsafe {
        let obj: Test = Test({
            let allocated: *mut Object = msg_send![class, alloc];
            msg_send![allocated, retain]
        });
        let obj: Test = Test(msg_send![
            obj.0,
            initWithBytes:bytes
            length:hello_world.len() as u64
            encoding:4 // this encoding represents UTF-8
        ]);

        obj
    };
    let bytes = unsafe {
        let bytes: *const std::os::raw::c_char = msg_send![nsstr.0, UTF8String];
        bytes as *const u8
    };
    let len: u64 = unsafe { msg_send![nsstr.0, length] };
    let message = unsafe {
        let bytes = std::slice::from_raw_parts(bytes, len as usize);
        std::str::from_utf8(bytes).unwrap()
    };

    println!("{}", message);
}
