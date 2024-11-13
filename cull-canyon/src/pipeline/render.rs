use crate::MTLFunction;
use crate::__just_clone;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use savannah::{handle, release, retain};

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptor?language=objc).
pub struct MTLVertexAttributeDescriptor(pub *mut Object);
handle!(MTLVertexAttributeDescriptor);

impl MTLVertexAttributeDescriptor {
    pub unsafe fn new() -> MTLVertexAttributeDescriptor {
        MTLVertexAttributeDescriptor(retain(msg_send![class!(MTLVertexAttributeDescriptor), new]))
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptor/1516081-format?language=objc).
    /// [MTLVertexFormat docs](https://developer.apple.com/documentation/metal/mtlvertexformat?language=objc).
    pub unsafe fn set_format(&self, format: u64) {
        let _: () = msg_send![self.0, setFormat: format];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptor/1515785-offset?language=objc).
    pub unsafe fn set_offset(&self, offset: u64) {
        let _: () = msg_send![self.0, setOffset: offset];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptor/1515502-bufferindex?language=objc).
    pub unsafe fn set_buffer_index(&self, index: u64) {
        let _: () = msg_send![self.0, setBufferIndex: index];
    }
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptorarray?language=objc).
pub struct MTLVertexAttributeDescriptorArray(pub *mut Object);
// handle!(MTLVertexAttributeDescriptorArray);

impl MTLVertexAttributeDescriptorArray {
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptorarray/1516138-objectatindexedsubscript?language=objc).
    pub unsafe fn object_at_indexed_subscript(&self, index: u64) -> MTLVertexAttributeDescriptor {
        MTLVertexAttributeDescriptor(retain(msg_send![self.0, objectAtIndexedSubscript: index]))
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptorarray/1550757-setobject?language=objc).
    pub unsafe fn set_object_at_indexed_subscript(
        &self,
        attribute_descriptor: MTLVertexAttributeDescriptor,
        index: u64,
    ) {
        let _: () = msg_send![self.0, setObject:attribute_descriptor.0 atIndexedSubscript:index];
    }
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptor?language=objc).
pub struct MTLVertexBufferLayoutDescriptor(pub *mut Object);
handle!(MTLVertexBufferLayoutDescriptor);

impl MTLVertexBufferLayoutDescriptor {
    pub unsafe fn new() -> MTLVertexBufferLayoutDescriptor {
        MTLVertexBufferLayoutDescriptor(retain(msg_send![
            class!(MTLVertexBufferLayoutDescriptor),
            new
        ]))
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptor/1515341-stepfunction?language=objc).
    /// [MTLStepFunction docs](https://developer.apple.com/documentation/metal/mtlstepfunction?language=objc).
    pub unsafe fn set_step_function(&self, step_function: u64) {
        let _: () = msg_send![self.0, setStepFunction: step_function];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptor/1516148-steprate?language=objc).
    pub unsafe fn set_step_rate(&self, rate: u64) {
        let _: () = msg_send![self.0, setStepRate: rate];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptor/1515441-stride?language=objc).
    pub unsafe fn set_stride(&self, stride: u64) {
        let _: () = msg_send![self.0, setStride: stride];
    }
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlvertexdescriptor/1515480-layouts?language=objc).
pub struct MTLVertexBufferLayoutDescriptorArray(pub *mut Object);
// handle!(MTLVertexBufferLayoutDescriptorArray);

impl MTLVertexBufferLayoutDescriptorArray {
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptorarray/1516230-objectatindexedsubscript?language=objc).
    pub unsafe fn object_at_indexed_subscript(
        &self,
        index: u64,
    ) -> MTLVertexBufferLayoutDescriptor {
        MTLVertexBufferLayoutDescriptor(retain(msg_send![self.0, objectAtIndexedSubscript: index]))
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptorarray/1550758-setobject?language=objc).
    pub unsafe fn set_object_at_indexed_subscript(
        &self,
        buffer_layout_descriptor: MTLVertexBufferLayoutDescriptor,
        index: u64,
    ) {
        let _: () =
            msg_send![self.0, setObject:buffer_layout_descriptor.0 atIndexedSubscript:index];
    }
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlvertexdescriptor?language=objc).
pub struct MTLVertexDescriptor(pub *mut Object);
handle!(MTLVertexDescriptor);

