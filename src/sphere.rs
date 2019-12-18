//! A module for constructing sphere shapes.

extern crate cgmath;
extern crate glium;

use errors::ShapeCreationError;
use self::cgmath::*;
use std::f32;
use vertex::Vertex;

/// A polygonal `Sphere` object.
///
/// This object is constructed using a `SphereBuilder` object.
pub struct Sphere {
    vertices: glium::vertex::VertexBufferAny,
}

/// Allows a `Sphere` object to be passed as a source of vertices.
impl<'a> glium::vertex::IntoVerticesSource<'a> for &'a Sphere {
    fn into_vertices_source(self) -> glium::vertex::VerticesSource<'a> {
        return self.vertices.into_vertices_source();
    }
}

/// Allows a `Sphere` object to be passed as a source of indices.
impl<'a> Into<glium::index::IndicesSource<'a>> for &'a Sphere {
    fn into(self) -> glium::index::IndicesSource<'a> {
        return glium::index::IndicesSource::NoIndices {
            primitives: glium::index::PrimitiveType::TrianglesList,
        };
    }
}

/// Responsible for building and returning a `Sphere` object.
///
/// By default, the sphere is defined as a unit-sphere (e.g. a radius of 1)
/// with its centre-of-mass located at the origin. This can be overriden
/// using the transformation methods on this object.
///
/// The resultant geometry is constructed to suit OpenGL defaults - assuming
/// a right-handed coordinate system, front-facing polygons are defined in
/// counter-clock-wise order. Vertex normals point in the direction of their
/// respective face (such that the shape appears faceted when lit). Vertex
/// texture coordinates define a spherical-projection on the object.
pub struct SphereBuilder {
    matrix: cgmath::Matrix4<f32>,
    u_divisions: usize,
    v_divisions: usize,
}

impl Default for SphereBuilder {
    fn default() -> Self {
        SphereBuilder {
            matrix: cgmath::Matrix4::<f32>::identity(),
            u_divisions: 24,
            v_divisions: 12,
        }
    }
}

impl SphereBuilder {
    /// Create a new `SphereBuilder` object.
    pub fn new() -> SphereBuilder {
        Default::default()
    }

    /// Specify the number of divisions to make in the u direction (horizontal),
    /// and v direction (vertical). By default, the builder will use 12 divisions
    /// in both axes.
    pub fn with_divisions(mut self, u: usize, v: usize) -> Self {
        self.u_divisions = u;
        self.v_divisions = v;
        return self;
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
        self.matrix = cgmath::Matrix4::<f32>::from(
            cgmath::Matrix3::<f32>::from_angle_x(
                cgmath::Rad::<f32>(radians)
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
                cgmath::Rad::<f32>(radians)
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
                cgmath::Rad::<f32>(radians)
            )
        ) * self.matrix;
        return self;
    }

    /// Build a new `Sphere` object.
    pub fn build<F>(self, display: &F) -> Result<Sphere, ShapeCreationError>
        where F: glium::backend::Facade
    {
        let vertices =
            glium::vertex::VertexBuffer::<Vertex>::new(display, &self.build_vertices()?)?;

        Ok(Sphere { vertices: glium::vertex::VertexBufferAny::from(vertices) })
    }

