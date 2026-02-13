use color::{HueDirection, Oklab, OpaqueColor};

pub struct Colormap {
    background: OpaqueColor<Oklab>,
    first: OpaqueColor<Oklab>,
    second: OpaqueColor<Oklab>,
}

impl Colormap {
    pub fn new(
        background: OpaqueColor<Oklab>,
        first: OpaqueColor<Oklab>,
        second: OpaqueColor<Oklab>,
    ) -> Self {
        Self {
            background,
            first,
            second,
        }
    }

    pub fn apply(&self, x: f64) -> OpaqueColor<Oklab> {
        if x > 0.99 {
            self.background
        } else {
            self.first
                .lerp(self.second, x as f32, HueDirection::Shorter)
        }
    }
}
