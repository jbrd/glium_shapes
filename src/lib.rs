//! Prefabricated shapes for the Glium OpenGL library.
//!
//! # Building a shape
//!
//! Each shape is constructed using a builder object. The builder objects allow us
//! to customise the resultant shape.
//!
//! In the following example, we use a `CuboidBuilder` to construct a 2x3x4 `Cuboid`
//! with its base at the origin:
//!
//! ```ignore
//! let cuboid = glium_shapes::cuboid::CuboidBuilder::new()
//!              .translate(0.0, 0.5, 0.0)
//!              .scale(2.0, 3.0, 4.0)
//!              .build(display)
//!              .expect("Failed to build cuboid shape");
//! ```
//!
//! # Drawing a shape
//!
//! All of the shapes provided in this library are a source of both vertices and
//! indices, such that you can pass them directly to the `glium::Surface::draw` method
//! like so:
//!
//! ```ignore
//! frame.draw(/*vertices=*/&cuboid, /*indices=*/&cuboid, program, uniforms, params);
//! ```
//!
//! The shader program, uniform buffers, and draw parameters are not provided by this library.

extern crate approx;

#[macro_use]
extern crate glium;

pub mod axes;
pub mod cuboid;
pub mod errors;
pub mod quad;
pub mod sphere;
pub mod vertex;
