//! Formatting
//!
//! Formatting can be used to declare a style,
//! or used in within a document direct.

mod bold;
mod border;
mod character_property;
mod color;
mod dstrike;
mod indent_level;
mod italics;
mod justification;
mod numbering_id;
mod numbering_property;
mod outline;
mod paragraph_property;
mod size;
mod strike;
mod underline;

// re-export
pub use self::{
    bold::*, border::*, character_property::*, color::*, dstrike::*, indent_level::*, italics::*,
    justification::*, numbering_id::*, numbering_property::*, outline::*, paragraph_property::*,
    size::*, strike::*, underline::*,
};
