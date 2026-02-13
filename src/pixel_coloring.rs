use color::{Oklch, OpaqueColor};

use crate::colormap::Colormap;

pub trait PixelColoring {
    fn subpixels(
        self,
        background: OpaqueColor<Oklch>,
        first_color: OpaqueColor<Oklch>,
        second_color: OpaqueColor<Oklch>,
    ) -> Vec<u8>;
}

impl PixelColoring for &[f64] {
    fn subpixels(
        self,
        background: OpaqueColor<Oklch>,
        first_color: OpaqueColor<Oklch>,
        second_color: OpaqueColor<Oklch>,
    ) -> Vec<u8> {
        let max_value = max(self);

        let colormap = Colormap::new(background, first_color, second_color);

        self.iter()
            .flat_map(|v| {
                let value = 1.0 - contrast(v / max_value);

                let color = colormap.apply(value).to_rgba8();

                [color.r, color.g, color.b]
            })
            .collect()
    }
}

fn max(v: &[f64]) -> f64 {
    v.iter().fold(f64::MIN, |b, x| b.max(*x))
}

fn contrast(u: f64) -> f64 {
    let x = u * 2.0 - 1.0;

    (1.0 + (x - x.powi(3) / 3.0) * 1.5) / 2.0
}