    /// Build the shape vertices and return them in a vector.
    ///
    /// Useful if you wish to do other things with the vertices besides constructing
    /// a `Sphere` object (e.g. unit testing, further processing, etc).
    pub fn build_vertices(&self) -> Result<Vec<Vertex>, ShapeCreationError> {

        // Ensure there are enough divisions in u and v to produce valid
        // sphere geometry
        if self.u_divisions < 3 {
            return Err(ShapeCreationError::NotEnoughDivisionsInU);
        }

        if self.v_divisions < 2 {
            return Err(ShapeCreationError::NotEnoughDivisionsInV);
        }

        // Build lookup tables.
        let u_angle = 2.0 * f32::consts::PI / self.u_divisions as f32;
        let v_angle = f32::consts::PI / self.v_divisions as f32;

        fn sin_cos(val: f32) -> [f32; 2] {
            [val.sin(), val.cos()]
        }

        let u_tab = (0..(self.u_divisions + 1))
            .map(|x| sin_cos(((x % self.u_divisions) as f32) * u_angle))
            .collect::<Vec<[f32; 2]>>();

        let v_tab = (0..(self.v_divisions + 1))
            .map(|x| sin_cos((x as f32) * v_angle))
            .collect::<Vec<[f32; 2]>>();

        let indices = [0, 1, 2, 2, 1, 3];

        // Compute the normal transformation matrix.
        let normal_matrix = Matrix3::<f32>::from_cols(self.matrix.x.truncate(),
                                                      self.matrix.y.truncate(),
                                                      self.matrix.z.truncate())
            .invert()
            .unwrap_or(Matrix3::<f32>::identity())
            .transpose();

        // Build vertex array.
        let total_num_verts = self.num_vertices();
        let mut vertices = Vec::<Vertex>::with_capacity(total_num_verts);

        for v in 0..self.v_divisions {
            for u in 0..self.u_divisions {

                // Compute slice vertices
                let verts = [Vector3::<f32>::new(u_tab[u + 1][1] * v_tab[v][0],
                                                 v_tab[v][1],
                                                 u_tab[u + 1][0] * v_tab[v][0]),
                             Vector3::<f32>::new(u_tab[u + 1][1] * v_tab[v + 1][0],
                                                 v_tab[v + 1][1],
                                                 u_tab[u + 1][0] * v_tab[v + 1][0]),
                             Vector3::<f32>::new(u_tab[u][1] * v_tab[v][0],
                                                 v_tab[v][1],
                                                 u_tab[u][0] * v_tab[v][0]),
                             Vector3::<f32>::new(u_tab[u][1] * v_tab[v + 1][0],
                                                 v_tab[v + 1][1],
                                                 u_tab[u][0] * v_tab[v + 1][0])];

                let lut_coords = [(u + 1, v), (u + 1, v + 1), (u, v), (u, v + 1)];

                // Compute face index offset and count
                let (offset, count) = if v == 0 {
                    (3, 3)
                } else if v == self.v_divisions - 1 {
                    (0, 3)
                } else {
                    (0, 6)
                };

                // Compute face normal
                let v0 = &verts[indices[offset + 0]];
                let v1 = &verts[indices[offset + 1]];
                let v2 = &verts[indices[offset + 2]];
                let normal = (v1 - v0).cross(v2 - v0).normalize();

                // Emit vertices.
                for index in offset..offset + count {
                    let vpos = &verts[indices[index]];
                    let pos = self.matrix * vpos.extend(1.0);
                    let (u, v) = lut_coords[indices[index]];
                    vertices.push(Vertex {
                        position: Point3::<f32>::from_homogeneous(pos).into(),
                        normal: (normal_matrix * normal).normalize().into(),
                        texcoord: [u as f32 / self.u_divisions as f32,
                                   v as f32 / self.v_divisions as f32],
                    });
                }
            }
        }

        assert!(vertices.len() == total_num_verts);
        return Ok(vertices);
    }

    /// Returns the number of caps in the resultant sphere geometry. The current implementation
    /// will always return 2.
    pub fn num_caps(&self) -> usize {
        2
    }

    /// Returns the number of vertices generated for each cap face. The current implementation
    /// will always return 3.
    pub fn num_vertices_per_cap_face(&self) -> usize {
        3
    }

    /// Returns the total number of vertices in each cap.
    pub fn num_vertices_per_cap(&self) -> usize {
        self.num_vertices_per_cap_face() * self.u_divisions
    }

    /// Returns the number of vertical slices in the resultant sphere geometry. The resultant
    /// value will depend on the number of v divisions specified on the builder.
    pub fn num_slices(&self) -> usize {
        self.v_divisions - self.num_caps()
    }

    /// Returns the total number of vertices in each vertical slice face (e.g. excluding caps).
    /// The current implementation will always return 6.
    pub fn num_vertices_per_slice_face(&self) -> usize {
        6
    }

    /// Returns the total number of vertices in each vertical slice.
    pub fn num_vertices_per_slice(&self) -> usize {
        self.num_vertices_per_slice_face() * self.u_divisions
    }

    /// Returns the total number of vertices that will be generated by the builder.
    pub fn num_vertices(&self) -> usize {
        (self.num_vertices_per_slice() * self.num_slices()) +
        (self.num_vertices_per_cap() * self.num_caps())
    }
}

#[test]
pub fn ensure_default_sphere_is_unit_sphere() {
    let vertices = SphereBuilder::new()
        .build_vertices()
        .expect("Failed to build vertices");
    for ref vertex in vertices {
        assert_ulps_eq!(Vector3::<f32>::from(vertex.position).magnitude(), 1.0);
    }
}

