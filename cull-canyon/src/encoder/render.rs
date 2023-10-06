use crate::{MTLBuffer, MTLDepthStencilState, MTLResource, MTLSamplerState, __just_clone};
use crate::{MTLCommandEncoder, MTLRenderPipelineState, MTLTexture};
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use savannah::{handle, release, retain};
use std::os::raw::c_void;

pub trait MTLRenderPassAttachmentDescriptor {
    #[doc(hidden)]
    fn get_obj(&self) -> *mut Object;
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/1437958-texture?language=objc).
    unsafe fn set_texture(&self, texture: MTLTexture) {
        let _: () = msg_send![self.get_obj(), setTexture:texture.0];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/1437984-level?language=objc).
    unsafe fn set_level(&self, level: u64) {
        let _: () = msg_send![self.get_obj(), setLevel: level];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/1437914-slice?language=objc).
    unsafe fn set_slice(&self, slice: u64) {
        let _: () = msg_send![self.get_obj(), setSlice: slice];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/1437952-depthplane?language=objc).
    unsafe fn set_depth_plane(&self, plane: u64) {
        let _: () = msg_send![self.get_obj(), setDepthPlane: plane];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/1437905-loadaction?language=objc).
    /// [MTLLoadAction docs](https://developer.apple.com/documentation/metal/mtlloadaction?language=objc).
    unsafe fn set_load_action(&self, action: u64) {
        let _: () = msg_send![self.get_obj(), setLoadAction: action];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/1437956-storeaction?language=objc).
    /// [MTLStoreAction docs](https://developer.apple.com/documentation/metal/mtlstoreaction?language=objc).
    unsafe fn set_store_action(&self, action: u64) {
        let _: () = msg_send![self.get_obj(), setStoreAction: action];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/2919782-storeactionoptions?language=objc).
    /// [MTLStoreActionOptions docs](https://developer.apple.com/documentation/metal/mtlstoreactionoptions?language=objc).
    unsafe fn set_store_action_options(&self, options: u64) {
        let _: () = msg_send![self.get_obj(), setStoreActionOptions: options];
    }
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpasscolorattachmentdescriptor?language=objc).
pub struct MTLRenderPassColorAttachmentDescriptor(pub *mut Object);
handle!(MTLRenderPassColorAttachmentDescriptor);

impl MTLRenderPassAttachmentDescriptor for MTLRenderPassColorAttachmentDescriptor {
    fn get_obj(&self) -> *mut Object {
        self.0
    }
}

impl MTLRenderPassAttachmentDescriptor for MTLRenderPassDepthAttachmentDescriptor {
    fn get_obj(&self) -> *mut Object {
        self.0
    }
}

impl MTLRenderPassColorAttachmentDescriptor {
    pub unsafe fn new() -> MTLRenderPassColorAttachmentDescriptor {
        MTLRenderPassColorAttachmentDescriptor(msg_send![
            class!(MTLRenderPassColorAttachmentDescriptor),
            new
        ])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpasscolorattachmentdescriptor/1437924-clearcolor?language=objc).
    pub unsafe fn set_clear_color(&self, r: f64, g: f64, b: f64, a: f64) {
        let _: () = msg_send![self.0, setClearColor: (r, g, b, a)];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpasscolorattachmentdescriptor/1437924-clearcolor?language=objc).
    pub unsafe fn get_clear_color(&self) -> (f64, f64, f64, f64) {
        msg_send![self.0, clearColor]
    }
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpasscolorattachmentdescriptorarray?language=objc).
pub struct MTLRenderPassColorAttachmentDescriptorArray(pub *mut Object);
__just_clone!(MTLRenderPassColorAttachmentDescriptorArray);

