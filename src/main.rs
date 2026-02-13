mod colormap;
mod dejong_oscillator;
mod renderer;

use error_stack::{Report, ResultExt};
use thiserror::Error;

use crate::dejong_oscillator::DeJong;
use crate::dejong_oscillator::DeJongState;
use crate::renderer::Renderer;

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

    let mut renderer = Renderer::new();

    renderer.render(dj);

    let buffer = renderer.get_image();

    buffer
        .save("hola.png")
        .change_context(AppError::ImageSave)?;

    Ok(())
}
