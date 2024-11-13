use crate::behavior::Behavior;
use crate::wave::bundles::ui::UiBundle;
use crate::wave::bundles::water::{WaterBundle, Wave};
use crate::wave::constants::{CAMERA_SPEED, FILL_MODE, FREQ_OF_UPDATES, VERTEX_COUNT};
use crate::wave::raycaster::cast_ray;
use crate::wave::util::generate_transformation;
use crate::wave::WaveApp;
use cgmath::{Matrix4, Vector3};
use cull_canyon::{
    MTLCommandEncoder, MTLRenderPassAttachmentDescriptor, MTLRenderPassColorAttachmentDescriptor,
    MTLRenderPassDescriptor,
};
use std::os::raw::c_void;
use winit::event::{ElementState, VirtualKeyCode};

pub struct MainBehavior;
impl Behavior<WaveApp> for MainBehavior {
    fn init(&self, state: &mut WaveApp) {
        state.ui_bundle = Some(unsafe { UiBundle::new(state.base_metal_bundle.as_ref().unwrap()) });
        state.water = Some(unsafe {
            WaterBundle::generate_water(&state.base_metal_bundle.as_ref().unwrap())
        });
    }

    fn update(&self, state: &mut WaveApp) -> Option<Box<dyn Behavior<WaveApp>>> {
        state
            .window_bundle
            .as_ref()
            .unwrap()
            .window
            .request_redraw();

        let window_size = state.window_bundle.as_ref().unwrap().window.inner_size();
        let pitch =
            (window_size.height as f64 / 2.0) - (window_size.height as f64 - state.mouse_pos.1);
        let yaw = -(window_size.width as f64 - state.mouse_pos.0 / 2.0);

        let pitch = if pitch >= 90.0 { 90.0 } else { pitch };
        let pitch = if pitch <= -90.0 { -90.0 } else { pitch };

        let pitch = pitch.to_radians() as f32;
        let yaw = yaw.to_radians() as f32;

        let cam = &mut state.matrix_bundle.as_mut().unwrap().camera;
        cam.pitch = pitch;
        cam.yaw = yaw;

        if state.keyboard.is_key_down(VirtualKeyCode::W) {
            cam.z -= yaw.cos() * CAMERA_SPEED;
            cam.x += yaw.sin() * CAMERA_SPEED;
        };
        if state.keyboard.is_key_down(VirtualKeyCode::S) {
            cam.z += yaw.cos() * CAMERA_SPEED;
            cam.x -= yaw.sin() * CAMERA_SPEED;
        };
        if state.keyboard.is_key_down(VirtualKeyCode::D) {
            cam.z += yaw.sin() * CAMERA_SPEED;
            cam.x += yaw.cos() * CAMERA_SPEED;
        };
        if state.keyboard.is_key_down(VirtualKeyCode::A) {
            cam.z -= yaw.sin() * CAMERA_SPEED;
            cam.x -= yaw.cos() * CAMERA_SPEED;
        };
        if state.keyboard.is_key_down(VirtualKeyCode::Space) {
            cam.y += CAMERA_SPEED;
        };
        if state.keyboard.is_key_down(VirtualKeyCode::LShift) {
            cam.y -= CAMERA_SPEED;
        };

        if state.keyboard.is_key_down(VirtualKeyCode::P) {
            state.paused = true;
        }
        if state.keyboard.is_key_down(VirtualKeyCode::L) {
            state.paused = false;
        }

        unsafe { state.matrix_bundle.as_ref().unwrap().edit_view() };

        None
    }

