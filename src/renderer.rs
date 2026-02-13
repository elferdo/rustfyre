use array2d::Array2D;
use color::{Oklch, OpaqueColor};
use image::RgbImage;

use crate::{
    dejong_oscillator::DeJongState, pixel_coloring::PixelColoring, screen_size::SCREEN_SIZE_4K,
};

pub struct Renderer4k {
    array: Array2D<u32>,
}

impl Renderer4k {
    pub fn new() -> Self {
        let array = Array2D::filled_with(1u32, SCREEN_SIZE_4K.rows, SCREEN_SIZE_4K.columns);

        Self { array }
    }

    pub fn render(&mut self, mut i: impl Iterator<Item = DeJongState>) {
        let xmin = -2.0;
        let xmax = 2.0;
        let ymin = -2.0;
        let ymax = 2.0;

        for _ in 1..10000000u64 {
            let state = i.next().unwrap();

            let columns = SCREEN_SIZE_4K.columns as f64;
            let rows = SCREEN_SIZE_4K.rows as f64;

            let x4k = ((state.x - xmin) * columns / (xmax - xmin)) as usize;
            let y4k = ((state.y - ymin) * rows / (ymax - ymin)) as usize;

            self.array[(y4k, x4k)] += 1;
        }
    }

    pub fn make_image(
        &self,
        background: OpaqueColor<Oklch>,
        first_color: OpaqueColor<Oklch>,
        second_color: OpaqueColor<Oklch>,
    ) -> RgbImage {
        let scaled_array: Vec<_> = scale_array(&self.array);

        let subpixels: Vec<_> = scaled_array.subpixels(background, first_color, second_color);

        RgbImage::from_vec(
            SCREEN_SIZE_4K.columns as u32,
            SCREEN_SIZE_4K.rows as u32,
            subpixels,
        )
        .unwrap()
    }
}

fn scale_array(array: &Array2D<u32>) -> Vec<f64> {
    array
        .as_row_major()
        .into_iter()
        .map(|v| (v as f64).log2())
        .collect()
}
