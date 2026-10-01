//! The modifier stack: what each modifier is, and what a backend needs to know
//! to run one.
//!
//! Everything here works for either backend. A modifier declares its own
//! parameters, how far it reads from its input, and what geometry it produces.
//! The GPU pipeline and the CPU export path both read those declarations.
//! Nothing here matches on a modifier kind to answer a question the modifier
//! could answer about itself. That is what lets a new modifier land without
//! edits to both backends.

pub mod cpu;
pub mod drawing_raster;
pub mod gpu;
pub mod kinds;
pub mod pixel_sort;
pub mod plan;
pub mod roi;
pub mod text_raster;
pub mod text_render;
mod types;

pub use kinds::motion_blur_samples;
pub use types::{
    InputRequest, MediaTiming, Modifier, ModifierImpl, ModifierKind, ModifierParam, ModifierType,
    StageTransform, ViewCtx, ids, tool_target,
};
