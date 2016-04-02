//! A module for common example logic, in an attempt to keep the example
//! source files as concise as possible.
extern crate cgmath;
extern crate glium;

use self::cgmath::*;
use self::glium::*;
use self::glium::uniforms::*;
use self::glium::backend::glutin_backend::GlutinFacade;

/// Shared data used in the examples.
pub struct ExampleData<'a> {

    /// The shader program used to render the shapes.
    pub program: glium::Program,

    /// The draw parameter state used to render the shapes.
    pub draw_params: glium::DrawParameters<'a>
}

/// A type to represent the frame-varying uniforms in the examples. At
/// the moment, this just holds the camera (view * projection) matrix.
pub type FrameUniforms<'a> = UniformsStorage<'a, [[f32; 4]; 4], EmptyUniforms>;

/// Setup the glium display and example data and return them both in a tuple.
pub fn setup<'a>() -> (GlutinFacade, ExampleData<'a>) {

    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(600, 600)
        .with_title("Example Viewer".into())
        .build_glium()
        .expect("Failed to build glium display");

    let data = ExampleData {
        program: program!(&display,
            140 => {
                vertex: "
                    #version 140

                    in vec3 position;
                    in vec3 normal;
                    in vec2 texcoord;

                    out vec3 v_normal;
                    out vec2 v_texcoord;

                    uniform mat4 matrix;

                    void main() {
                        gl_Position = matrix * vec4( position, 1.0 );
                        v_normal = normal;
                    }
                ",
                fragment: "
                    #version 140

                    in vec3 v_normal;
                    in vec2 v_texcoord;

                    out vec4 fragColor;

                    void main() {
                        fragColor = vec4( ( v_normal + 1 ) / 2, 1.0 );
                    }
                ",
            }
        ).expect("Failed to compile shader program"),
        draw_params: glium::DrawParameters {
            backface_culling: BackfaceCullingMode::CullClockwise,
            .. Default::default()
        }
    };
    return (display, data);
}

/// Process any pending events in the glium display. Return true if the
/// display is still open, or false if the user has closed the display.
pub fn process_events(display: &GlutinFacade) -> bool {
    for event in display.poll_events() {
        match event {
            glium::glutin::Event::Closed => return false,
            _ => continue
        }
    }
    return true;
}

/// Called before rendering a frame. Returns the glium frame and the
/// uniforms that should be used for this frame.
pub fn begin_frame<'a>(display: &GlutinFacade) -> (glium::Frame, FrameUniforms<'a>) {
    let mut frame = display.draw();
    frame.clear_color(0.1, 0.1, 0.1, 1.0);
    let uniforms = build_frame_uniforms(&frame);
    return (frame, uniforms);
}

/// Called after rendering a frame. Swaps the display buffers.
pub fn end_frame(frame: glium::Frame) {
    frame.finish().expect("Failed to draw frame");
}

/// Build the frame-varying uniforms. At the moment, this just
/// consists of the camera (view * projection) matrix.
fn build_frame_uniforms<'a>(frame: &glium::Frame) -> FrameUniforms<'a> {
    let (width, height) = frame.get_dimensions();

    let projection = cgmath::PerspectiveFov::<f32> {
        fovy: cgmath::Rad::<f32>::from( cgmath::Deg::<f32>::new( 90.0 ) ),
        aspect: width as f32 / height as f32,
        near: 1.0,
        far: 1000.0
    };

    let view = cgmath::Matrix4::<f32>::from_translation(
        cgmath::Vector3::<f32>::new(0.0, 0.0, -10.0)
    );

    let uniforms = uniform! {
        matrix: (cgmath::Matrix4::<f32>::from(projection) * view).into()
    };

    return uniforms;
}
