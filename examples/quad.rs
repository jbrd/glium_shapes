extern crate cgmath;
extern crate glium;
extern crate glium_shapes;
mod common;
use glium::Surface;

fn main() {

    // Setup glium display and shared example data (program, uniforms, draw params, etc)
    let (display, data) = common::setup();

    // KEY POINT: Use an QuadBuilder to build a new Quad object.
    // Use the methods on the builder object to customise the resultant
    // shape. In this case we will create a default quad at the origin.
    //
    // NOTE: The quad is set up to face the negative Z-axis by default,
    // but our example camera also looks down the negative Z-axis, and
    // so we rotate the quad 180 degrees in Y to face the camera such
    // that it doesn't get backface culled.
    let quad = glium_shapes::quad::QuadBuilder::new()
        .rotate_y(std::f32::consts::PI)
        .build(&display)
        .expect("Failed to build quad shape");

    // Loop until the user closes the display window.
    while common::process_events(&display) {

        // Begin a new frame.
        let (mut frame, uniforms) = common::begin_frame(&display);

        // KEY POINT: Draw the quad shape by passing it as a source
        // of both vertices and indices to glium.
        frame.draw(&quad, &quad, &data.program, &uniforms, &data.draw_params)
            .expect("Failed to draw quad shape");

        // Finish the frame.
        common::end_frame(frame);
    }
}
