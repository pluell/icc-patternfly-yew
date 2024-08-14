mod text_input_group;
mod text_input_group_main;
mod text_input_group_utilities;

pub use text_input_group::*;
pub use text_input_group_main::*;
pub use text_input_group_utilities::*;

#[derive(Clone, PartialEq)]
pub struct TextInputGroupContext
{
    pub is_disabled: bool,
}