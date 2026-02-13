#[derive(Copy, Clone, Debug, Default)]
pub struct DeJongState {
    pub x: f64,
    pub y: f64,
}

pub struct DeJong {
    state: DeJongState,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

impl DeJong {
    pub fn new(init: DeJongState, a: f64, b: f64, c: f64, d: f64) -> Self {
        Self {
            state: init,
            a,
            b,
            c,
            d,
        }
    }
}

impl Iterator for DeJong {
    type Item = DeJongState;

    fn next(&mut self) -> Option<Self::Item> {
        let new_x = (self.a * self.state.y).sin() - (self.b * self.state.x).cos();
        let new_y = (self.c * self.state.x).sin() - (self.d * self.state.y).cos();

        self.state = DeJongState { x: new_x, y: new_y };

        Some(self.state)
    }
}