impl MTLVertexDescriptor {
    pub unsafe fn new() -> MTLVertexDescriptor {
        MTLVertexDescriptor(retain(msg_send![
            class!(MTLVertexDescriptor),
            vertexDescriptor
        ]))
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlvertexdescriptor/1515921-attributes?language=objc).
    pub unsafe fn get_attributes(&self) -> MTLVertexAttributeDescriptorArray {
        MTLVertexAttributeDescriptorArray(msg_send![self.0, attributes])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlvertexdescriptor/1515480-layouts?language=objc).
    pub unsafe fn get_layouts(&self) -> MTLVertexBufferLayoutDescriptorArray {
        MTLVertexBufferLayoutDescriptorArray(msg_send![self.0, layouts])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlvertexdescriptor/1515988-reset?language=objc).
    pub unsafe fn reset(&self) {
        let _: () = msg_send![self.0, reset];
    }
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptor?language=objc).
pub struct MTLPipelineBufferDescriptor(pub *mut Object);
handle!(MTLPipelineBufferDescriptor);

impl MTLPipelineBufferDescriptor {
    pub unsafe fn new() -> MTLPipelineBufferDescriptor {
        MTLPipelineBufferDescriptor(msg_send![class!(MTLPipelineBufferDescriptor), new])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptor/2879274-mutability?language=objc).
    /// [MTLMutability docs](https://developer.apple.com/documentation/metal/mtlmutability?language=objc).
    pub unsafe fn get_mutability(&self) -> u64 {
        msg_send![self.0, mutability]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptor/2879274-mutability?language=objc).
    /// [MTLMutability docs](https://developer.apple.com/documentation/metal/mtlmutability?language=objc).
    pub unsafe fn set_mutability(&self, mutability: u64) {
        let _: () = msg_send![self.0, setMutability: mutability];
    }
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptorarray?language=objc).
pub struct MTLPipelineBufferDescriptorArray(pub *mut Object);
__just_clone!(MTLPipelineBufferDescriptorArray);

impl MTLPipelineBufferDescriptorArray {
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptorarray/2879272-objectatindexedsubscript?language=objc).
    pub unsafe fn object_at_indexed_subscript(&self, index: u64) -> MTLPipelineBufferDescriptor {
        MTLPipelineBufferDescriptor(retain(msg_send![self.0, objectAtIndexedSubscript: index]))
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptorarray/2879310-setobject?language=objc).
    pub unsafe fn set_object_at_indexed_subscript(
        &self,
        descriptor: MTLPipelineBufferDescriptor,
        index: u64,
    ) {
        let _: () = msg_send![self.0, setObject:descriptor.0 atIndexedSubscript:index];
    }
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor?language=objc).
pub struct MTLRenderPipelineColorAttachmentDescriptor(pub *mut Object);
handle!(MTLRenderPipelineColorAttachmentDescriptor);

