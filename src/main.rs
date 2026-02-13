mod colormap;
mod dejong_oscillator;
mod renderer;
mod screen_size;

use color::Oklab;
use color::OpaqueColor;
use error_stack::{Report, ResultExt};
use thiserror::Error;

use crate::dejong_oscillator::DeJong;
use crate::dejong_oscillator::DeJongState;
use crate::renderer::Renderer4k;

#[derive(Debug, Error)]
enum AppError {
    #[error("saving image")]
    ImageSave,
}

fn main() -> Result<(), Report<AppError>> {
    let init_state = DeJongState::default();

    let a = -2.7;
    let b = -0.9;
    let c = -0.86;
    let d = -2.2;

    let dj = DeJong::new(init_state, a, b, c, d);

    let mut renderer = Renderer4k::new();

    renderer.render(dj);

    let background = OpaqueColor::<Oklab>::new([1.0, 0.0, 0.0]);
    let first_color = OpaqueColor::<Oklab>::new([0.3, 0.0, -0.5]);
    let second_color = OpaqueColor::<Oklab>::new([0.99, -0.35, -0.3]);

    let image_buffer = renderer.make_image(background, first_color, second_color);

    image_buffer
        .save("hola.png")
        .change_context(AppError::ImageSave)?;

    Ok(())
}
