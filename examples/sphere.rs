#[macro_use]
extern crate glium;
extern crate glium_shapes;
mod common;
use glium::Surface;

fn main() {

    // Setup glium display and shared example data (program, uniforms, draw params, etc)
    let (display, data) = common::setup();

    // KEY POINT: Use a SphereBuilder to build a new sphere.
    // Use the methods on the builder object to customise the resultant
    // shape. In this case we will create a 2x3x4 sphere with its base
    // located at the origin.
    let sphere = glium_shapes::sphere::SphereBuilder::new()
        .scale(2.0, 3.0, 4.0)
        .translate(0.0, 1.5, 0.0)
        .build(&display)
        .expect("Failed to build sphere shape");

    // Loop until the user closes the display window.
    while common::process_events(&display) {

        // Begin a new frame.
        let (mut frame, uniforms) = common::begin_frame(&display);

        // KEY POINT: Draw the sphere shape by passing it as a source
        // of both vertices and indices to glium.
        frame.draw(&sphere,
                  &sphere,
                  &data.program,
                  &uniforms,
                  &data.draw_params)
            .expect("Failed to draw sphere shape");

        // Finish the frame.
        common::end_frame(frame);
    }
}
