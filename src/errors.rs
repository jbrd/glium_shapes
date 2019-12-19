//! A module containing the error structures for this crate.
extern crate core;
extern crate glium;
extern crate std;

use std::error::Error;

/// The error object that is returned when a shape fails to build.
#[derive(Debug, Copy, Clone)]
pub enum ShapeCreationError {
    /// The shape failed to build because vertex buffer could not be created.
    VertexBufferCreationError(glium::vertex::BufferCreationError),

    /// The shape failed to build because the number of divisions in the u axis
    /// is too small.
    NotEnoughDivisionsInU,

    /// The shape failed to build because the number of divisions in the v axis
    /// is too small.
    NotEnoughDivisionsInV,
}

impl std::error::Error for ShapeCreationError {
    fn description(&self) -> &str {
        match &self {
            ShapeCreationError::VertexBufferCreationError(ref err) => err.description(),
            ShapeCreationError::NotEnoughDivisionsInU => "Not enough divisions in the u axis",
            ShapeCreationError::NotEnoughDivisionsInV => "Not enough divisions in the v axis",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match &self {
            ShapeCreationError::VertexBufferCreationError(ref error) => Some(error),
            _ => None,
        }
    }
}

impl From<glium::vertex::BufferCreationError> for ShapeCreationError {
    fn from(error: glium::vertex::BufferCreationError) -> Self {
        ShapeCreationError::VertexBufferCreationError(error)
    }
}

impl core::fmt::Display for ShapeCreationError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(fmt, "{}", self.description())
    }
}
