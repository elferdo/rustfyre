use array2d::Array2D;
use error_stack::{Report, ResultExt};
use image::GrayImage;
use thiserror::Error;

#[derive(Copy, Clone, Debug, Default)]
struct DeJongState {
    x: f64,
    y: f64,
}

struct DeJong {
    state: DeJongState,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
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

#[derive(Debug, Error)]
enum AppError {
    #[error("saving image")]
    ImageSave,
}

fn contrast(u: f64) -> f64 {
    let x = u * 2.0 - 1.0;
    (1.0 + (x - x.powi(3) / 3.0) * 1.5) / 2.0
}

fn main() -> Result<(), Report<AppError>> {
    let init_state = DeJongState::default();

    let a = -2.7;
    let b = -0.9;
    let c = -0.86;
    let d = -2.2;

    let mut dj = DeJong {
        state: init_state,
        a,
        b,
        c,
        d,
    };

    let mut array = Array2D::filled_with(1u32, 2160, 3840);

    let xmin = -2.0;
    let xmax = 2.0;
    let ymin = -2.0;
    let ymax = 2.0;

    for _ in 1..10000000000u64 {
        let state = dj.next().unwrap();

        let x4k = ((state.x - xmin) * 3839.0 / (xmax - xmin)) as usize;
        let y4k = ((state.y - ymin) * 2159.0 / (ymax - ymin)) as usize;

        array[(y4k, x4k)] = array[(y4k, x4k)] + 1;
    }

    let v: Vec<_> = array
        .as_row_major()
        .into_iter()
        .map(|v| (v as f64).log2())
        .collect();

    let max_v = v.iter().fold(0.0f64, |b, x| b.max(*x));

    let v2: Vec<_> = v
        .into_iter()
        .map(|v| (255.0 - contrast(v / max_v) * 255.0) as u8)
        .collect();

    let color = OpaqueColor::<OkLab>::new();

    let buffer = GrayImage::from_vec(3840, 2160, v2).unwrap();

    buffer
        .save("hola.png")
        .change_context(AppError::ImageSave)?;

    Ok(())
}
