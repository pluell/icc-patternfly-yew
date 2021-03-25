mod select;
mod select_menu;
mod select_option;
mod select_toggle;

pub use select::*;
pub use select_option::*;

use select_menu::*;
use select_toggle::*;

#[derive(Clone, PartialEq)]
pub enum SelectVariant
{
    Single,
    Checkbox,
    TypeAhead,
    TypeAheadMulti,
    Panel,
}