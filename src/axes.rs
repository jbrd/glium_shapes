//! A module for constructing axes locator shapes.

extern crate cgmath;
extern crate glium;

use cgmath::*;
use errors::ShapeCreationError;
use vertex::Vertex;

/// A set of orthogonal `Axes` lines.
///
/// This object is constructed using a `AxesBuilder` object.
pub struct Axes {
    vertices: glium::vertex::VertexBufferAny
}

/// Allows an `Axes` object to be passed as a source of vertices.
impl<'a> glium::vertex::IntoVerticesSource<'a> for &'a Axes {
    fn into_vertices_source(self) -> glium::vertex::VerticesSource<'a> {
        return self.vertices.into_vertices_source();
    }
}

/// Allows an `Axes` object to be passed as a source of indices.
impl<'a> Into<glium::index::IndicesSource<'a>> for &'a Axes {
    fn into(self) -> glium::index::IndicesSource<'a> {
        return glium::index::IndicesSource::NoIndices{
            primitives: glium::index::PrimitiveType::LinesList
        };
    }
}

/// Responsible for building and returning an `Axes` object.
///
/// By default, each orthogonal axis line is 1 unit in length, with the
/// centre point located at the origin. This can be overriden using the
/// transformation methods on this object.
///
/// The resultant geometry is constructed to suit OpenGL defaults - assuming
/// a right-handed coordinate system. Vertex normals define the normalised
/// direction of their respective axis line. Vertex texture coordinates encode
/// end point in the U coordinate (a value of 0 or 1), and the axis number in
/// the V coordinate (a value of 0, 1, or 2).
pub struct AxesBuilder {
    matrix: cgmath::Matrix4<f32>
}

impl Default for AxesBuilder {
    fn default() -> AxesBuilder {
        AxesBuilder {
            matrix: cgmath::Matrix4::<f32>::identity()
        }
    }
}

impl AxesBuilder {

    /// Create a new `AxesBuilder` object.
    pub fn new() -> AxesBuilder {
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
    pub fn translate(mut self, x: f32, y:f32, z: f32) -> Self {
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
        self.matrix = cgmath::Matrix4::<f32>::from(
            cgmath::Matrix3::<f32>::from_angle_x(
                cgmath::Rad::<f32>::new(radians)
            )
        ) * self.matrix;
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
        self.matrix = cgmath::Matrix4::<f32>::from(
            cgmath::Matrix3::<f32>::from_angle_y(
                cgmath::Rad::<f32>::new(radians)
            )
        ) * self.matrix;
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
        self.matrix = cgmath::Matrix4::<f32>::from(
            cgmath::Matrix3::<f32>::from_angle_z(
                cgmath::Rad::<f32>::new(radians)
            )
        ) * self.matrix;
        return self;
    }

    /// Build a new `Axes` object.
    pub fn build<F>(self, display: &F) -> Result<Axes, ShapeCreationError>
    where F:glium::backend::Facade {
        let vertices = try!(glium::vertex::VertexBuffer::<Vertex>::new(
            display, &try!(self.build_vertices())
        ));

        Ok(Axes {
            vertices: glium::vertex::VertexBufferAny::from(vertices),
        })
    }

    /// Build the axes vertices and return them in a vector.
    ///
    /// Useful if you wish to do other things with the vertices besides constructing
    /// a `Axes` object (e.g. unit testing, further processing, etc).
    pub fn build_vertices(&self) -> Result<Vec<Vertex>, ShapeCreationError> {

        // Compute the normal transformation matrix.
        let normal_matrix = Matrix3::<f32>::from_cols(
            self.matrix.x.truncate(),
            self.matrix.y.truncate(),
            self.matrix.z.truncate()
        ).invert().unwrap_or(Matrix3::<f32>::identity()).transpose();

        // Build the vertices.
        let num_axes = 3;
        let verts_per_axis = 2;
        let mut vertices = Vec::<Vertex>::with_capacity(
            verts_per_axis * num_axes
        );

        for axis in 0..num_axes {
            for vert in 0..verts_per_axis {
                let mut normal = Vector3::<f32>::new(0.0, 0.0, 0.0);
                normal[axis] = 1.0;
                let position = (normal * (vert as f32)).extend(1.0);
                vertices.push(Vertex{
                    position: Point3::<f32>::from_homogeneous(self.matrix * position).into(),
                    normal: (normal_matrix * normal).normalize().into(),
                    texcoord: [
                        vert as f32,
                        axis as f32,
                    ],
                });
            }
        }

        return Ok(vertices);
    }
}

#[test]
pub fn ensure_default_axes_has_unit_dimensions() {
    let vertices = AxesBuilder::new()
                   .build_vertices()
                   .expect("Failed to build vertices");
    for ref vertex in vertices {
        let pos = Vector3::<f32>::from(vertex.position);
        assert!(pos.x >= 0.0);
        assert!(pos.x <= 1.0);
        assert!(pos.y >= 0.0);
        assert!(pos.y <= 1.0);
        assert!(pos.z >= 0.0);
        assert!(pos.z <= 1.0);
    }
}

#[test]
pub fn ensure_default_axes_are_placed_at_origin() {
    let vertices = AxesBuilder::new()
                   .build_vertices()
                   .expect("Failed to build vertices");
    for chunk in vertices.chunks(2) {
        assert_eq!(Vector3::<f32>::from(chunk[0].position), Vector3::<f32>::zero());
    }
}

#[test]
pub fn ensure_default_axes_are_orthogonal() {
    let vertices = AxesBuilder::new()
                   .build_vertices()
                   .expect("Failed to build vertices");
    let chunks = vertices.chunks(2);
    let axes: Vec<Vector3<f32>> = chunks.map(
        |chunk| Vector3::<f32>::from(chunk[1].position) - Vector3::<f32>::from(chunk[0].position)
    ).collect();
    assert_eq!(axes[0].cross(axes[1]), axes[2]);
}

#[test]
pub fn ensure_axes_are_axis_aligned() {
    let vertices = AxesBuilder::new()
                   .build_vertices()
                   .expect("Failed to build vertices");
    for chunk in vertices.chunks(2) {
        let p0 = Vector3::<f32>::from(chunk[0].position);
        let p1 = Vector3::<f32>::from(chunk[1].position);
        let dir = p1 - p0;
        assert!(dir[0] == 1.0 || dir[1] == 1.0 || dir[2] == 1.0);
        assert_eq!(dir.magnitude(), 1.0);
    }
}

#[test]
pub fn ensure_axes_normals_define_axis_direction() {
    let vertices = AxesBuilder::new()
                   .build_vertices()
                   .expect("Failed to build vertices");
    for chunk in vertices.chunks(2) {
        let p0 = Vector3::<f32>::from(chunk[0].position);
        let p1 = Vector3::<f32>::from(chunk[1].position);
        let dir = p1 - p0;
        assert_eq!(dir, Vector3::<f32>::from(chunk[0].normal));
        assert_eq!(dir, Vector3::<f32>::from(chunk[1].normal));
    }
}
