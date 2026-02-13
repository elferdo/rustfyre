use color::{HueDirection, Oklab, OpaqueColor};

pub struct Colormap {
    first: OpaqueColor<Oklab>,
    second: OpaqueColor<Oklab>,
}

impl Colormap {
    pub fn new(first: OpaqueColor<Oklab>, second: OpaqueColor<Oklab>) -> Self {
        Self { first, second }
    }

    pub fn apply(&self, x: f64) -> OpaqueColor<Oklab> {
        if x > 0.99 {
            OpaqueColor::<Oklab>::new([1.0, 0.0, 0.0])
        } else {
            self.first
                .lerp(self.second, x as f32, HueDirection::Shorter)
        }
    }
}
