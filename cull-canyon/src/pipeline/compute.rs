use crate::{MTLFunction, MTLPipelineBufferDescriptorArray, __just_clone};
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use savannah::{handle, release, retain};

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor?language=objc).
pub struct MTLComputePipelineDescriptor(pub *mut Object);
handle!(MTLComputePipelineDescriptor);

impl MTLComputePipelineDescriptor {
    pub unsafe fn new() -> MTLComputePipelineDescriptor {
        MTLComputePipelineDescriptor(msg_send![class!(MTLComputePipelineDescriptor), new])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/1414917-computefunction?language=objc).
    pub unsafe fn get_compute_function(&self) -> MTLFunction {
        msg_send![self.0, computeFunction]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/1414917-computefunction?language=objc).
    pub unsafe fn set_compute_function(&self, function: MTLFunction) {
        let _: () = msg_send![self.0, setComputeFunction:function.0];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/1414915-threadgroupsizeismultipleofthrea?language=objc).
    pub unsafe fn is_threadgroup_size_is_multiple_of_thread_execution_width(&self) -> u64 {
        msg_send![self.0, threadGroupSizeIsMultipleOfThreadExecutionWidth]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/1414915-threadgroupsizeismultipleofthrea?language=objc).
    pub unsafe fn set_threadgroup_size_is_multiple_of_thread_execution_width(&self, width: u64) {
        let _: () = msg_send![
            self.0,
            setThreadGroupSizeIsMultipleOfThreadExecutionWidth: width
        ];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/2966560-maxtotalthreadsperthreadgroup?language=objc).
    pub unsafe fn get_max_total_threads_per_threadgroup(&self) -> u64 {
        msg_send![self.0, maxTotalThreadsPerThreadgroup]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/2966560-maxtotalthreadsperthreadgroup?language=objc).
    pub unsafe fn set_max_total_threads_per_threadgroup(&self, max: u64) {
        let _: () = msg_send![self.0, setMaxTotalThreadsPerThreadgroup: max];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/2092373-stageinputdescriptor?language=objc).
    pub unsafe fn get_stage_input_descriptor(&self) -> MTLStageInputOutputDescriptor {
        MTLStageInputOutputDescriptor(msg_send![self.0, stageInputDescriptor])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/2092373-stageinputdescriptor?language=objc).
    pub unsafe fn set_stage_input_descriptor(&self, descriptor: MTLStageInputOutputDescriptor) {
        let _: () = msg_send![self.0, setStageInputDescriptor:descriptor.0];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/2879269-buffers?language=objc).
    pub unsafe fn get_buffers(&self) -> MTLPipelineBufferDescriptorArray {
        MTLPipelineBufferDescriptorArray(msg_send![self.0, buffers])
    }
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlattributedescriptor?language=objc).
pub struct MTLAttributeDescriptor(pub *mut Object);
handle!(MTLAttributeDescriptor);

impl MTLAttributeDescriptor {
    pub unsafe fn new() -> MTLAttributeDescriptor {
        MTLAttributeDescriptor(msg_send![class!(MTLAttributeDescriptor), new])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlattributedescriptor/2097218-bufferindex?language=objc).
    pub unsafe fn set_buffer_index(&self, index: u64) {
        let _: () = msg_send![self.0, setBufferIndex: index];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlattributedescriptor/2097220-offset?language=objc).
    pub unsafe fn set_offset(&self, offset: u64) {
        let _: () = msg_send![self.0, setOffset: offset];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlattributedescriptor/2097194-format?language=objc).
    /// [MTLAttributeFormat docs](https://developer.apple.com/documentation/metal/mtlattributeformat?language=objc).
    pub unsafe fn set_format(&self, format: u64) {
        let _: () = msg_send![self.0, setFormat: format];
    }
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlstageinputoutputdescriptor?language=objc).
pub struct MTLAttributeDescriptorArray(pub *mut Object);
__just_clone!(MTLAttributeDescriptorArray);

impl MTLAttributeDescriptorArray {
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlattributedescriptorarray/2097215-objectatindexedsubscript?language=objc).
    pub unsafe fn object_at_indexed_subscript(&self, index: u64) -> MTLAttributeDescriptor {
        MTLAttributeDescriptor(retain(msg_send![self.0, objectAtIndexedSubscript: index]))
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlattributedescriptorarray/2097293-setobject?language=objc).
    pub unsafe fn set_object_at_indexed_subscript(
        &self,
        attribute: MTLAttributeDescriptor,
        index: u64,
    ) {
        let _: () = msg_send![self.0, setObject:attribute.0 atIndexedSubscript:index];
    }
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptor?language=objc).
pub struct MTLBufferLayoutDescriptor(pub *mut Object);
handle!(MTLBufferLayoutDescriptor);

