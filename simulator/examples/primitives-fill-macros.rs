//! # Example: Using macros to define and style primitives
//!
//! Demonstrate usage of macros to create primitives and associated styles. The final output is the
//! same as the `primitives-fill` example.

use embedded_graphics::{
    egcircle, egrectangle, egtriangle, pixelcolor::BinaryColor, prelude::*, primitive_style,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

static CIRCLE_SIZE: i32 = 65;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(384, 128));

    egcircle!(
        top_left = (0, 0),
        diameter = CIRCLE_SIZE as u32,
        style = primitive_style!(stroke_color = BinaryColor::On, stroke_width = 1,)
    )
    .draw(&mut display)?;

    egcircle!(
        top_left = (0, 0),
        diameter = CIRCLE_SIZE as u32,
        style = primitive_style!(
            stroke_color = BinaryColor::Off,
            stroke_width = 1,
            fill_color = BinaryColor::On,
        )
    )
    .translate(Point::new(16, 16))
    .draw(&mut display)?;

    egcircle!(
        top_left = (0, 0),
        diameter = CIRCLE_SIZE as u32,
        style = primitive_style!(
            stroke_color = BinaryColor::Off,
            stroke_width = 1,
            fill_color = BinaryColor::Off
        )
    )
    .translate(Point::new(CIRCLE_SIZE / 2, CIRCLE_SIZE / 2))
    .draw(&mut display)?;

    egrectangle!(
        top_left = (0, 0),
        bottom_right = (64, 64),
        style = primitive_style!(stroke_color = BinaryColor::On, stroke_width = 1,)
    )
    .translate(Point::new(96, 0))
    .draw(&mut display)?;

    egrectangle!(
        top_left = (0, 0),
        bottom_right = (64, 64),
        style = primitive_style!(
            stroke_color = BinaryColor::Off,
            stroke_width = 1,
            fill_color = BinaryColor::On
        )
    )
    .translate(Point::new(96 + 16, 16))
    .draw(&mut display)?;

    egrectangle!(
        top_left = (0, 0),
        bottom_right = (64, 64),
        style = primitive_style!(
            stroke_color = BinaryColor::Off,
            stroke_width = 1,
            fill_color = BinaryColor::Off
        )
    )
    .translate(Point::new(96 + 32, 32))
    .draw(&mut display)?;

    egtriangle!(
        points = [(32, 0), (0, 64), (64, 64)],
        style = primitive_style!(stroke_color = BinaryColor::On, stroke_width = 1,)
    )
    .translate(Point::new(96 * 2, 0))
    .draw(&mut display)?;

    egtriangle!(
        points = [(32, 0), (0, 64), (64, 64)],
        style = primitive_style!(
            stroke_color = BinaryColor::Off,
            stroke_width = 1,
            fill_color = BinaryColor::On,
        )
    )
    .translate(Point::new(96 * 2 + 16, 16))
    .draw(&mut display)?;

    egtriangle!(
        points = [(32, 0), (0, 64), (64, 64)],
        style = primitive_style!(
            stroke_color = BinaryColor::Off,
            stroke_width = 1,
            fill_color = BinaryColor::Off,
        )
    )
    .translate(Point::new(96 * 2 + 32, 32))
    .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("Filled primitives using macros", &output_settings).show_static(&display);

    Ok(())
}
