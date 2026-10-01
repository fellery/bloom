//! Custom widgets, each implementing iced's Widget trait directly.
//!
//! These exist because iced has no equivalent, or because the stock version
//! cannot do what this app needs. An overlay has to measure itself in image
//! coordinates. A dropdown has to stay open across a re-layout. A numeric field
//! has to edit on drag as well as on type. Anything that can be built by
//! composing stock widgets is a `components` view function instead.

pub mod angle_dial;
pub mod color_swatch;
pub mod context_menu;
pub mod crop_overlay;
pub mod draw_overlay;
pub mod field_editor;
pub mod font_picker;
pub mod histogram;
pub mod hover_row;
pub mod loading_spinner;
pub mod logo_bloom;
pub mod menu;
pub mod menu_button;
pub mod modifier_picker;
pub mod number_entry;
pub mod option_picker;
pub mod scale_entry;
pub mod slide_in;
pub mod text_overlay;
pub mod theme_picker;
pub mod timeline;
pub mod value_slider;
pub mod viewport_nav;
