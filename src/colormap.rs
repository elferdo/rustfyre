use color::{HueDirection, Oklch, OpaqueColor};

pub struct Colormap {
    background: OpaqueColor<Oklch>,
    first: OpaqueColor<Oklch>,
    second: OpaqueColor<Oklch>,
}

impl Colormap {
    pub fn new(
        background: OpaqueColor<Oklch>,
        first: OpaqueColor<Oklch>,
        second: OpaqueColor<Oklch>,
    ) -> Self {
        Self {
            background,
            first,
            second,
        }
    }

    pub fn apply(&self, x: f64) -> OpaqueColor<Oklch> {
        if x > 0.99 {
            self.background
        } else {
            self.first
                .lerp(self.second, x as f32, HueDirection::Shorter)
        }
    }
}
