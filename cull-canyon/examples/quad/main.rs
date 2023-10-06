use cull_canyon::*;
use std::os::raw::c_void;
use winit::platform::macos::WindowBuilderExtMacOS;

unsafe fn execute() {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_inner_size(winit::dpi::PhysicalSize::new(1280, 720))
        .with_resizable(true)
        .with_titlebar_transparent(true)
        .with_title("Quad")
        .build(&event_loop)
        .unwrap();

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

    let library = device
        .new_library_with_data(include_bytes!("quad.metallib"))
        .unwrap();
    // let library = device
    //     .new_library_with_source(include_str!("quad.metal"), MTLCompileOptions::new())
    //     .unwrap();
    let vertex = library.new_function_with_name("vertex_shader").unwrap();
    let fragment = library.new_function_with_name("fragment_shader").unwrap();

    let data = [
        // triangle 1
        -1.0f32, -1.0, 0.0, 1.0, // v1
        1.0, 0.0, 0.0, 1.0, // c1
        -1.0, 1.0, 0.0, 1.0, // v2
        0.0, 0.0, 0.0, 1.0, // c2
        1.0, 1.0, 0.0, 1.0, // v3
        0.0, 0.0, 1.0, 1.0, // c3
        // triangle 2
        1.0, 1.0, 0.0, 1.0, // v3
        0.0, 0.0, 1.0, 1.0, // c3
        1.0, -1.0, 0.0, 1.0, // v4
        0.0, 1.0, 0.0, 1.0, // c4
        -1.0, -1.0, 0.0, 1.0, // v1
        1.0, 0.0, 0.0, 1.0, // c1
    ];
    let buffer = device.new_buffer_with_bytes(
        data.as_ptr() as *const c_void,
        data.len() as u64 * 4, // 4 bytes per f32
        0,
    );

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
                    .set_object_at_indexed_subscript(attrib_2, 1);
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
                        desc
                    },
                    0,
                );
            desc
        })
        .unwrap();

    let command_queue = device.new_command_queue();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Wait;
        match event {
            winit::event::Event::RedrawRequested(_window) => {
                if let Some(drawable) = surface.next_drawable() {
                    let command_buffer = command_queue.new_command_buffer();

                    let render_pass_desc = MTLRenderPassDescriptor::new();
                    let color_attachment = MTLRenderPassColorAttachmentDescriptor::new();
                    color_attachment.set_load_action(2);
                    color_attachment.set_store_action(1);
                    color_attachment.set_texture(drawable.get_texture());
                    render_pass_desc
                        .get_color_attachments()
                        .set_object_at_indexed_subscript(0, color_attachment);

                    let render_encoder =
                        command_buffer.new_render_command_encoder_with_descriptor(render_pass_desc);
                    render_encoder.set_render_pipeline_state(render_pipeline.clone());
                    render_encoder.set_vertex_buffer(buffer.clone(), 0, 0);
                    render_encoder.draw_primitives(3, 0, 6, 1, 0);
                    render_encoder.end_encoding();

                    command_buffer.present_drawable(drawable);
                    command_buffer.commit();
                }
            }
            winit::event::Event::NewEvents(_cause) => {
                window.request_redraw();
            }
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

fn main() {
    unsafe {
        execute();
    };
}