#[test]
pub fn ensure_default_sphere_has_centroid_at_origin() {
    let vertices = SphereBuilder::new()
        .build_vertices()
        .expect("Failed to build vertices");
    let mut sum = Vector3::<f32>::zero();
    for ref vertex in vertices {
        sum = sum + Vector3::<f32>::from(vertex.position);
    }
    assert_ulps_eq!(sum, Vector3::<f32>::zero(), epsilon = 0.0001);
}

#[test]
pub fn ensure_default_sphere_has_outward_facing_normals() {
    let vertices = SphereBuilder::new()
        .scale(2.0, 2.0, 2.0)
        .build_vertices()
        .expect("Failed to build vertices");
    for ref vertex in vertices {
        let position = Vector3::<f32>::from(vertex.position);
        let normal = Vector3::<f32>::from(vertex.normal);
        let outside = position + normal;
        assert!(outside.x.abs() >= position.x.abs());
        assert!(outside.y.abs() >= position.y.abs());
        assert!(outside.z.abs() >= position.z.abs());
    }
}

#[test]
pub fn ensure_default_sphere_has_uvs_in_unit_range() {
    let vertices = SphereBuilder::new()
        .with_divisions(4, 4)
        .build_vertices()
        .expect("Failed to build vertices");
    for ref vertex in vertices {
        assert!(vertex.texcoord[0] >= 0.0);
        assert!(vertex.texcoord[1] >= 0.0);
        assert!(vertex.texcoord[0] <= 1.0);
        assert!(vertex.texcoord[1] <= 1.0);
    }
}

#[test]
pub fn ensure_default_sphere_has_ccw_triangles() {
    let vertices = SphereBuilder::new()
        .build_vertices()
        .expect("Failed to build vertices");
    for chunk in vertices.chunks(3) {
        let v0 = Vector3::<f32>::from(chunk[0].position);
        let v1 = Vector3::<f32>::from(chunk[1].position);
        let v2 = Vector3::<f32>::from(chunk[2].position);
        let eyepos = v0 + Vector3::<f32>::from(chunk[0].normal);
        let e0 = v1 - v0;
        let e1 = v2 - v0;
        let n = e0.cross(e1);
        assert!(n.dot(v0 - eyepos) <= 0.0);
        assert!(n.dot(v1 - eyepos) <= 0.0);
        assert!(n.dot(v2 - eyepos) <= 0.0);
    }
}

#[test]
pub fn ensure_default_sphere_has_faceted_normals() {
    let vertices = SphereBuilder::new()
        .build_vertices()
        .expect("Failed to build vertices");

    for chunk in vertices.chunks(3) {
        let v0 = Vector3::<f32>::from(chunk[0].position);
        let v1 = Vector3::<f32>::from(chunk[1].position);
        let v2 = Vector3::<f32>::from(chunk[2].position);
        let n0 = Vector3::<f32>::from(chunk[0].normal);
        let n1 = Vector3::<f32>::from(chunk[1].normal);
        let n2 = Vector3::<f32>::from(chunk[2].normal);
        let e0 = v1 - v0;
        let e1 = v2 - v0;
        let n = e0.cross(e1).normalize();
        assert_ulps_eq!(n, n0, epsilon = 0.0001);
        assert_ulps_eq!(n, n1, epsilon = 0.0001);
        assert_ulps_eq!(n, n2, epsilon = 0.0001);
    }
}

#[test]
pub fn ensure_default_sphere_has_planar_quads() {
    let builder = SphereBuilder::new();
    let vertices = builder.build_vertices()
        .expect("Failed to build vertices");

    let mut index = builder.num_vertices_per_cap();
    for _ in 0..builder.num_slices() {
        for _ in 0..builder.num_vertices_per_slice() / 6 {
            let tri0 = [Vector3::<f32>::from(vertices[index + 0].position),
                        Vector3::<f32>::from(vertices[index + 1].position),
                        Vector3::<f32>::from(vertices[index + 2].position)];

            let tri1 = [Vector3::<f32>::from(vertices[index + 3].position),
                        Vector3::<f32>::from(vertices[index + 4].position),
                        Vector3::<f32>::from(vertices[index + 5].position)];

            index += 6;

            let n0 = (tri0[1] - tri0[0]).cross(tri0[2] - tri0[0]).normalize();
            let n1 = (tri1[1] - tri1[0]).cross(tri1[2] - tri1[0]).normalize();
            assert_ulps_eq!(n0, n1, epsilon = 0.0001);
        }
    }
}