impl MTLRenderPipelineColorAttachmentDescriptor {
    pub unsafe fn new() -> MTLRenderPipelineColorAttachmentDescriptor {
        MTLRenderPipelineColorAttachmentDescriptor(msg_send![
            class!(MTLRenderPipelineColorAttachmentDescriptor),
            new
        ])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/1514651-pixelformat?language=objc).
    /// [MTLPixelFormat docs](https://developer.apple.com/documentation/metal/mtlpixelformat?language=objc).
    pub unsafe fn set_pixel_format(&self, pixel_format: u64) {
        let _: () = msg_send![self.0, setPixelFormat: pixel_format];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/1514619-writemask?language=objc).
    /// [MTLColorWriteMask docs](https://developer.apple.com/documentation/metal/mtlcolorwritemask?language=objc).
    pub unsafe fn set_write_mask(&self, mask: u64) {
        let _: () = msg_send![self.0, setWriteMask: mask];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/1514642-blendingenabled?language=objc).
    pub unsafe fn set_blending_enabled(&self, blending: bool) {
        let _: () = msg_send![self.0, setBlendingEnabled: blending];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/1514666-alphablendoperation?language=objc).
    /// [MTLBlendOperation docs](https://developer.apple.com/documentation/metal/mtlblendoperation?language=objc).
    pub unsafe fn set_alpha_blend_operation(&self, operation: u64) {
        let _: () = msg_send![self.0, setAlphaBlendOperation: operation];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/1514659-rgbblendoperation?language=objc).
    /// [MTLBlendOperation docs](https://developer.apple.com/documentation/metal/mtlblendoperation?language=objc).
    pub unsafe fn set_rgb_blend_operation(&self, operation: u64) {
        let _: () = msg_send![self.0, setRgbBlendOperation: operation];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/1514657-destinationalphablendfactor?language=objc).
    /// [MTLBlendFactor docs](https://developer.apple.com/documentation/metal/mtlblendfactor?language=objc).
    pub unsafe fn set_destination_alpha_blend_factor(&self, factor: u64) {
        let _: () = msg_send![self.0, setDestinationAlphaBlendFactor: factor];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/1514626-destinationrgbblendfactor?language=objc).
    /// [MTLBlendFactor docs](https://developer.apple.com/documentation/metal/mtlblendfactor?language=objc).
    pub unsafe fn set_destination_rgb_blend_factor(&self, factor: u64) {
        let _: () = msg_send![self.0, setDestinationRGBBlendFactor: factor];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/1514615-sourcergbblendfactor?language=objc).
    /// [MTLBlendFactor docs](https://developer.apple.com/documentation/metal/mtlblendfactor?language=objc).
    pub unsafe fn set_source_alpha_blend_factor(&self, factor: u64) {
        let _: () = msg_send![self.0, setSourceAlphaBlendFactor: factor];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/1514615-sourcergbblendfactor?language=objc).
    /// [MTLBlendFactor docs](https://developer.apple.com/documentation/metal/mtlblendfactor?language=objc).
    pub unsafe fn set_source_rgb_blend_factor(&self, factor: u64) {
        let _: () = msg_send![self.0, setSourceRGBBlendFactor: factor];
    }
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptorarray?language=objc).
pub struct MTLRenderPipelineColorAttachmentDescriptorArray(pub *mut Object);
__just_clone!(MTLRenderPipelineColorAttachmentDescriptorArray);

impl MTLRenderPipelineColorAttachmentDescriptorArray {
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptorarray/1514673-objectatindexedsubscript?language=objc).
    pub unsafe fn object_at_indexed_subscript(
        &self,
        index: u64,
    ) -> MTLRenderPipelineColorAttachmentDescriptor {
        MTLRenderPipelineColorAttachmentDescriptor(retain(msg_send![
            self.0,
            objectAtIndexedSubscript: index
        ]))
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptorarray/1514675-setobject?language=objc).
    pub unsafe fn set_object_at_indexed_subscript(
        &self,
        descriptor: MTLRenderPipelineColorAttachmentDescriptor,
        index: u64,
    ) {
        let _: () = msg_send![self.0, setObject:descriptor.0 atIndexedSubscript:index];
    }
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor?language=objc).
pub struct MTLRenderPipelineDescriptor(pub *mut Object);
handle!(MTLRenderPipelineDescriptor);

