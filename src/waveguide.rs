pub struct WaveguideSection {
    pub fwd: Vec<f32>,
    pub bwd: Vec<f32>,
    pub len: usize,
    pub pos: usize,

    pub reflection_left: f32,
    pub reflection_right: f32,
}

impl WaveguideSection {
    pub fn new(len: usize, reflection_left: f32, reflection_right: f32) -> Self {
        WaveguideSection {
            fwd: vec![0.0; len],
            bwd: vec![0.0; len],
            len,
            pos: 0,
            reflection_left,
            reflection_right,
        }
    }

    pub fn step(&mut self, input_left: f32, input_right: f32) -> f32 {
        let pos = self.pos;

        self.fwd[pos] = input_left;
        self.bwd[(pos + self.len - 1) % self.len] = input_right;

        let out_left = self.bwd[pos];
        let out_right = self.fwd[(pos + self.len - 1) % self.len];

        self.bwd[pos] = self.reflection_right * out_right.tanh();
        self.fwd[(pos + self.len - 1) % self.len] = self.reflection_left * out_left.tanh();

        let pressure_out = out_right + self.bwd[(pos + self.len - 1) % self.len];

        self.pos = (self.pos + 1) % self.len;

        pressure_out
    }
}

pub struct WaveguideChain {
    pub sections: Vec<WaveguideSection>,
}

impl WaveguideChain {
    pub fn step(&mut self, input_left: f32) -> f32 {
        let mut signal = input_left;
        for section in self.sections.iter_mut() {
            signal = section.step(signal, 0.0); // no right-going input
        }
        signal
    }
}