impl MTLBufferLayoutDescriptor {
    pub unsafe fn new() -> MTLBufferLayoutDescriptor {
        MTLBufferLayoutDescriptor(msg_send![class!(MTLBufferLayoutDescriptor), new])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptor/2097190-stride?language=objc).
    pub unsafe fn set_stride(&self, stride: u64) {
        let _: () = msg_send![self.0, setStride: stride];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptor/2097182-stepfunction?language=objc).
    /// [MTLStepFunction docs](https://developer.apple.com/documentation/metal/mtlstepfunction?language=objc).
    pub unsafe fn set_step_function(&self, step_function: u64) {
        let _: () = msg_send![self.0, setStepFunction: step_function];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptor/2097164-steprate?language=objc).
    pub unsafe fn set_step_rate(&self, rate: u64) {
        let _: () = msg_send![self.0, setStepRate: rate];
    }
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptorarray?language=objc).
pub struct MTLBufferLayoutDescriptorArray(pub *mut Object);
__just_clone!(MTLBufferLayoutDescriptorArray);

impl MTLBufferLayoutDescriptorArray {
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptorarray/2097228-objectatindexedsubscript?language=objc).
    pub unsafe fn object_at_indexed_subscript(&self, index: u64) -> MTLBufferLayoutDescriptor {
        MTLBufferLayoutDescriptor(retain(msg_send![self.0, objectAtIndexedSubscript: index]))
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptorarray/2097295-setobject?language=objc).
    pub unsafe fn set_object_at_indexed_subscript(
        &self,
        attribute: MTLBufferLayoutDescriptor,
        index: u64,
    ) {
        let _: () = msg_send![self.0, setObject:attribute.0 atIndexedSubscript:index];
    }
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlstageinputoutputdescriptor?language=objc).
pub struct MTLStageInputOutputDescriptor(pub *mut Object);
handle!(MTLStageInputOutputDescriptor);

impl MTLStageInputOutputDescriptor {
    pub unsafe fn new() -> MTLStageInputOutputDescriptor {
        MTLStageInputOutputDescriptor(retain(msg_send![
            class!(MTLStageInputOutputDescriptor),
            stageInputOutputDescriptor
        ]))
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlstageinputoutputdescriptor/2097206-attributes?language=objc).
    pub unsafe fn get_attributes(&self) -> MTLAttributeDescriptorArray {
        MTLAttributeDescriptorArray(msg_send![self.0, attributes])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlstageinputoutputdescriptor/2097202-layouts?language=objc).
    pub unsafe fn get_layouts(&self) -> MTLBufferLayoutDescriptorArray {
        MTLBufferLayoutDescriptorArray(msg_send![self.0, layouts])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlstageinputoutputdescriptor/2097237-indexbufferindex?language=objc).
    pub unsafe fn set_index_buffer_index(&self, index: u64) {
        let _: () = msg_send![self.0, setIndexBufferIndex: index];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlstageinputoutputdescriptor/2097184-indextype?language=objc).
    /// [MTLIndexType docs](https://developer.apple.com/documentation/metal/mtlindextype?language=objc).
    pub unsafe fn set_index_type(&self, index_type: u64) {
        let _: () = msg_send![self.0, setIndexType: index_type];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlstageinputoutputdescriptor/2097185-reset?language=objc).
    pub unsafe fn reset(&self) {
        let _: () = msg_send![self.0, reset];
    }
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate?language=objc).
pub struct MTLComputePipelineState(pub *mut Object);
handle!(MTLComputePipelineState);

impl MTLComputePipelineState {
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/1414927-maxtotalthreadsperthreadgroup?language=objc).
    pub unsafe fn max_total_threads_per_threadgroup(&self) -> u64 {
        msg_send![self.0, maxTotalThreadsPerThreadgroup]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/1414911-threadexecutionwidth?language=objc).
    pub unsafe fn thread_execute_width(&self) -> u64 {
        msg_send![self.0, threadExecuteWidth]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/2877435-staticthreadgroupmemorylength?language=objc).
    pub unsafe fn static_threadgroup_memory_length(&self) -> u64 {
        msg_send![self.0, staticThreadgroupMemoryLength]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/2928195-imageblockmemorylengthfordimensi?language=objc).
    pub unsafe fn imageblock_memory_length_for_dimensions(&self) -> (u64, u64) {
        msg_send![self.0, imageblockMemoryLengthForDimensions]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/2966562-supportindirectcommandbuffers?language=objc).
    pub unsafe fn support_indirect_command_buffers(&self) -> bool {
        msg_send![self.0, supportIndirectCommandBuffers]
    }
}
