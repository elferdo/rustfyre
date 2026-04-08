mod cli;
mod colormap;
mod dejong_oscillator;
mod pixel_coloring;
mod renderer;
mod screen_size;

use clap::Parser;
use color::Oklch;
use color::OpaqueColor;
use error_stack::{Report, ResultExt};
use thiserror::Error;

use crate::cli::Args;
use crate::dejong_oscillator::DeJong;
use crate::dejong_oscillator::DeJongState;
use crate::renderer::Renderer4k;

#[derive(Debug, Error)]
enum AppError {
    #[error("saving image")]
    ImageSave,
}

fn main() -> Result<(), Report<AppError>> {
    let args = Args::parse();

    let init_state = DeJongState::default();

    let dj = DeJong::new(init_state, args.a, args.b, args.c, args.d);

    let mut renderer = Renderer4k::new();

    renderer.render(dj, args.iterations);

    /* @todo move this to cli arguments */
    let background = OpaqueColor::<Oklch>::new([1.0, 0.0, 0.0]);
    let first_color = OpaqueColor::<Oklch>::new([0.3, 0.0, 0.0]);
    let second_color = OpaqueColor::<Oklch>::new([0.90, 0.0, 0.0]);

    let image_buffer = renderer.make_image(background, first_color, second_color);

    image_buffer
        .save(args.output_filename)
        .change_context(AppError::ImageSave)?;

    Ok(())
}
