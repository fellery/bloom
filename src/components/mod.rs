//! Screen regions assembled from stock iced widgets.
//!
//! A component is a `view` function returning an Element, with no state of its
//! own beyond what the app hands it. Anything that needs to implement the
//! Widget trait, because it draws or handles events itself, belongs in
//! `widgets` instead.

pub mod bottom_bar;
pub mod edit_panel;
pub mod info_panel;
pub mod modifier_stack;
pub mod notifications;
pub mod preferences;
pub mod timeline_bar;
pub mod viewer;