impl MTLRenderPassColorAttachmentDescriptorArray {
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpasscolorattachmentdescriptorarray/1437977-objectatindexedsubscript?language=objc).
    pub unsafe fn object_at_indexed_subscript(
        &self,
        index: u64,
    ) -> MTLRenderPassColorAttachmentDescriptor {
        MTLRenderPassColorAttachmentDescriptor(retain(msg_send![
            self.0,
            objectAtIndexedSubscript: index
        ]))
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpasscolorattachmentdescriptorarray/1437982-setobject?language=objc).
    pub unsafe fn set_object_at_indexed_subscript(
        &self,
        index: u64,
        descriptor: MTLRenderPassColorAttachmentDescriptor,
    ) {
        let _: () = msg_send![self.0, setObject:descriptor.0 atIndexedSubscript:index];
    }
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpassdepthattachmentdescriptor?language=objc).
pub struct MTLRenderPassDepthAttachmentDescriptor(pub *mut Object);
__just_clone!(MTLRenderPassDepthAttachmentDescriptor);

impl MTLRenderPassDepthAttachmentDescriptor {
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpassdepthattachmentdescriptor/1437933-cleardepth?language=objc).
    pub unsafe fn set_clear_depth(&self, depth: f64) {
        let _: () = msg_send![self.0, setClearDepth: depth];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpassdepthattachmentdescriptor/1437933-cleardepth?language=objc).
    pub unsafe fn get_clear_depth(&self) -> f64 {
        msg_send![self.0, clearDepth]
    }
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor?language=objc).
pub struct MTLRenderPassDescriptor(pub *mut Object);
handle!(MTLRenderPassDescriptor);

impl MTLRenderPassDescriptor {
    pub unsafe fn new() -> MTLRenderPassDescriptor {
        MTLRenderPassDescriptor(retain(msg_send![
            class!(MTLRenderPassDescriptor),
            renderPassDescriptor
        ]))
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/1437970-colorattachments?language=objc).
    pub unsafe fn get_color_attachments(&self) -> MTLRenderPassColorAttachmentDescriptorArray {
        MTLRenderPassColorAttachmentDescriptorArray(msg_send![self.0, colorAttachments])
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/1437973-depthattachment?language=objc).
    pub unsafe fn get_depth_attachment(&self) -> MTLRenderPassDepthAttachmentDescriptor {
        MTLRenderPassDepthAttachmentDescriptor(msg_send![self.0, depthAttachment])
    }
}

/// [Metal docs](https://developer.apple.com/documentation/metal/mtlrendercommandencoder?language=objc).
pub struct MTLRenderCommandEncoder(pub *mut Object);
handle!(MTLRenderCommandEncoder);

impl MTLCommandEncoder for MTLRenderCommandEncoder {
    fn get_obj(&self) -> *mut Object {
        self.0
    }
}

