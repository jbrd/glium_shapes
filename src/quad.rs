//! A module for constructing polygonal quad shapes.

extern crate cgmath;
extern crate glium;

use self::cgmath::*;
use errors::ShapeCreationError;
use vertex::Vertex;

/// A polygonal quad.
///
/// This object is constructed using a `QuadBuilder` object.
pub struct Quad {
    vertices: glium::vertex::VertexBufferAny,
}

/// Allows a `Quad` object to be passed as a source of vertices.
impl<'a> From<&'a Quad> for glium::vertex::VerticesSource<'a> {
    fn from(quad: &'a Quad) -> glium::vertex::VerticesSource<'a> {
        (&quad.vertices).into()
    }
}

/// Allows a `Quad` object to be passed as a source of indices.
impl<'a> Into<glium::index::IndicesSource<'a>> for &'a Quad {
    fn into(self) -> glium::index::IndicesSource<'a> {
        return glium::index::IndicesSource::NoIndices {
            primitives: glium::index::PrimitiveType::TriangleStrip,
        };
    }
}

/// Responsible for building and returning a `Quad` object.
///
/// By default, the resultant polygon will be a regular quad of length 2
/// on each side, with its centre at the origin, and aligned to face the
/// negative Z-axis. The default position, size, and alignment can be
/// overridden using the transformation methods on this object. These
/// defaults are chosen such that the default quad can be used directly
/// as geometry for screen-aligned effects.
///
/// The resultant geometry is constructed to suit OpenGL defaults - assuming
/// a right-handed coordinate system, front-facing polygons are defined in
/// counter-clock-wise order. Vertex normals point in the direction of their
/// respective face (such that the shape appears faceted when lit). Vertex
/// texture coordinates define a planar-projection on the face.
pub struct QuadBuilder {
    matrix: cgmath::Matrix4<f32>,
}

impl Default for QuadBuilder {
    fn default() -> QuadBuilder {
        QuadBuilder {
            matrix: cgmath::Matrix4::<f32>::identity(),
        }
    }
}

impl QuadBuilder {
    /// Create a new `QuadBuilder` object.
    pub fn new() -> QuadBuilder {
        Default::default()
    }

    /// Apply a scaling transformation to the shape.
    ///
    /// The `scale`, `translate`, and `rotate` functions accumulate, and are
    /// not commutative. The transformation functions are intended to provide
    /// flexibility in model-space. For per-instance world-space transformations,
    /// one should prefer to share as few shapes as possible across multiple
    /// instances, and instead rely on uniform constants in the shader and/or
    /// instanced drawing.
    pub fn scale(mut self, x: f32, y: f32, z: f32) -> Self {
        self.matrix = cgmath::Matrix4::from_nonuniform_scale(x, y, z) * self.matrix;
        return self;
    }

    /// Apply a translation transformation to the shape.
    ///
    /// The `scale`, `translate`, and `rotate` functions accumulate, and are
    /// not commutative. The transformation functions are intended to provide
    /// flexibility in model-space. For per-instance world-space transformations,
    /// one should prefer to share as few shapes as possible across multiple
    /// instances, and instead rely on uniform constants in the shader and/or
    /// instanced drawing.
    pub fn translate(mut self, x: f32, y: f32, z: f32) -> Self {
        self.matrix = cgmath::Matrix4::from_translation([x, y, z].into()) * self.matrix;
        return self;
    }

    /// Apply a rotation transformation to the shape about the x-axis.
    ///
    /// The `scale`, `translate`, and `rotate` functions accumulate, and are
    /// not commutative. The transformation functions are intended to provide
    /// flexibility in model-space. For per-instance world-space transformations,
    /// one should prefer to share as few shapes as possible across multiple
    /// instances, and instead rely on uniform constants in the shader and/or
    /// instanced drawing.
    pub fn rotate_x(mut self, radians: f32) -> Self {
        self.matrix = cgmath::Matrix4::<f32>::from(cgmath::Matrix3::<f32>::from_angle_x(
            cgmath::Rad::<f32>(radians),
        )) * self.matrix;
        return self;
    }

