//! The GPU render path, from decoded media to the pixels on screen.
//!
//! `media` decodes a file into frames. `tiled_source` uploads those frames to
//! textures, split across a grid because one image can exceed the device's
//! maximum texture dimension. `modifier_pipeline` runs the modifier chain over
//! those tiles, one tile at a time and at a quality chosen for the current
//! zoom. `view_pipeline` composites the result with the checkerboard and the
//! pixel grid.
//!
//! The probe modules are test-only. They measure rather than assert, because
//! the memory and scaling limits they report cannot be derived from the code.

pub mod media;
pub mod view_program;

mod error;
mod gpu;
#[cfg(test)]
mod histogram_scale_probe;
#[cfg(test)]
mod large_image_probe;
pub mod modifier_pipeline;
pub mod passes;
mod scale;
#[cfg(test)]
mod test_device;
mod tiled_source;
mod view_pipeline;
mod view_primitive;
