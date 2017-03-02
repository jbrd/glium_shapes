//! A module containing the vertex structure shared across all shapes.

extern crate glium;

/// The vertex structure shared across all shapes.
#[derive(Copy,Clone,Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub texcoord: [f32; 2]
}

implement_vertex!(Vertex, position, normal, texcoord);
