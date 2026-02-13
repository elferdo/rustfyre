use array2d::Array2D;
use color::{Oklab, OpaqueColor};
use image::RgbImage;

use crate::{colormap::Colormap, dejong_oscillator::DeJongState};

pub struct Renderer {
    array: Array2D<u32>,
}

impl Renderer {
    pub fn new() -> Self {
        let array = Array2D::filled_with(1u32, 2160, 3840);

        Self { array }
    }

    pub fn render(&mut self, mut i: impl Iterator<Item = DeJongState>) {
        let xmin = -2.0;
        let xmax = 2.0;
        let ymin = -2.0;
        let ymax = 2.0;

        for _ in 1..10000000u64 {
            let state = i.next().unwrap();

            let x4k = ((state.x - xmin) * 3839.0 / (xmax - xmin)) as usize;
            let y4k = ((state.y - ymin) * 2159.0 / (ymax - ymin)) as usize;

            self.array[(y4k, x4k)] += 1;
        }
    }

    pub fn make_image(
        &self,
        background: OpaqueColor<Oklab>,
        first_color: OpaqueColor<Oklab>,
        second_color: OpaqueColor<Oklab>,
    ) -> RgbImage {
        let scaled_array: Vec<_> = self
            .array
            .as_row_major()
            .into_iter()
            .map(|v| (v as f64).log2())
            .collect();

        let max_value = scaled_array.iter().fold(0.0f64, |b, x| b.max(*x));

        let colormap = Colormap::new(background, first_color, second_color);

        let subpixels: Vec<_> = scaled_array
            .into_iter()
            .flat_map(|v| {
                let sp = 1.0 - contrast(v / max_value);

                let color = colormap.apply(sp).to_rgba8();

                [color.r, color.g, color.b]
            })
            .collect();

        RgbImage::from_vec(3840, 2160, subpixels).unwrap()
    }
}

fn contrast(u: f64) -> f64 {
    let x = u * 2.0 - 1.0;

    (1.0 + (x - x.powi(3) / 3.0) * 1.5) / 2.0
}