    fn draw(&self, state: &mut WaveApp) {
        let bundle = state.base_metal_bundle.as_ref().unwrap();
        let ui = state.ui_bundle.as_ref().unwrap();
        let water = state.water.as_ref().unwrap();
        let debug = state.debug_bundle.as_ref().unwrap();
        let matrices = state.matrix_bundle.as_ref().unwrap();

        unsafe {
            if let Some(drawable) = bundle.surface.next_drawable() {
                let command_buffer = bundle.queue.new_command_buffer();

                let encoder = command_buffer.new_render_command_encoder_with_descriptor({
                    let desc = MTLRenderPassDescriptor::new();
                    {
                        let desc = desc.get_depth_attachment();
                        desc.set_texture(bundle.depth_texture.clone());
                        desc.set_load_action(2);
                        desc.set_store_action(1);
                    };
                    desc.get_color_attachments()
                        .set_object_at_indexed_subscript(0, {
                            let desc = MTLRenderPassColorAttachmentDescriptor::new();
                            desc.set_texture(drawable.get_texture());
                            desc.set_clear_color(0.0, 0.0, 0.0, 1.0);
                            desc.set_load_action(2);
                            desc.set_store_action(1);
                            desc
                        });
                    desc
                });
                encoder.set_render_pipeline_state(water.render_pipeline.clone());
                encoder.set_vertex_buffer(water.water_buffer.clone(), 0, 0);
                encoder.set_vertex_buffer(matrices.projection.clone(), 0, 1);
                encoder.set_vertex_buffer(matrices.view.clone(), 0, 2);
                encoder.set_vertex_bytes(
                    state.waves.as_ptr() as *const c_void,
                    state.waves.len() as u64 * std::mem::size_of::<Wave>() as u64,
                    3,
                );
                encoder.set_triangle_fill_mode(FILL_MODE);
                encoder.set_depth_stencil_state(bundle.basic_depth.clone());
                encoder.set_vertex_texture(water.texture.clone(), 0);
                encoder.set_fragment_texture(water.water_surface.clone(), 0);
                encoder.set_fragment_sampler_state(water.sampler.clone(), 0);

                encoder.draw_indexed_primitives(
                    3,
                    water.indices_count as u64,
                    1,
                    water.water_indices.clone(),
                    0,
                    1,
                    0,
                    0,
                );

                let point = cast_ray(
                    matrices.proj_contents,
                    &matrices.camera,
                    water.texture.clone(),
                    state,
                );
                if let Some(point) = point {
                    state.current_ray_pos = point;
                    encoder.set_render_pipeline_state(debug.pipeline.clone());
                    encoder.set_vertex_buffer(debug.vertices.clone(), 0, 0);
                    encoder.set_vertex_buffer(matrices.projection.clone(), 0, 1);
                    encoder.set_vertex_buffer(matrices.view.clone(), 0, 2);
                    let transformation = generate_transformation(
                        Vector3 {
                            x: point.x,
                            y: point.y + 1.0,
                            z: point.z,
                        },
                        (0.0, 0.0, 0.0),
                        (1.0, 1.0, 1.0),
                    );
                    encoder.set_vertex_bytes(
                        &transformation as *const Matrix4<f32> as *const c_void,
                        64,
                        3,
                    );
                    encoder.set_depth_stencil_state(bundle.basic_depth.clone());
                    encoder.draw_indexed_primitives(
                        3,
                        debug.indices_count,
                        0,
                        debug.indices.clone(),
                        0,
                        1,
                        0,
                        0,
                    );
                }

                encoder.set_render_pipeline_state(ui.pipeline.clone());
                encoder.set_vertex_buffer(ui.quad.clone(), 0, 0);
                let aspect_ratio = {
                    let b = state.window_bundle.as_ref().unwrap().window.inner_size();
                    b.width as f32 / b.height as f32
                };
                encoder.set_vertex_bytes(
                    [0.0f32, 0.0, 0.05, 0.05 * aspect_ratio].as_ptr() as *const c_void,
                    16,
                    1,
                );
                encoder.set_fragment_texture(water.crosshair.clone(), 0);
                encoder.set_fragment_sampler_state(water.sampler.clone(), 0);
                encoder.draw_primitives(3, 0, 6, 1, 0);

                encoder.end_encoding();

                if !state.paused && state.time != 0 && state.time % FREQ_OF_UPDATES == 0 {
                    let encoder = command_buffer.new_compute_command_encoder();
                    encoder.set_compute_pipeline_state(water.compute_pipeline.clone());
                    encoder.set_bytes(
                        state.waves.as_ptr() as *const c_void,
                        state.waves.len() as u64 * std::mem::size_of::<Wave>() as u64,
                        0,
                    );
                    encoder.set_texture(water.texture.clone(), 0);
                    encoder.set_texture(water.texture.clone(), 1);
                    encoder.dispatch_threadgroups(
                        (VERTEX_COUNT as u64 / 10, VERTEX_COUNT as u64 / 10, 1),
                        (10, 10, 1),
                    );
                    encoder.end_encoding();
                };

                command_buffer.present_drawable(drawable);
                command_buffer.commit();
            }
        };
    }