    /// Apply a rotation transformation to the shape about the y-axis.
    ///
    /// The `scale`, `translate`, and `rotate` functions accumulate, and are
    /// not commutative. The transformation functions are intended to provide
    /// flexibility in model-space. For per-instance world-space transformations,
    /// one should prefer to share as few shapes as possible across multiple
    /// instances, and instead rely on uniform constants in the shader and/or
    /// instanced drawing.
    pub fn rotate_y(mut self, radians: f32) -> Self {
        self.matrix = cgmath::Matrix4::<f32>::from(cgmath::Matrix3::<f32>::from_angle_y(
            cgmath::Rad::<f32>(radians),
        )) * self.matrix;
        return self;
    }

    /// Apply a rotation transformation to the shape about the z-axis.
    ///
    /// The `scale`, `translate`, and `rotate` functions accumulate, and are
    /// not commutative. The transformation functions are intended to provide
    /// flexibility in model-space. For per-instance world-space transformations,
    /// one should prefer to share as few shapes as possible across multiple
    /// instances, and instead rely on uniform constants in the shader and/or
    /// instanced drawing.
    pub fn rotate_z(mut self, radians: f32) -> Self {
        self.matrix = cgmath::Matrix4::<f32>::from(cgmath::Matrix3::<f32>::from_angle_z(
            cgmath::Rad::<f32>(radians),
        )) * self.matrix;
        return self;
    }

    /// Build a new `Quad` object.
    pub fn build<F>(self, display: &F) -> Result<Quad, ShapeCreationError>
    where
        F: glium::backend::Facade,
    {
        let vertices =
            glium::vertex::VertexBuffer::<Vertex>::new(display, &self.build_vertices()?)?;

        Ok(Quad {
            vertices: glium::vertex::VertexBufferAny::from(vertices),
        })
    }

    /// Build the Quad vertices and return them in a vector.
    ///
    /// Useful if you wish to do other things with the vertices besides constructing
    /// a `Quad` object (e.g. unit testing, further processing, etc).
    pub fn build_vertices(&self) -> Result<Vec<Vertex>, ShapeCreationError> {
        // Compute the normal transformation matrix.
        let normal_matrix = Matrix3::<f32>::from_cols(
            self.matrix.x.truncate(),
            self.matrix.y.truncate(),
            self.matrix.z.truncate(),
        )
        .invert()
        .unwrap_or(Matrix3::<f32>::identity())
        .transpose();

        // Build the vertices.
        let verts_per_quad = 4;
        let mut vertices = Vec::<Vertex>::with_capacity(verts_per_quad);
        for vert in 0..verts_per_quad {
            let (u, v) = ((vert / 2) as f32, (vert % 2) as f32);
            let position = Vector4::<f32>::new((u * 2.0) - 1.0, (v * 2.0) - 1.0, 0.0, 1.0);
            let normal = Vector3::<f32>::new(0.0, 0.0, -1.0);
            vertices.push(Vertex {
                position: Point3::<f32>::from_homogeneous(self.matrix * position).into(),
                normal: (normal_matrix * normal).normalize().into(),
                texcoord: [u, v],
            });
        }
        return Ok(vertices);
    }
}

#[test]
pub fn ensure_default_quad_has_edge_lengths_of_two() {
    use std::f32;
    let vertices = QuadBuilder::new()
        .build_vertices()
        .expect("Failed to build vertices");
    let mut min = Vector3::<f32>::new(f32::MAX, f32::MAX, f32::MAX);
    let mut max = -min;
    for ref vertex in vertices {
        let pos = Vector3::<f32>::from(vertex.position);
        min.x = f32::min(min.x, pos.x);
        min.y = f32::min(min.y, pos.y);
        min.z = f32::min(min.z, pos.z);
        max.x = f32::max(max.x, pos.x);
        max.y = f32::max(max.y, pos.y);
        max.z = f32::max(max.z, pos.z);
    }
    assert_eq!(min, Vector3::new(-1.0, -1.0, 0.0));
    assert_eq!(max, Vector3::new(1.0, 1.0, 0.0));
}

