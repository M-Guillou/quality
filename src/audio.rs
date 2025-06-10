use crate::combustion_event::{CombustionEvent};
use sdl2::audio::{AudioCallback, AudioSpecDesired};
use crossbeam::channel;

use crate::audio_engine::{AudioEngine}; // You can put AudioEngine in its own module if you want

pub fn start_audio_device(audio_subsystem: &sdl2::AudioSubsystem, event_receiver: channel::Receiver<CombustionEvent>) -> sdl2::audio::AudioDevice<EngineAudioCallback> {
    let desired_spec = AudioSpecDesired {
        freq: Some(48000),
        channels: Some(1),
        samples: Some(512),
    };

    let audio_device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        println!("Opened audio device at spec: {:?}", spec);

        EngineAudioCallback {
            engine: AudioEngine::new(event_receiver, spec.freq as f32),
        }
    }).unwrap();

    audio_device.resume();
    audio_device
}

pub struct EngineAudioCallback {
    engine: AudioEngine,
}

impl AudioCallback for EngineAudioCallback {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        self.engine.callback(out);
    }
}
