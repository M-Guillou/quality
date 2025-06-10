mod combustion_event;
mod engine_sim;
mod audio;
mod audio_engine;
mod waveguide;

use crossbeam::channel;
use std::thread;
use std::time::Duration;

fn main() {
    let (event_sender, event_receiver) = channel::unbounded();

    // Start engine sim thread
    engine_sim::start_engine_sim_thread(event_sender);

    // Start audio device
    let sdl_context = sdl2::init().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();
    let _audio_device = audio::start_audio_device(&audio_subsystem, event_receiver);

    // Main loop (placeholder)
    loop {
        // For now just sleep, we'll later add Vulkan rendering here
        thread::sleep(Duration::from_secs(1));
    }

}