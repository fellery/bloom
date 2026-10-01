//! Decoding: turning a file on disk into frames the GPU path can upload.
//!
//! Every decoder produces the same thing, an RGBA buffer and its dimensions,
//! so the GPU path never asks which format a frame came from. Video and audio
//! sit behind the `av` feature because they pull in FFmpeg, and a default
//! build has to work without it.

pub mod animation;
#[cfg(feature = "av")]
pub mod audio;
pub mod exif_data;
pub mod image_data;
#[cfg(feature = "av")]
pub mod video;
