use crate::wave::bundles::basemetal::BaseMetalBundle;
use crate::wave::constants::VERTEX_COUNT;
use cull_canyon::{
    MTLBuffer, MTLComputePipelineState, MTLRenderPipelineColorAttachmentDescriptor,
    MTLRenderPipelineDescriptor, MTLRenderPipelineState, MTLSamplerDescriptor, MTLSamplerState,
    MTLTexture, MTLTextureDescriptor, MTLVertexDescriptor,
};
use std::os::raw::c_void;

pub struct WaterBundle {
    pub render_pipeline: MTLRenderPipelineState,
    pub compute_pipeline: MTLComputePipelineState,
    pub water_buffer: MTLBuffer,
    pub water_indices: MTLBuffer,
    pub indices_count: usize,
    pub water_surface: MTLTexture,
    pub texture: MTLTexture,
    pub crosshair: MTLTexture,
    pub sampler: MTLSamplerState,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Wave {
    // x | 1 == up
    // x | 2 == down
    // x | 4 == left
    // x | 8 == right
    pub directions: u8,
    // in vertices
    pub wavelength: u8,
    // amplitude of the wave; may be negative
    // amplitude is calculated in the vertex shader
    pub amplitude_factor: f32,
}

impl Wave {
    pub fn empty() -> Wave {
        Wave {
            directions: 0,
            wavelength: 0,
            amplitude_factor: 0.0,
        }
    }
}

impl WaterBundle {
    pub unsafe fn generate_water(bundle: &BaseMetalBundle) -> WaterBundle {
        // row by row generation
        const HL: f32 = VERTEX_COUNT as f32 / 2.0;
        const DIMENSIONS: usize = 2;

        let vertices: [f32; (2 * VERTEX_COUNT * VERTEX_COUNT) as usize] = *((0..VERTEX_COUNT)
            .map(|z: u32| {
                (0..VERTEX_COUNT)
                    .map(|x: u32| [x as f32 - HL, z as f32 - HL])
                    .collect::<Vec<[f32; DIMENSIONS]>>()
            })
            .collect::<Vec<Vec<[f32; DIMENSIONS]>>>()
            .into_iter()
            .flatten()
            .collect::<Vec<[f32; DIMENSIONS]>>()
            .as_ptr()
            as *const [f32; (2 * VERTEX_COUNT * VERTEX_COUNT) as usize]);

        const INDICES_COUNT: usize = (6 * (VERTEX_COUNT - 1) * (VERTEX_COUNT - 1)) as usize;
        let mut indices: [u32; INDICES_COUNT] = [0; INDICES_COUNT];
        let mut pointer = 0;
        (0..VERTEX_COUNT - 1).for_each(|z| {
            (0..VERTEX_COUNT - 1).for_each(|x| {
                let top_left = z * VERTEX_COUNT + x;
                let top_right = top_left + 1;
                let bottom_left = (z + 1) * VERTEX_COUNT + x;
                let bottom_right = bottom_left + 1;
                indices[pointer] = top_left;
                pointer += 1;
                indices[pointer] = bottom_left;
                pointer += 1;
                indices[pointer] = top_right;
                pointer += 1;
                indices[pointer] = top_right;
                pointer += 1;
                indices[pointer] = bottom_left;
                pointer += 1;
                indices[pointer] = bottom_right;
                pointer += 1;
            });
        });

        let render_pipeline = bundle
            .device
            .new_render_pipeline_state_with_descriptor({
                let desc = MTLRenderPipelineDescriptor::new();
                desc.get_color_attachments()
                    .set_object_at_indexed_subscript(
                        {
                            let desc = MTLRenderPipelineColorAttachmentDescriptor::new();
                            desc.set_pixel_format(80); // bgra8unorm
                            desc
                        },
                        0,
                    );
                desc.set_vertex_function(
                    bundle.library.new_function_with_name("water_vert").unwrap(),
                );
                desc.set_fragment_function(
                    bundle.library.new_function_with_name("water_frag").unwrap(),
                );
                desc.set_depth_attachment_pixel_format(252); // depth 32 float
                desc.set_vertex_descriptor(MTLVertexDescriptor::new());
                desc
            })
            .unwrap();

        let compute_pipeline = bundle
            .device
            .new_compute_pipeline_state_with_function(
                bundle
                    .library
                    .new_function_with_name("process_water")
                    .unwrap(),
            )
            .unwrap();

        let texture = bundle.device.new_texture_with_descriptor({
            let desc = MTLTextureDescriptor::new();
            desc.set_width(VERTEX_COUNT as u64);
            desc.set_height(VERTEX_COUNT as u64);
            desc.set_pixel_format(113); // 113 = rgba16uint; 63 = rg16uint
            desc.set_texture_type(2); // 2d
            desc.set_usage(0x0001 | 0x0002); // shader read + write
            desc
        });
        texture.replace_region(
            (0, 0, VERTEX_COUNT as u64, VERTEX_COUNT as u64),
            0,
            [[0u16, 0, 0, 0]; VERTEX_COUNT as usize * VERTEX_COUNT as usize].as_ptr()
                as *mut c_void,
            VERTEX_COUNT as u64 * 8,
        );

        let crosshair = bundle.device.new_texture_with_descriptor({
            let desc = MTLTextureDescriptor::new();
            desc.set_width(5);
            desc.set_height(5);
            desc.set_pixel_format(70); // rgba8unorm
            desc.set_texture_type(2);
            desc
        });
        let decoder = png::Decoder::new(include_bytes!("crosshair.png") as &[u8]);
        let (info, mut reader) = decoder.read_info().unwrap();
        let mut img = vec![0; info.buffer_size()];
        reader.next_frame(&mut img).unwrap();
        crosshair.replace_region((0, 0, 5, 5), 0, img.as_ptr() as *mut c_void, 20);

        let surface = bundle.device.new_texture_with_descriptor({
            let desc = MTLTextureDescriptor::new();
            desc.set_width(384);
            desc.set_height(384);
            desc.set_pixel_format(70);
            desc.set_texture_type(2);
            desc
        });
        let decoder = png::Decoder::new(include_bytes!("thatiswater.png") as &[u8]);
        let (info, mut reader) = decoder.read_info().unwrap();
        let mut img = vec![0; info.buffer_size()];
        reader.next_frame(&mut img).unwrap();
        surface.replace_region((0, 0, 384, 384), 0, img.as_ptr() as *mut c_void, 3 * 512);

        WaterBundle {
            render_pipeline,
            compute_pipeline,
            water_buffer: bundle.device.new_buffer_with_bytes(
                vertices.as_ptr() as *const c_void,
                vertices.len() as u64 * 4,
                0,
            ),
            water_indices: bundle.device.new_buffer_with_bytes(
                indices.as_ptr() as *const c_void,
                indices.len() as u64 * 4,
                0,
            ),
            indices_count: INDICES_COUNT,
            water_surface: surface,
            texture,
            crosshair,
            sampler: bundle
                .device
                .new_sampler_state_with_descriptor(MTLSamplerDescriptor::new()),
        }
    }
}