#[test]
pub fn ensure_default_quad_has_centroid_at_origin() {
    let vertices = QuadBuilder::new()
        .build_vertices()
        .expect("Failed to build vertices");
    let mut sum = Vector3::<f32>::zero();
    for ref vertex in vertices {
        sum = sum + Vector3::<f32>::from(vertex.position);
    }
    assert_eq!(sum, Vector3::<f32>::zero());
}

#[test]
pub fn ensure_default_quad_is_planar() {
    let vertices = QuadBuilder::new()
        .build_vertices()
        .expect("Failed to build vertices");
    let tri0 = [
        Vector3::<f32>::from(vertices[0].position),
        Vector3::<f32>::from(vertices[1].position),
        Vector3::<f32>::from(vertices[2].position),
    ];

    let tri1 = [
        Vector3::<f32>::from(vertices[2].position),
        Vector3::<f32>::from(vertices[1].position),
        Vector3::<f32>::from(vertices[3].position),
    ];

    let n0 = (tri0[1] - tri0[0]).cross(tri0[2] - tri0[0]).normalize();
    let n1 = (tri1[1] - tri1[0]).cross(tri1[2] - tri1[0]).normalize();
    assert_ulps_eq!(n0, n1, epsilon = 0.0001);
}

#[test]
pub fn ensure_default_quad_has_ccw_triangles() {
    let vertices = QuadBuilder::new()
        .build_vertices()
        .expect("Failed to build vertices");
    let tris = [[0, 1, 2], [2, 1, 3]];
    for tri in tris.iter() {
        let v0 = Vector3::<f32>::from(vertices[tri[0]].position);
        let v1 = Vector3::<f32>::from(vertices[tri[1]].position);
        let v2 = Vector3::<f32>::from(vertices[tri[2]].position);
        let eyepos = v0 + Vector3::<f32>::from(vertices[tri[0]].normal);
        let e0 = v1 - v0;
        let e1 = v2 - v0;
        let n = e0.cross(e1);
        assert!(n.dot(v0 - eyepos) <= 0.0);
        assert!(n.dot(v1 - eyepos) <= 0.0);
        assert!(n.dot(v2 - eyepos) <= 0.0);
    }
}

#[test]
pub fn ensure_default_quad_has_face_aligned_normals() {
    let vertices = QuadBuilder::new()
        .build_vertices()
        .expect("Failed to build vertices");
    let tri0 = [
        Vector3::<f32>::from(vertices[0].position),
        Vector3::<f32>::from(vertices[1].position),
        Vector3::<f32>::from(vertices[2].position),
    ];
    let fnormal = (tri0[1] - tri0[0]).cross(tri0[2] - tri0[0]).normalize();
    for vertex in vertices.iter() {
        let vnormal = Vector3::<f32>::from(vertex.normal);
        assert_eq!(vnormal, fnormal);
    }
}

#[test]
pub fn ensure_quad_uvs_are_in_correct_range() {
    use std::f32;
    let vertices = QuadBuilder::new()
        .build_vertices()
        .expect("Failed to build vertices");
    let mut min = Vector2::<f32>::new(f32::MAX, f32::MAX);
    let mut max = -min;
    for ref vertex in vertices {
        min.x = f32::min(min.x, vertex.texcoord[0]);
        min.y = f32::min(min.y, vertex.texcoord[1]);
        max.x = f32::max(max.x, vertex.texcoord[0]);
        max.y = f32::max(max.y, vertex.texcoord[1]);
    }
    assert!(min == Vector2::<f32>::zero());
    assert!(max == Vector2::<f32>::from_value(1.0));
}