impl MTLRenderPipelineDescriptor {
    pub unsafe fn new() -> MTLRenderPipelineDescriptor {
        MTLRenderPipelineDescriptor(msg_send![class!(MTLRenderPipelineDescriptor), new])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1514679-vertexfunction?language=objc).
    pub unsafe fn get_vertex_function(&self) -> MTLFunction {
        MTLFunction(msg_send![self.0, vertexFunction])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1514679-vertexfunction?language=objc).
    pub unsafe fn set_vertex_function(&self, function: MTLFunction) {
        let _: () = msg_send![self.0, setVertexFunction:function.0];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1514600-fragmentfunction?language=objc).
    pub unsafe fn get_fragment_function(&self) -> MTLFunction {
        MTLFunction(msg_send![self.0, fragmentFunction])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1514600-fragmentfunction?language=objc).
    pub unsafe fn set_fragment_function(&self, function: MTLFunction) {
        let _: () = msg_send![self.0, setFragmentFunction:function.0];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1514681-vertexdescriptor?language=objc).
    pub unsafe fn set_vertex_descriptor(&self, descriptor: MTLVertexDescriptor) {
        let _: () = msg_send![self.0, setVertexDescriptor:descriptor.0];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/2879280-vertexbuffers?language=objc).
    pub unsafe fn get_vertex_buffers(&self) -> MTLPipelineBufferDescriptorArray {
        MTLPipelineBufferDescriptorArray(msg_send![self.0, vertexBuffers])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/2879275-fragmentbuffers?language=objc).
    pub unsafe fn get_fragment_buffers(&self) -> MTLPipelineBufferDescriptorArray {
        MTLPipelineBufferDescriptorArray(msg_send![self.0, fragmentBuffers])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1514712-colorattachments?language=objc).
    pub unsafe fn get_color_attachments(&self) -> MTLRenderPipelineColorAttachmentDescriptorArray {
        MTLRenderPipelineColorAttachmentDescriptorArray(msg_send![self.0, colorAttachments])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1514608-depthattachmentpixelformat?language=objc).
    /// [MTLPixelFormat docs](https://developer.apple.com/documentation/metal/mtlpixelformat?language=objc).
    pub unsafe fn set_depth_attachment_pixel_format(&self, format: u64) {
        let _: () = msg_send![self.0, setDepthAttachmentPixelFormat: format];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1514650-stencilattachmentpixelformat?language=objc).
    /// [MTLPixelFormat docs](https://developer.apple.com/documentation/metal/mtlpixelformat?language=objc).
    pub unsafe fn set_stencil_attachment_pixel_format(&self, format: u64) {
        let _: () = msg_send![self.0, setStencilAttachmentPixelFormat: format];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1514699-samplecount?language=objc).
    pub unsafe fn set_sample_count(&self, count: u64) {
        let _: () = msg_send![self.0, setSampleCount: count];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1514624-alphatocoverageenabled?language=objc).
    pub unsafe fn set_alpha_to_coverage_enabled(&self, enabled: bool) {
        let _: () = msg_send![self.0, setAlphaToCoverageEnabled: enabled];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1514697-alphatooneenabled?language=objc).
    pub unsafe fn set_alpha_to_one_enabled(&self, enabled: bool) {
        let _: () = msg_send![self.0, setAlphaToOneEnabled: enabled];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1514708-rasterizationenabled?language=objc).
    pub unsafe fn set_rasterization_enabled(&self, enabled: bool) {
        let _: () = msg_send![self.0, setRasterizationEnabled: enabled];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1514684-inputprimitivetopology?language=objc).
    /// [MTLPrimitiveTopologyClass docs](https://developer.apple.com/documentation/metal/mtlprimitivetopologyclass?language=objc).
    pub unsafe fn set_input_primitive_topology(&self, topology: u64) {
        let _: () = msg_send![self.0, setInputPrimitiveTopology: topology];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/2890271-rastersamplecount?language=objc).
    pub unsafe fn set_raster_sample_count(&self, count: u64) {
        let _: () = msg_send![self.0, setRasterSampleCount: count];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1640060-maxtessellationfactor?language=objc).
    pub unsafe fn set_max_tessellation_factor(&self, factor: u64) {
        let _: () = msg_send![self.0, setMaxTessellationFactor: factor];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1640045-tessellationfactorscaleenabled?language=objc).
    pub unsafe fn set_tessellation_factor_scale_enabled(&self, enabled: bool) {
        let _: () = msg_send![self.0, setTessellationFactorScaleEnabled: enabled];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1639951-tessellationfactorformat?language=objc).
    /// [MTLTessellationFactorFormat docs](https://developer.apple.com/documentation/metal/mtltessellationfactorformat?language=objc).
    pub unsafe fn set_tessellation_factor_format(&self, format: u64) {
        let _: () = msg_send![self.0, setTessellationFactorFormat: format];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1640059-tessellationcontrolpointindextyp?language=objc).
    /// [MTLTessellationControlPointIndexType docs](https://developer.apple.com/documentation/metal/mtltessellationcontrolpointindextype?language=objc).
    pub unsafe fn set_tessellation_control_point_index_type(&self, index_type: u64) {
        let _: () = msg_send![self.0, setTessellationControlPointIndexType: index_type];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1640062-tessellationfactorstepfunction?language=objc).
    /// [MTLTessellationFactorStepFunction docs](https://developer.apple.com/documentation/metal/mtltessellationfactorstepfunction?language=objc).
    pub unsafe fn set_tessellation_factor_step_function(&self, step_function: u64) {
        let _: () = msg_send![self.0, setTessellationFactorStepFunction: step_function];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1639911-tessellationoutputwindingorder?language=objc).
    /// [MTLWinding docs](https://developer.apple.com/documentation/metal/mtlwinding?language=objc).
    pub unsafe fn set_tessellation_output_winding_order(&self, winding: u64) {
        let _: () = msg_send![self.0, setTessellationOutputWindingOrder: winding];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1639979-tessellationpartitionmode?language=objc).
    /// [MTLTessellationPartitionMode docs](https://developer.apple.com/documentation/metal/mtltessellationpartitionmode?language=objc).
    pub unsafe fn set_tessellation_partition_mode(&self, mode: u64) {
        let _: () = msg_send![self.0, setTessellationPartitionMode: mode];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/2966638-supportindirectcommandbuffers?language=objc).
    pub unsafe fn set_support_indirect_command_buffers(&self, support: bool) {
        let _: () = msg_send![self.0, setSupportIndirectCommandBuffers: support];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/3088854-maxvertexamplificationcount?language=objc).
    pub unsafe fn set_max_vertex_amplification_count(&self, count: u64) {
        let _: () = msg_send![self.0, setMaxVertexAmplificationCount: count];
    }
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate?language=objc).
pub struct MTLRenderPipelineState(pub *mut Object);
handle!(MTLRenderPipelineState);

impl MTLRenderPipelineState {
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/2866352-maxtotalthreadsperthreadgroup?language=objc).
    pub unsafe fn max_total_threads_per_threadgroup(&self) -> u64 {
        msg_send![self.0, maxTotalThreadsPerThreadgroup]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/2866353-threadgroupsizematchestilesize?language=objc).
    pub unsafe fn threadgroup_size_matches_tile_size(&self) -> bool {
        msg_send![self.0, threadgroupSizeMatchesTileSize]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/2928215-imageblocksamplelength?language=objc).
    pub unsafe fn imageblock_sample_length(&self) -> u64 {
        msg_send![self.0, imageblockSampleLength]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/2928216-imageblockmemorylengthfordimensi?language=objc).
    pub unsafe fn imageblock_memory_length_for_dimensions(&self, dimensions: (u64, u64)) -> u64 {
        msg_send![self.0, imageblockMemoryLengthForDimensions: dimensions]
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/2966639-supportindirectcommandbuffers?language=objc).
    pub unsafe fn support_indirect_command_buffers(&self) -> bool {
        msg_send![self.0, supportIndirectCommandBuffers]
    }
}