impl MTLRenderCommandEncoder {
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1515811-setrenderpipelinestate?language=objc).
    pub unsafe fn set_render_pipeline_state(&self, state: MTLRenderPipelineState) {
        let _: () = msg_send![self.0, setRenderPipelineState:state.0];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1516029-settrianglefillmode?language=objc).
    /// [MTLTriangleFillMode docs](https://developer.apple.com/documentation/metal/mtltrianglefillmode?language=objc).
    pub unsafe fn set_triangle_fill_mode(&self, fill_mode: u64) {
        let _: () = msg_send![self.0, setTriangleFillMode: fill_mode];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1515499-setfrontfacingwinding?language=objc).
    /// [MTLWinding docs](https://developer.apple.com/documentation/metal/mtlwinding?language=objc).
    pub unsafe fn set_front_facing_winding(&self, face: u64) {
        let _: () = msg_send![self.0, setFrontFacingWinding: face];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1515975-setcullmode?language=objc).
    /// [MTLCullMode docs](https://developer.apple.com/documentation/metal/mtlcullmode?language=objc).
    pub unsafe fn set_cull_mode(&self, cull_mode: u64) {
        let _: () = msg_send![self.0, setCullMode: cull_mode];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1516119-setdepthstencilstate?language=objc).
    pub unsafe fn set_depth_stencil_state(&self, state: MTLDepthStencilState) {
        let _: () = msg_send![self.0, setDepthStencilState:state.0];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1516269-setdepthbias?language=objc).
    pub unsafe fn set_depth_bias(&self, bias: f32, scale: f32, clamp: f32) {
        let _: () = msg_send![self.0, setDepthBias:bias slopeScale:scale clamp:clamp];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1516267-setdepthclipmode?language=objc).
    /// [MTLDepthClipMode docs](https://developer.apple.com/documentation/metal/mtldepthclipmode?language=objc).
    pub unsafe fn set_depth_clip_mode(&self, mode: u64) {
        let _: () = msg_send![self.0, setDepthClipMode: mode];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1515527-setviewport?language=objc).
    /// [MTLViewport docs](https://developer.apple.com/documentation/metal/mtlviewport?language=objc).
    pub unsafe fn set_viewport(&self, viewport: (f64, f64, f64, f64, f64, f64)) {
        let _: () = msg_send![self.0, setViewport: viewport];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1515583-setscissorrect?language=objc).
    /// [MTLScissorRect docs](https://developer.apple.com/documentation/metal/mtlscissorrect?language=objc).
    pub unsafe fn set_scissor_rect(&self, rect: (u64, u64, u64, u64)) {
        let _: () = msg_send![self.0, setScissorRect: rect];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1515592-setblendcolorred?language=objc).
    pub unsafe fn set_blend_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        let _: () = msg_send![self.0, setBlendColorRed:red green:green blue:blue alpha:alpha];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1515556-setvisibilityresultmode?language=objc).
    /// [MTLVisibilityResultMode docs](https://developer.apple.com/documentation/metal/mtlvisibilityresultmode?language=objc).
    pub unsafe fn set_visibility_result_mode(&self, mode: u64, offset: u64) {
        let _: () = msg_send![self.0, setVisibilityResultMode:mode offset:offset];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/3043404-useresource?language=objc).
    /// [MTLResourceUsage docs](https://developer.apple.com/documentation/metal/mtlresourceusage?language=objc).
    /// [MTLRenderStages docs](https://developer.apple.com/documentation/metal/mtlrenderstages?language=objc).
    pub unsafe fn use_resource<T: MTLResource>(&self, resource: T, usage: u64, stages: u64) {
        let _: () = msg_send![self.0, useResource:resource.get_obj() usage:usage stages:stages];
    }
    // TODO add the remainder of the resource usages for argument buffers
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1515829-setvertexbuffer?language=objc).
    pub unsafe fn set_vertex_buffer(&self, buffer: MTLBuffer, offset: u64, index: u64) {
        let _: () = msg_send![self.0, setVertexBuffer:buffer.0 offset:offset atIndex:index];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1515433-setvertexbufferoffset?language=objc).
    pub unsafe fn set_vertex_buffer_offset(&self, offset: u64, index: u64) {
        let _: () = msg_send![self.0, setVertexBufferOffset:offset atIndex:index];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1515846-setvertexbytes?language=objc).
    pub unsafe fn set_vertex_bytes(&self, bytes: *const c_void, length: u64, index: u64) {
        let _: () = msg_send![self.0, setVertexBytes:bytes length:length atIndex:index];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1515537-setvertexsamplerstate?language=objc).
    pub unsafe fn set_vertex_sampler_state(&self, state: MTLSamplerState, index: u64) {
        let _: () = msg_send![self.0, setVertexSamplerState:state.0 atIndex:index];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1515864-setvertexsamplerstate?language=objc).
    pub unsafe fn set_vertex_sampler_state_lod_clamps(
        &self,
        state: MTLSamplerState,
        index: u64,
        lod_min_clamp: f32,
        lod_max_clamp: f32,
    ) {
        let _: () = msg_send![self.0,
            setVertexSamplerState:state.0
            lodMinClamp:lod_min_clamp
            lodMaxClamp:lod_max_clamp
            atIndex:index
        ];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1515842-setvertextexture?language=objc).
    pub unsafe fn set_vertex_texture(&self, texture: MTLTexture, index: u64) {
        let _: () = msg_send![self.0, setVertexTexture:texture.0 atIndex:index];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1515470-setfragmentbuffer?language=objc).
    pub unsafe fn set_fragment_buffer(&self, buffer: MTLBuffer, offset: u64, index: u64) {
        let _: () = msg_send![self.0, setFragmentBuffer:buffer.0 offset:offset atIndex:index];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1515917-setfragmentbufferoffset?language=objc).
    pub unsafe fn set_fragment_buffer_offset(&self, offset: u64, index: u64) {
        let _: () = msg_send![self.0, setFragmentBufferOffset:offset atIndex:index];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1516192-setfragmentbytes?language=objc).
    pub unsafe fn set_fragment_bytes(&self, bytes: *const c_void, length: u64, index: u64) {
        let _: () = msg_send![self.0, setFragmentBytes:bytes length:length atIndex:index];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1515577-setfragmentsamplerstate?language=objc).
    pub unsafe fn set_fragment_sampler_state(&self, state: MTLSamplerState, index: u64) {
        let _: () = msg_send![self.0, setFragmentSamplerState:state.0 atIndex:index];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1515485-setfragmentsamplerstate?language=objc).
    pub unsafe fn set_fragment_sampler_state_lod_clamps(
        &self,
        state: MTLSamplerState,
        index: u64,
        lod_min_clamp: f32,
        lod_max_clamp: f32,
    ) {
        let _: () = msg_send![self.0,
            setFragmentSamplerState:state.0
            lodMinClamp:lod_min_clamp
            lodMaxClamp:lod_max_clamp
            atIndex:index
        ];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1515390-setfragmenttexture?language=objc).
    pub unsafe fn set_fragment_texture(&self, texture: MTLTexture, index: u64) {
        let _: () = msg_send![self.0, setFragmentTexture:texture.0 atIndex:index];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1640035-settessellationfactorbuffer?language=objc).
    pub unsafe fn set_tessellation_factor_buffer(
        &self,
        buffer: MTLBuffer,
        offset: u64,
        instance_stride: u64,
    ) {
        let _: () = msg_send![
            self.0,
            setTessellationFactorBuffer:buffer.0
            offset:offset
            instanceStride:instance_stride
        ];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1639992-settessellationfactorscale?language=objc).
    pub unsafe fn set_tessellation_factor_scale(&self, scale: f32) {
        let _: () = msg_send![self.0, setTessellationFactorScale: scale];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1515561-drawprimitives?language=objc).
    /// [MTLPrimitiveType docs](https://developer.apple.com/documentation/metal/mtlprimitivetype?language=objc).
    pub unsafe fn draw_primitives(
        &self,
        primitive: u64,
        start: u64,
        count: u64,
        instances: u64,
        base_instance: u64,
    ) {
        let _: () = msg_send![
            self.0,
            drawPrimitives:primitive
            vertexStart:start
            vertexCount:count
            instanceCount:instances
            baseInstance:base_instance
        ];
    }
    /// [Metal docs](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1515520-drawindexedprimitives?language=objc).
    /// [MTLPrimitiveType docs](https://developer.apple.com/documentation/metal/mtlprimitivetype?language=objc).
    /// [MTLIndexType docs](https://developer.apple.com/documentation/metal/mtlindextype?language=objc).
    pub unsafe fn draw_indexed_primitives(
        &self,
        primitive: u64,
        count: u64,
        index_type: u64,
        index_buffer: MTLBuffer,
        index_buffer_offset: u64,
        instances: u64,
        base_vertex: u64,
        base_instance: u64,
    ) {
        let _: () = msg_send![
            self.0,
            drawIndexedPrimitives:primitive
            indexCount:count
            indexType:index_type
            indexBuffer:index_buffer.0
            indexBufferOffset:index_buffer_offset
            instanceCount:instances
            baseVertex:base_vertex
            baseInstance:base_instance
        ];
    }
    // TODO add the rest of the drawing methods / tessellation
}