    fn on_resize(&self, state: &mut WaveApp, size: (u32, u32)) {
        unsafe {
            state
                .matrix_bundle
                .as_ref()
                .unwrap()
                .edit_projection(size.0 as f32 / size.1 as f32)
        };
    }

    fn on_death(&self, _state: &mut WaveApp) {
        //
    }

    fn on_keyboard_update(&self, state: &mut WaveApp, key: VirtualKeyCode, el_state: ElementState) {
        if el_state == ElementState::Pressed {
            match key {
                VirtualKeyCode::G => {
                    println!("Let's build a wave!");
                    println!(
                        "This wave will NOT be placed on the scene.\
                        We will JUST be filling the wave slot."
                    );
                    println!(
                        "To abort wave production, just give \"abort\" as an input at any \
                        time, and we'll abort."
                    );
                    let mut s;

                    let mut amplitude: f32 = 0.0;
                    let mut wavelength: u8 = 0;
                    let directions: u8;

                    println!("Enter the desired amplitude.");
                    loop {
                        s = String::new();
                        std::io::stdin().read_line(&mut s).unwrap();
                        s = s.trim().to_string();
                        if s.to_lowercase().eq(&"abort".to_string()) {
                            println!("Aborting.");
                            return;
                        };
                        let value = s.parse::<f32>();
                        let b = match value {
                            Ok(val) => {
                                if val < -50.0 || val > 50.0 {
                                    println!(
                                        "Amplitude {} is invalid; -50 <= amplitude <= 50.",
                                        val
                                    );
                                    false
                                } else {
                                    amplitude = val;
                                    true
                                }
                            }
                            Err(_) => {
                                println!("Invalid amplitude value {}.", s);
                                false
                            }
                        };
                        if b {
                            break;
                        };
                    }
                    println!("Amplitude = {}", amplitude);

                    println!("Enter the desired wavelength.");
                    loop {
                        s = String::new();
                        std::io::stdin().read_line(&mut s).unwrap();
                        s = s.trim().to_string();
                        if s.to_lowercase().eq(&"abort".to_string()) {
                            println!("Aborting.");
                            return;
                        };
                        let value = s.parse::<u8>();
                        let b = match value {
                            Ok(val) => {
                                wavelength = val;
                                true
                            }
                            Err(_) => {
                                println!("Invalid wavelength value {}.", s);
                                false
                            }
                        };
                        if b {
                            break;
                        };
                    }
                    println!("Wavelength = {}", wavelength);

                    println!("Enter the desired directions of propagation for the wave.");
                    println!("Valid directions are up, left, right, and down.");
                    println!("Direction instructions should be formatted like: \"up right\".");
                    println!(
                        "Warning: if you input two opposed directions (ie. left and right) \
                         for the same wave, some very not-wavelike chaos ensues. If \
                         you want a wave to propagate in all directions, I recommend \
                         making several waves to achieve that effect."
                    );
                    s = String::new();
                    std::io::stdin().read_line(&mut s).unwrap();
                    s = s.trim().to_string();
                    if s.to_lowercase().eq(&"abort".to_string()) {
                        println!("Aborting.");
                        return;
                    };
                    let mut up = false;
                    let mut down = false;
                    let mut right = false;
                    let mut left = false;
                    let parts = s.split(" ").collect::<Vec<&str>>();
                    parts.iter().for_each(|item: &&str| {
                        match *item {
                            "up" => up = true,
                            "left" => left = true,
                            "down" => down = true,
                            "right" => right = true,
                            _ => {}
                        };
                    });
                    println!(
                        "Your wave will{}go up.",
                        match up {
                            true => " ",
                            false => " not ",
                        }
                    );
                    println!(
                        "Your wave will{}go left.",
                        match left {
                            true => " ",
                            false => " not ",
                        }
                    );
                    println!(
                        "Your wave will{}go right.",
                        match right {
                            true => " ",
                            false => " not ",
                        }
                    );
                    println!(
                        "Your wave will{}go down.",
                        match down {
                            true => " ",
                            false => " not ",
                        }
                    );

                    directions = match up {
                        true => 1,
                        false => 0,
                    } | match down {
                        true => 2,
                        false => 0,
                    } | match left {
                        true => 4,
                        false => 0,
                    } | match right {
                        true => 8,
                        false => 0,
                    };

                    println!("Enter the desired wave slot.");
                    let mut wave_id = 0;
                    loop {
                        s = String::new();
                        std::io::stdin().read_line(&mut s).unwrap();
                        s = s.trim().to_string();
                        if s.to_lowercase().eq(&"abort".to_string()) {
                            println!("Aborting.");
                            return;
                        };
                        let value = s.parse::<usize>();
                        let b = match value {
                            Ok(val) => {
                                if val > 3 {
                                    println!("Invalid wave id {}; 0 <= id <= 3", val);
                                    false
                                } else {
                                    wave_id = val;
                                    true
                                }
                            }
                            Err(_) => {
                                println!("Invalid wave id {}.", s);
                                false
                            }
                        };
                        if b {
                            break;
                        };
                    }

                    println!("Wave id {}", wave_id);

                    // TODO fix the bug with waves going in all directions not working

                    state.waves[wave_id].wavelength = wavelength;
                    state.waves[wave_id].amplitude_factor = amplitude;
                    state.waves[wave_id].directions = directions;

                    println!("Done!");
                }
                VirtualKeyCode::N => {
                    println!("Alright, let's place a wave!");
                    println!("We will place a wave on the tile you are currently pointing at.");
                    println!(
                        "To abort wave placement, just give \"abort\" as an input at any \
                        time, and we'll abort."
                    );
                    let mut s;
                    let mut wave_id = 0;
                    loop {
                        println!("Pick a wave slot.");
                        println!("If a wave slot picked is empty, the wave will not be placed.");
                        s = String::new();
                        std::io::stdin().read_line(&mut s).unwrap();
                        s = s.trim().to_string();
                        if s.to_lowercase().eq(&"abort".to_string()) {
                            println!("Aborting.");
                            return;
                        };
                        let value = s.parse::<usize>();
                        let b = match value {
                            Ok(val) => {
                                if val > 3 {
                                    println!("Invalid wave id {}; 0 <= id <= 3", val);
                                    false
                                } else {
                                    wave_id = val;
                                    true
                                }
                            }
                            Err(_) => {
                                println!("Invalid wave id {}.", s);
                                false
                            }
                        };
                        if b {
                            break;
                        };
                    }

                    println!("Wave id {}", wave_id);

                    let normalized_ray_coords = (
                        (state.current_ray_pos.x + (VERTEX_COUNT / 2) as f32) as u64,
                        VERTEX_COUNT as u64
                            - (state.current_ray_pos.z + VERTEX_COUNT as f32 / 2.0) as u64,
                    );

                    let mut k = [0, 0, 0, 0];
                    k[wave_id] = 1u16 << 8;
                    unsafe {
                        state.water.as_ref().unwrap().texture.replace_region(
                            (normalized_ray_coords.0, normalized_ray_coords.1, 1, 1),
                            0,
                            k.as_ptr() as *mut c_void,
                            VERTEX_COUNT as u64 * 8,
                        );
                    };

                    println!("Done!");
                }
                VirtualKeyCode::R => unsafe {
                    FILL_MODE = !FILL_MODE;
                },
                _ => {}
            }
        }
    }
}
