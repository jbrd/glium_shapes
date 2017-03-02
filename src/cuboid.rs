//! A module for constructing cuboid shapes.

extern crate cgmath;
extern crate glium;

use cgmath::{Angle,EuclideanVector,Matrix,Matrix3,Point3,Rotation3,SquareMatrix,Vector3,Vector4};
use vertex::Vertex;

/// A polygonal `Cuboid` object.
///
/// This object is constructed using a `CuboidBuilder` object.
pub struct Cuboid {
    vertices: glium::vertex::VertexBufferAny
}

/// Allows a `Cuboid` object to be passed as a source of vertices.
impl<'a> glium::vertex::IntoVerticesSource<'a> for &'a Cuboid {
    fn into_vertices_source(self) -> glium::vertex::VerticesSource<'a> {
        return self.vertices.into_vertices_source();
    }
}

/// Allows a `Cuboid` object to be passed as a source of indices.
impl<'a> Into<glium::index::IndicesSource<'a>> for &'a Cuboid {
    fn into(self) -> glium::index::IndicesSource<'a> {
        return glium::index::IndicesSource::NoIndices{
            primitives: glium::index::PrimitiveType::TrianglesList
        };
    }
}

/// Responsible for building and returning a `Cuboid` object.
///
/// By default, the cuboid is defined as a unit-cube with its centre-of-mass
/// located at the origin. This can be overriden using the transformation
/// methods on this object.
///
/// The resultant geometry is constructed to suit OpenGL defaults - assuming
/// a right-handed coordinate system, front-facing polygons are defined in
/// counter-clock-wise order. Vertex normals point in the direction of their
/// respective face (such that the cuboid appears faceted when lit). Vertex
/// texture coordinates define a planar-projection on each face.
pub struct CuboidBuilder {
    matrix: cgmath::Matrix4<f32>
}

impl Default for CuboidBuilder {
    fn default() -> Self {
        CuboidBuilder {
            matrix: cgmath::Matrix4::<f32>::identity()
        }
    }
}

impl CuboidBuilder {

    /// Create a new `CuboidBuilder` object.
    pub fn new() -> CuboidBuilder {
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

    /// Build a new `Cuboid` object.
    pub fn build<F>(self, display: &F) -> Cuboid where F:glium::backend::Facade {
        let vertices = glium::vertex::VertexBuffer::<Vertex>::new(
            display, &self.build_vertices()
        ).unwrap();

        Cuboid {
            vertices: glium::vertex::VertexBufferAny::from(vertices),
        }
    }

    /// Build the cube vertices and return them in a vector.
    ///
    /// Useful if you wish to do other things with the vertices besides constructing
    /// a `Cuboid` object (e.g. unit testing, further processing, etc).
    pub fn build_vertices(&self) -> Vec<Vertex> {

        // Define lookup-tables used during construction of the cuboid geometry
        let index_lut = [
            0, 4, 1, 5,
            6, 2, 7, 3,
            0, 2, 4, 6,
            5, 7, 1, 3,
            2, 0, 3, 1,
            4, 6, 5, 7,
        ];
        let poly_lut = [0, 1, 2, 2, 1, 3];
        let num_sides = 6;
        let verts_per_side = 6;

        // Compute the normal transformation matrix.
        let normal_matrix = Matrix3::<f32>::from_cols(
            self.matrix.x.truncate(),
            self.matrix.y.truncate(),
            self.matrix.z.truncate()
        ).invert().unwrap_or(Matrix3::<f32>::identity()).transpose();

        // Generate cuboid vertices.
        let mut vertices = Vec::<Vertex>::with_capacity(
            verts_per_side * num_sides
        );

        for side in 0..num_sides {

            // Compute side normal.
            let mut normal = Vector3::<f32>::new(0.0, 0.0, 0.0);
            normal[ side / 2 ] = ( ( ( side % 2 ) * 2 ) as f32 ) - 1.0;

            // Build side vertices.
            for vert in 0..verts_per_side {

                let coord = index_lut[ poly_lut[ vert ] + ( side * 4 ) ];
                vertices.push(Vertex{
                    position: Point3::<f32>::from_homogeneous(self.matrix * Vector4::<f32>::new(
                        (((coord & 2) - 1) as f32) * 0.5,
                        (((coord & 1) * 2 - 1) as f32) * 0.5,
                        ((((coord >> 1) & 2) - 1) as f32) * 0.5,
                        1.0
                    )).into(),
                    normal: (normal_matrix * normal).normalize().into(),
                    texcoord: [
                        ( poly_lut[ vert ] % 2 ) as f32,
                        ( poly_lut[ vert ] / 2 ) as f32,
                    ],
                });
            }
        }

        return vertices;
    }
}
