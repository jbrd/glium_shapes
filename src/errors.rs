//! A module containing the error structures for this crate.
extern crate glium;
extern crate core;
extern crate std;

use std::error::Error;

/// The error object that is returned when a shape fails to build.
#[derive(Debug,Copy,Clone)]
pub enum ShapeCreationError {

    /// The shape failed to build because vertex buffer could not be created.
    VertexBufferCreationError(glium::vertex::BufferCreationError)
}

impl From<glium::vertex::BufferCreationError> for ShapeCreationError {
    fn from(error: glium::vertex::BufferCreationError) -> Self {
        ShapeCreationError::VertexBufferCreationError(error)
    }
}

impl core::fmt::Display for ShapeCreationError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            &ShapeCreationError::VertexBufferCreationError(e) =>
                e.fmt(fmt)
        }
    }
}

impl std::error::Error for ShapeCreationError {
    fn description(&self) -> &str {
        match self {
            &ShapeCreationError::VertexBufferCreationError(_) =>
                "Error while creating vertex buffer"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &ShapeCreationError::VertexBufferCreationError(ref error) =>
                Some(error)
        }
    }
}
