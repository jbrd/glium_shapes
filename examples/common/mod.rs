//! A module for common example logic, in an attempt to keep the example
//! source files as concise as possible.
extern crate cgmath;
extern crate glium;

use self::cgmath::*;
use self::glium::uniforms::*;
use self::glium::Display;
use self::glium::*;

/// Shared data used in the examples.
pub struct ExampleData<'a> {
    /// The shader program used to render the shapes.
    pub program: glium::Program,

    /// The draw parameter state used to render the shapes.
    pub draw_params: glium::DrawParameters<'a>,
}

/// A type to represent the frame-varying uniforms in the examples. At
/// the moment, this just holds the camera (view * projection) matrix.
pub type FrameUniforms<'a> = UniformsStorage<'a, [[f32; 4]; 4], EmptyUniforms>;

/// Setup the glium display and example data and return them both in a tuple.
pub fn setup<'a>() -> (glium::glutin::event_loop::EventLoop<()>, Display, ExampleData<'a>) {
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(600.0, 600.0))
        .with_title("Example Viewer");

    let cb = glium::glutin::ContextBuilder::new().with_depth_buffer(16);

    let ev = glium::glutin::event_loop::EventLoop::new();
    let display = glium::Display::new(wb, cb, &ev).expect("Failed to build glium display");

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
                        gl_Position = matrix * vec4(position, 1.0);
                        v_normal = normal;
                    }
                ",
                fragment: "
                    #version 140

                    in vec3 v_normal;
                    in vec2 v_texcoord;

                    out vec4 fragColor;

                    void main() {
                        fragColor = vec4((v_normal + 1) / 2, 1.0);
                    }
                ",
            }
        )
        .expect("Failed to compile shader program"),
        draw_params: glium::DrawParameters {
            backface_culling: BackfaceCullingMode::CullClockwise,
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        },
    };
    (ev, display, data)
}

/// Process any pending events in the glium display. Return true if the
/// display is still open, or false if the user has closed the display.
pub fn process_events(event: &glium::glutin::event::Event<()>, control_flow: &mut glium::glutin::event_loop::ControlFlow) -> bool {
    *control_flow = glium::glutin::event_loop::ControlFlow::Poll;
    if let glium::glutin::event::Event::WindowEvent { ref event, .. } = event {
        if let glium::glutin::event::WindowEvent::CloseRequested = event {
            *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
            return false;
        }
    }
    return true;
}

/// Called before rendering a frame. Returns the glium frame and the
/// uniforms that should be used for this frame.
pub fn begin_frame<'a>(display: &Display) -> (glium::Frame, FrameUniforms<'a>) {
    let mut frame = display.draw();
    frame.clear_color(0.1, 0.1, 0.1, 1.0);
    frame.clear_depth(1.0);
    let uniforms = build_frame_uniforms(&frame);
    (frame, uniforms)
}

/// Called after rendering a frame. Swaps the display buffers.
pub fn end_frame(frame: glium::Frame) {
    frame.finish().expect("Failed to draw frame");
}

/// Build the frame-varying uniforms. At the moment, this just
/// consists of the camera (view * projection) matrix.
fn build_frame_uniforms<'a>(frame: &glium::Frame) -> FrameUniforms<'a> {
    let (width, height) = frame.get_dimensions();

    let projection = PerspectiveFov::<f32> {
        fovy: Rad::<f32>::from(Deg::<f32>(90.0)),
        aspect: width as f32 / height as f32,
        near: 1.0,
        far: 1000.0,
    };

    let view = Matrix4::<f32>::from_translation(Vector3::<f32>::new(0.0, 0.0, -10.0));

    uniform! {
        matrix: (Matrix4::<f32>::from(projection) * view).into()
    }
}
