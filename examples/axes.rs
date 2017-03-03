#[macro_use]
extern crate glium;
extern crate glium_shapes;
mod common;
use glium::Surface;

fn main() {

    // Setup glium display and shared example data (program, uniforms, draw params, etc)
    let (display, data) = common::setup();

    // KEY POINT: Use an AxesBuilder to build a new Axes object.
    // Use the methods on the builder object to customise the resultant
    // shape. In this case we will create an axes locator with its origin
    // at (0.0, 4.0, 0.0)
    let axes = glium_shapes::axes::AxesBuilder::new()
        .translate(0.0, 4.0, 0.0)
        .build(&display)
        .expect("Failed to build axes shape");

    // Loop until the user closes the display window.
    while common::process_events(&display) {

        // Begin a new frame.
        let (mut frame, uniforms) = common::begin_frame(&display);

        // KEY POINT: Draw the axes shape by passing it as a source
        // of both vertices and indices to glium.
        frame.draw(&axes, &axes, &data.program, &uniforms, &data.draw_params)
            .expect("Failed to draw axes shape");

        // Finish the frame.
        common::end_frame(frame);
    }
}
