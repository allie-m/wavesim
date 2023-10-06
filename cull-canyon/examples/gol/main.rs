use cull_canyon::*;
use std::fs::File;
use std::os::raw::c_void;
use std::time::{Duration, Instant};
use winit::platform::macos::WindowBuilderExtMacOS;

// John Conway's Game of Life (GoL); RIP John Conway

fn main() {
    unsafe {
        execute();
    };
}

const WINDOW_SCALE_FACTOR: f32 = 10.0;
const FPS: f32 = 30.0;

unsafe fn execute() {
    let decoder = png::Decoder::new(File::open("examples/gol/gol_init_state.png").unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut img = vec![0; info.buffer_size()];
    reader.next_frame(&mut img).unwrap();

    let gol_size = (info.width, info.height);

    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_inner_size(winit::dpi::PhysicalSize::new(
            (gol_size.0 as f32 * WINDOW_SCALE_FACTOR) as u32,
            (gol_size.1 as f32 * WINDOW_SCALE_FACTOR) as u32,
        ))
        .with_resizable(false)
        .with_titlebar_transparent(true)
        .with_title("Conway's Game of Life")
        .build(&event_loop)
        .unwrap();

    // find a suitable device
    let device = {
        #[cfg(target_os = "macos")]
        {
            MTLDevice::copy_all_devices()
                .into_iter()
                .find_map(|d| Some(d))
                .unwrap()
        }
        #[cfg(target_os = "ios")]
        {
            MTLDevice::create_system_default_device()
        }
    };
    println!("Device: {}.", device.get_name());
    println!(
        "Device is {}.",
        match device.is_low_power() {
            true => "integrated",
            false => "discrete",
        }
    );

    let surface = CAMetalLayer::new();
    surface.set_device(device.clone());
    surface.set_display_sync_enabled(true); // vsync enabled
    surface.set_pixel_format(80); // bgra8unorm
    surface.set_presents_with_transaction(false);
    surface.set_drawable_size(1280.0, 720.0);
    set_layer_for_raw_window_handle(surface.clone(), &window);

    let data = [
        // remember that the bottom left it 0, 1
        // triangle 1
        -1.0f32, -1.0, 0.0, 1.0, // v1
        0.0, 1.0, 0.0, 0.0, // c1
        -1.0, 1.0, 0.0, 1.0, // v2
        0.0, 0.0, 0.0, 0.0, // c2
        1.0, 1.0, 0.0, 1.0, // v3
        1.0, 0.0, 0.0, 0.0, // c3
        // triangle 2
        1.0, 1.0, 0.0, 1.0, // v3
        1.0, 0.0, 0.0, 0.0, // c3
        1.0, -1.0, 0.0, 1.0, // v4
        1.0, 1.0, 0.0, 0.0, // c4
        -1.0, -1.0, 0.0, 1.0, // v1
        0.0, 1.0, 0.0, 0.0, // c1
    ];
    let buffer = device.new_buffer_with_bytes(
        data.as_ptr() as *const c_void,
        data.len() as u64 * 4, // 4 bytes per f32
        0,
    );

    let library = device
        .new_library_with_source(include_str!("gol.metal"), MTLCompileOptions::new())
        .unwrap();

    let vertex = library.new_function_with_name("vertex_shader").unwrap();
    let fragment = library.new_function_with_name("fragment_shader").unwrap();
    let kernel = library.new_function_with_name("update_game").unwrap();

    let sampler = device.new_sampler_state_with_descriptor({
        let desc = MTLSamplerDescriptor::new();
        desc
    });
    let texture = device.new_texture_with_descriptor({
        let desc = MTLTextureDescriptor::new();
        desc.set_width(gol_size.0 as u64);
        desc.set_height(gol_size.1 as u64);
        desc.set_pixel_format(70); // rgba8unorm
        desc.set_texture_type(2); // texture 2d
        desc
    });

    texture.replace_region(
        (0, 0, info.width as u64, info.height as u64),
        0,
        img.as_mut_ptr() as *mut c_void,
        4 * info.width as u64, // 4 because rgba8unorm is 4 bytes per pixel
    );

    let compute_pipeline = device
        .new_compute_pipeline_state_with_function(kernel)
        .unwrap();
    let render_pipeline = device
        .new_render_pipeline_state_with_descriptor({
            let desc = MTLRenderPipelineDescriptor::new();
            desc.set_vertex_function(vertex);
            desc.set_fragment_function(fragment);
            desc.set_vertex_descriptor({
                let desc = MTLVertexDescriptor::new();
                let attrib_1 = MTLVertexAttributeDescriptor::new();
                attrib_1.set_offset(0);
                attrib_1.set_buffer_index(0);
                attrib_1.set_format(31);
                let attrib_2 = MTLVertexAttributeDescriptor::new();
                attrib_2.set_offset(16);
                attrib_2.set_buffer_index(0);
                attrib_2.set_format(31);
                desc.get_attributes()
                    .set_object_at_indexed_subscript(attrib_1, 0);
                desc.get_attributes()
                    .set_object_at_indexed_subscript(attrib_2, 0);
                let layout_1 = MTLVertexBufferLayoutDescriptor::new();
                layout_1.set_stride(16);
                layout_1.set_step_function(1); // per vertex
                layout_1.set_step_rate(1);
                desc.get_layouts()
                    .set_object_at_indexed_subscript(layout_1, 0);
                desc
            });
            desc.get_color_attachments()
                .set_object_at_indexed_subscript(
                    {
                        let desc = MTLRenderPipelineColorAttachmentDescriptor::new();
                        desc.set_pixel_format(80); // bgra8unorm
                        desc.set_blending_enabled(true);
                        desc.set_source_rgb_blend_factor(4); // src alpha
                        desc.set_destination_rgb_blend_factor(5); // 1 - src alpha
                        desc.set_source_alpha_blend_factor(1); // 1
                        desc.set_destination_alpha_blend_factor(5); // 1 - src alpha
                        desc
                    },
                    0,
                );
            desc
        })
        .unwrap();

    let command_queue = device.new_command_queue();
    let duration = Duration::from_millis((1000.0 / FPS) as u64);
    let mut now = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::WaitUntil(now + duration);
        match event {
            winit::event::Event::RedrawRequested(_window) => {
                if let Some(drawable) = surface.next_drawable() {
                    let command_buffer = command_queue.new_command_buffer();

                    let render_pass_desc = MTLRenderPassDescriptor::new();
                    let color_attachment = MTLRenderPassColorAttachmentDescriptor::new();
                    color_attachment.set_load_action(2);
                    color_attachment.set_store_action(1);
                    color_attachment.set_texture(drawable.get_texture());
                    color_attachment.set_clear_color(0.0, 0.0, 0.0, 1.0);
                    render_pass_desc
                        .get_color_attachments()
                        .set_object_at_indexed_subscript(0, color_attachment);

                    let render_encoder =
                        command_buffer.new_render_command_encoder_with_descriptor(render_pass_desc);
                    render_encoder.set_render_pipeline_state(render_pipeline.clone());
                    render_encoder.set_vertex_buffer(buffer.clone(), 0, 0);
                    render_encoder.set_fragment_sampler_state(sampler.clone(), 0);
                    render_encoder.set_fragment_texture(texture.clone(), 0);
                    render_encoder.draw_primitives(3, 0, 6, 1, 0);
                    render_encoder.end_encoding();

                    let compute_encoder = command_buffer.new_compute_command_encoder();
                    compute_encoder.set_compute_pipeline_state(compute_pipeline.clone());
                    compute_encoder.set_texture(texture.clone(), 0);
                    compute_encoder.set_texture(texture.clone(), 1);
                    compute_encoder.dispatch_threadgroups(
                        (gol_size.0 as u64 / 10, gol_size.1 as u64 / 10, 1),
                        (10, 10, 1),
                    );
                    compute_encoder.end_encoding();

                    command_buffer.present_drawable(drawable);
                    command_buffer.commit();
                }
            }
            winit::event::Event::NewEvents(cause) => match cause {
                winit::event::StartCause::ResumeTimeReached {
                    start: _,
                    requested_resume: _,
                } => {
                    window.request_redraw();
                    now = Instant::now();
                }
                _ => {}
            },
            winit::event::Event::WindowEvent {
                window_id: _,
                event,
            } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }
                _ => {}
            },
            _ => {}
        }
    });
}
