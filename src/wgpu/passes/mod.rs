//! Individual GPU passes: one shader, its pipeline, and the uniforms it takes.
//!
//! A pass takes an input texture, a target, and a region, and draws. It reads
//! nothing about the chain around it. `modifier_pipeline` decides which passes
//! run, at what size, and against which tile.

pub mod checkerboard;
pub mod chromatic_aberration;
pub mod display;
pub mod drawing;
pub mod gaussian_blur;
pub mod motion_blur;
pub mod pixel_grid;
pub mod pixel_sort;
pub mod resample;
pub mod text;
