use crossbeam::channel::Sender;
use std::thread;
use std::time::{Duration, Instant};

use crate::combustion_event::CombustionEvent;

pub fn start_engine_sim_thread(event_sender: Sender<CombustionEvent>) {
    thread::spawn(move || {
        // Engine parameters
        let num_cylinders = 4;
        let firing_angles = [0.0, 180.0, 360.0, 540.0]; // degrees
        let mut current_cylinder = 0;

        let mut crank_angle = 0.0; // degrees
        let mut engine_time = 0.0; // seconds
        let mut rpm = 800.0; // initial RPM
        let mut load = 0.5;  // initial load

        // Time step
        let timestep = 1.0 / 1000.0; // 1000Hz → 1 ms per step
        let timestep_duration = Duration::from_micros(1000); // 1ms sleep

        // Valve lift profile
        let valve_steps = 50;
        let valve_lift_profile: Vec<f32> = (0..valve_steps)
            .map(|i| {
                if i < valve_steps / 2 {
                    (i as f32) / (valve_steps as f32 / 2.0)
                } else {
                    1.0 - ((i as f32 - valve_steps as f32 / 2.0) / (valve_steps as f32 / 2.0))
                }
            })
            .collect();

        // Exhaust flow and temperature curves
        let exhaust_flow_velocity = vec![10.0; valve_steps];
        let temperature_curve = vec![800.0; valve_steps];

        let mut last_instant = Instant::now();
        let valve_open_duration_degrees = 260.0; // typical cam profile

        let base_pressure = 4000000.0;
        let load_factor = 0.3 + 0.7 * load;

        loop {
            let now = Instant::now();
            let dt = now.duration_since(last_instant).as_secs_f32();
            last_instant = now;

            engine_time += dt;

            // RPM ramping → simple test behavior
            rpm += 100.0 * dt; // 100 RPM per second increase
            if rpm > 6000.0 {
                rpm = 800.0;
            }

            // Crank angle advance
            let degrees_per_sec = (rpm / 60.0) * 360.0;
            let delta_crank = degrees_per_sec * dt;
            crank_angle += delta_crank;

            let valve_open_duration = valve_open_duration_degrees / degrees_per_sec;

            let ve = if rpm < 3000.0 {
                0.9 - (rpm / 3000.0) * 0.2 // drops slightly
            } else {
                0.7 // assume efficiency drops at high RPM
            };

            let pressure_at_valve = base_pressure * load_factor * ve;

            // Check if it's time to fire current cylinder
            if crank_angle >= firing_angles[current_cylinder] {
                // Emit combustion event
                let piston_speed = 5.0; // dummy

                let event = CombustionEvent {
                    cylinder_id: current_cylinder as u32,
                    event_time: engine_time,
                    rpm,
                    crank_angle,
                    pressure_at_valve,
                    load,
                    valve_open_duration,
                    valve_lift_profile: valve_lift_profile.clone(),
                    piston_speed,
                    exhaust_flow_velocity: exhaust_flow_velocity.clone(),
                    temperature_curve: temperature_curve.clone(),
                };

                let _ = event_sender.send(event);

                // Move to next cylinder
                current_cylinder = (current_cylinder + 1) % num_cylinders;
            }

            // Wrap crank_angle at 720° (4-stroke)
            if crank_angle >= 720.0 {
                crank_angle -= 720.0;
            }

            // Sleep to maintain 1000Hz
            thread::sleep(timestep_duration);
        }
    });
}
