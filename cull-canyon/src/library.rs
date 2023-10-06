use crate::{nsstring_to_string, string_to_nsstring};
use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use savannah::{handle, release, retain};
use std::ops::Range;
use std::os::raw::c_void;

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlfunctionconstantvalues?language=objc).
pub struct MTLFunctionConstantValues(pub *mut Object);
handle!(MTLFunctionConstantValues);

impl MTLFunctionConstantValues {
    pub unsafe fn new() -> MTLFunctionConstantValues {
        MTLFunctionConstantValues(msg_send![class!(MTLFunctionConstantValues), new])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlfunctionconstantvalues/1639531-setconstantvalue?language=objc).
    /// [MTLDataType docs](https://developer.apple.com/documentation/metal/mtldatatype?language=objc).
    pub unsafe fn set_constant_value_at_index(
        &self,
        value: *mut c_void,
        data_type: u64,
        index: u64,
    ) {
        msg_send![self.0, setConstantValue:value type:data_type atIndex:index]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlfunctionconstantvalues/1639530-setconstantvalue?language=objc).
    /// [MTLDataType docs](https://developer.apple.com/documentation/metal/mtldatatype?language=objc).
    pub unsafe fn set_constant_value_with_name(
        &self,
        value: *mut c_void,
        data_type: u64,
        name: &str,
    ) {
        let string = string_to_nsstring(name);
        msg_send![self.0, setConstantValue:value type:data_type withName:string]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlfunctionconstantvalues/1639527-setconstantvalues?language=objc).
    /// [MTLDataType docs](https://developer.apple.com/documentation/metal/mtldatatype?language=objc).
    pub unsafe fn set_constant_value_with_range(
        &self,
        value: *mut c_void,
        data_type: u64,
        range: Range<u64>,
    ) {
        msg_send![self.0, setConstantValues:value type:data_type withRange:(range.start, range.end - range.start)]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlfunctionconstantvalues/1639526-reset?language=objc).
    pub unsafe fn reset(&self) {
        msg_send![self.0, reset]
    }
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlcompileoptions?language=objc).
pub struct MTLCompileOptions(pub *mut Object);
handle!(MTLCompileOptions);

impl MTLCompileOptions {
    pub unsafe fn new() -> MTLCompileOptions {
        MTLCompileOptions(msg_send![class!(MTLCompileOptions), new])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcompileoptions/1515914-fastmathenabled?language=objc).
    pub unsafe fn is_fast_math_enabled(&self) -> bool {
        msg_send![self.0, fastMathEnabled]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcompileoptions/1515914-fastmathenabled?language=objc).
    pub unsafe fn set_fast_math_enabled(&self, enabled: bool) {
        let _: () = msg_send![self.0, setFastMathEnabled: enabled];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcompileoptions/1515494-languageversion?language=objc).
    /// [MTLLanguageVersion docs](https://developer.apple.com/documentation/metal/mtllanguageversion?language=objc).
    pub unsafe fn get_language_version(&self) -> u64 {
        msg_send![self.0, languageVersion]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcompileoptions/1515494-languageversion?language=objc).
    /// [MTLLanguageVersion docs](https://developer.apple.com/documentation/metal/mtllanguageversion?language=objc).
    pub unsafe fn set_language_version(&self, version: u64) {
        let _: () = msg_send![self.0, setLanguageVersion: version];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcompileoptions/1516172-preprocessormacros?language=objc).
    ///
    /// The returned value is guaranteed to be an NSDictionary.
    pub unsafe fn get_preprocessor_macros(&self) -> *mut Object {
        msg_send![self.0, preprocessorMacros]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcompileoptions/1516172-preprocessormacros?language=objc).
    ///
    /// The `macros` input value is a mutable raw pointer to an Object, NOT
    /// an NSDictionary; however, it is undefined behavior for that object to not
    /// be an NSDictionary.
    pub unsafe fn set_preprocessor_macros(&self, macros: *mut Object) {
        let _: () = msg_send![self.0, setPreprocessorMacros: macros];
    }
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlfunction?language=objc).
pub struct MTLFunction(pub *mut Object);
handle!(MTLFunction);

/// [Metal docs](https://developer.apple.com/documentation/metal/mtllibrary?language=objc).
pub struct MTLLibrary(pub *mut Object);
handle!(MTLLibrary);

impl MTLLibrary {
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtllibrary/1515651-functionnames?language=objc).
    pub unsafe fn get_function_names(&self) -> Vec<&str> {
        let names: *mut Object = msg_send![self.0, functionNames];
        let length: u64 = msg_send![names, count];
        (0..length)
            .map(|index| nsstring_to_string(msg_send![names, objectAtIndex: index]))
            .collect::<Vec<_>>()
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtllibrary/1515524-newfunctionwithname?language=objc).
    pub unsafe fn new_function_with_name(&self, name: &str) -> Result<MTLFunction, String> {
        let obj = string_to_nsstring(name);

        let f: *mut Object = msg_send![self.0, newFunctionWithName: obj];
        release(obj);

        if f.is_null() {
            Err(format!("This function {} does not exist.", name))
        } else {
            Ok(MTLFunction(f))
        }
    }

    /// [Metal docs](https://developer.apple.com/documentation/metal/mtllibrary/1640020-newfunctionwithname?language=objc).
    pub unsafe fn new_function_with_name_and_constants(
        &self,
        name: &str,
        values: MTLFunctionConstantValues,
    ) -> Result<MTLFunction, &str> {
        let mut err: *mut Object = std::ptr::null_mut();
        let obj = string_to_nsstring(name);
        let function: *mut Object =
            msg_send![self.0, newFunctionWithName:obj constantValues:values.0 error:&mut err];

        if !err.is_null() {
            Err(nsstring_to_string(msg_send![err, localizedDescription]))
        } else {
            Ok(MTLFunction(function))
        }
    }
}
