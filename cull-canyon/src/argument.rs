use crate::{MTLArgumentEncoder, MTLIndirectCommandBuffer};
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use savannah::{handle, release, retain};

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlargumentdescriptor?language=objc).
pub struct MTLArgumentDescriptor(pub *mut Object);
handle!(MTLArgumentDescriptor);

impl MTLArgumentDescriptor {
    pub unsafe fn new() -> MTLArgumentDescriptor {
        MTLArgumentDescriptor(retain(msg_send![
            class!(MTLArgumentDescriptor),
            argumentDescriptor
        ]))
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/2915733-datatype?language=objc).
    /// [MTLDataType docs](https://developer.apple.com/documentation/metal/mtldatatype?language=objc).
    pub unsafe fn set_data_type(&self, data_type: u64) {
        let _: () = msg_send![self.0, setDataType: data_type];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/2915732-index?language=objc).
    pub unsafe fn set_index(&self, index: u64) {
        let _: () = msg_send![self.0, setIndex: index];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/2915735-access?language=objc).
    /// [MTLArgumentAccess docs](https://developer.apple.com/documentation/metal/mtlargumentaccess?language=objc).
    pub unsafe fn set_access(&self, access: u64) {
        let _: () = msg_send![self.0, setAccess: access];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/2915734-arraylength?language=objc).
    pub unsafe fn set_array_length(&self, length: u64) {
        let _: () = msg_send![self.0, setArrayLength: length];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/2915739-constantblockalignment?language=objc).
    pub unsafe fn set_constant_block_alignment(&self, alignment: u64) {
        let _: () = msg_send![self.0, setConstantBlockAlignment: alignment];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/2915741-texturetype?language=objc).
    /// [MTLTextureType docs](https://developer.apple.com/documentation/metal/mtltexturetype?language=objc).
    pub unsafe fn set_texture_type(&self, texture_type: u64) {
        let _: () = msg_send![self.0, setTextureType: texture_type];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlargumentencoder/2967410-setindirectcommandbuffer?language=objc).
    pub unsafe fn set_indirect_command_buffer(
        &self,
        command_buffer: MTLIndirectCommandBuffer,
        index: u64,
    ) {
        let _: () = msg_send![self.0, setIndirectCommandBuffer:command_buffer.0 atIndex:index];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlargumentencoder/2915783-newargumentencoderforbufferatind?language=objc).
    pub unsafe fn new_argument_encoder_for_buffer_at_index(
        &self,
        index: u64,
    ) -> MTLArgumentEncoder {
        MTLArgumentEncoder(msg_send![self.0, newArgumentEncoderForBufferAtIndex: index])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlargumentencoder/2915775-alignment?language=objc).
    pub unsafe fn get_alignment(&self) -> u64 {
        msg_send![self.0, alignment]
    }
}
