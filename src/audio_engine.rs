use std::collections::VecDeque;

use crate::{combustion_event::{generate_pressure_pulse, CombustionEvent}, waveguide::{WaveguideChain, WaveguideSection}};

pub struct ActiveCombustionPulse {
    samples: Vec<f32>,
    position: usize,
}

pub struct AudioEngine {
    event_receiver: crossbeam::channel::Receiver<CombustionEvent>,
    pulse_queue: VecDeque<ActiveCombustionPulse>,
    waveguide: WaveguideChain,
    sample_rate: f32,
}

impl AudioEngine {
    pub fn new(event_receiver: crossbeam::channel::Receiver<CombustionEvent>, sample_rate: f32) -> Self {
        let c = 343.0; // speed of sound m/s

        let section1 = WaveguideSection::new((0.3 / c * sample_rate) as usize, -0.8, 0.7); // header
        let section2 = WaveguideSection::new((0.5 / c * sample_rate) as usize, -0.9, 0.6); // pipe
        let section3 = WaveguideSection::new((0.6 / c * sample_rate) as usize, -0.7, 0.5); // muffler
        let section4 = WaveguideSection::new((1.0 / c * sample_rate) as usize, -0.6, 0.0); // tailpipe → open end

        let waveguide_chain = WaveguideChain {
            sections: vec![section1, section2, section3, section4],
        };

        Self {
            event_receiver,
            pulse_queue: VecDeque::new(),
            waveguide: waveguide_chain,
            sample_rate,
        }
    }

    pub fn callback(&mut self, out: &mut [f32]) {
        // Receive new combustion events and generate pulses
        while let Ok(event) = self.event_receiver.try_recv() {
            println!("Received CombustionEvent for audio processing: {:?}", event.rpm);

            let samples = generate_pressure_pulse(
                self.sample_rate,
                event.pressure_at_valve,
                event.load,
                event.rpm,
                event.valve_open_duration,
                &event.valve_lift_profile,
                event.piston_speed,
                &event.exhaust_flow_velocity,
                &event.temperature_curve,
            );

            self.pulse_queue.push_back(ActiveCombustionPulse {
                samples,
                position: 0,
            });
        }

        // Fill output buffer sample by sample
        for sample in out.iter_mut() {
            let mut input_left = 0.0;

            // Mix all active pulses:
            for pulse in self.pulse_queue.iter_mut() {
                if pulse.position < pulse.samples.len() {
                    input_left += pulse.samples[pulse.position];
                    pulse.position += 1;
                }
            }

            // Remove finished pulses:
            self.pulse_queue.retain(|p| p.position < p.samples.len());

            // Drive waveguide
            let pressure = self.waveguide.step(input_left);

            *sample = pressure * 1.0;
        }
    }
}
