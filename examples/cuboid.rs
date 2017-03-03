#[macro_use]
extern crate glium;
extern crate glium_shapes;
mod common;
use glium::Surface;

fn main() {

    // Setup glium display and shared example data (program, uniforms, draw params, etc)
    let (display, data) = common::setup();

    // KEY POINT: Use a CuboidBuilder to build a new cuboid.
    // Use the methods on the builder object to customise the resultant
    // shape. In this case we will create a 2x3x4 cuboid with its base
    // located at the origin.
    let cuboid = glium_shapes::cuboid::CuboidBuilder::new()
        .scale(2.0, 3.0, 4.0)
        .translate(0.0, 1.5, 0.0)
        .build(&display)
        .expect("Failed to build cuboid shape");

    // Loop until the user closes the display window.
    while common::process_events(&display) {

        // Begin a new frame.
        let (mut frame, uniforms) = common::begin_frame(&display);

        // KEY POINT: Draw the cuboid shape by passing it as a source
        // of both vertices and indices to glium.
        frame.draw(&cuboid,
                  &cuboid,
                  &data.program,
                  &uniforms,
                  &data.draw_params)
            .expect("Failed to draw cuboid shape");

        // Finish the frame.
        common::end_frame(frame);
    }
}
