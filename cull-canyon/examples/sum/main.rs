use cull_canyon::*;
use std::os::raw::c_void;

unsafe fn execute() {
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

    // let library = device
    //     .new_library_with_source(include_str!("sum.metal"), MTLCompileOptions::new())
    //     .unwrap();
    let library = device
        .new_library_with_data(include_bytes!("sum.metallib"))
        .unwrap();
    let kernel = library.new_function_with_name("sum").unwrap();

    let compute_pipeline = device
        .new_compute_pipeline_state_with_function(kernel)
        .unwrap();

    let in_data = [
        1u32, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29,
    ];
    let in_buffer = device.new_buffer_with_bytes(
        in_data.as_ptr() as *const c_void,
        in_data.len() as u64 * 4,
        0,
    );
    let out_buffer = device.new_buffer_with_length(1, 0);

    let queue = device.new_command_queue();
    let command_buffer = queue.new_command_buffer();
    let compute_encoder = command_buffer.new_compute_command_encoder();

    compute_encoder.set_buffer(in_buffer.clone(), 0, 0);
    compute_encoder.set_buffer(out_buffer.clone(), 0, 1);
    compute_encoder.set_compute_pipeline_state(compute_pipeline.clone());
    compute_encoder.dispatch_threads((in_data.len() as u64, 1, 1), (in_data.len() as u64, 1, 1));
    compute_encoder.end_encoding();

    command_buffer.commit();
    command_buffer.wait_until_completed();

    let contents = *(out_buffer.get_contents() as *const u32);
    println!("The sum: {:?}.", contents);
}

fn main() {
    unsafe {
        execute();
    };
}
