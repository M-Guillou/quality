use rand::Rng;

#[derive(Debug, Clone)]
pub struct CombustionEvent {
    pub cylinder_id: u32,
    pub event_time: f32, // absolute time
    pub rpm: f32,
    pub crank_angle: f32, // at event start

    pub pressure_at_valve: f32, // Pascals
    pub load: f32,              // 0..1

    pub valve_open_duration: f32, // seconds
    pub valve_lift_profile: Vec<f32>, // normalized 0..1 (length = N steps)

    pub piston_speed: f32, // m/s
    pub exhaust_flow_velocity: Vec<f32>, // m/s (varies during valve open)
    pub temperature_curve: Vec<f32>, // Kelvin (varies during valve open)
}


pub fn generate_pressure_pulse(
    sample_rate: f32,
    pressure_at_valve: f32,
    load: f32,
    rpm: f32,
    valve_open_duration: f32,
    valve_lift_profile: &[f32],
    piston_speed: f32,
    exhaust_flow_velocity: &[f32],
    temperature_curve: &[f32],
) -> Vec<f32> {
    let duration_sec = valve_open_duration * 2.0;
    let num_samples = (duration_sec * sample_rate) as usize;

    let mut pulse = Vec::with_capacity(num_samples);

    let C_d = 0.7;
    let exhaust_pressure = 101325.0;
    let delta_p = (pressure_at_valve - exhaust_pressure).max(0.0);
    let rho = 0.9;

    let D_valve = 0.03; // meters
    let L_max = 0.008; // meters

    for n in 0..num_samples {
        let t = n as f32 / sample_rate;

        let idx = ((t / valve_open_duration) * valve_lift_profile.len() as f32) as usize;
        let lift_factor = valve_lift_profile.get(idx).cloned().unwrap_or(0.0);
        let lift = lift_factor * L_max;
        let A_valve = std::f32::consts::PI * D_valve * lift;
        let m_dot = if delta_p > 0.0 {
            C_d * A_valve * (2.0 * delta_p * rho).sqrt()
        } else {
            0.0
        };

        let pulse_value = m_dot;
        pulse.push(pulse_value);
    }

    pulse
}

