mod dropdown;
mod dropdown_constants;
mod dropdown_item;
mod dropdown_item_types;
mod dropdown_menu;
mod dropdown_separator;
mod dropdown_toggle;
mod dropdown_with_context;
mod internal_dropdown_item;
mod kebab_toggle;
mod toggle;

pub use dropdown::*;
pub use dropdown_constants::*;
pub use dropdown_item::*;
pub use dropdown_item_types::*;
pub use dropdown_menu::*;
pub use dropdown_separator::*;
pub use dropdown_toggle::*;
pub use dropdown_with_context::*;
pub use kebab_toggle::*;

use internal_dropdown_item::*;
use toggle::*;

use yew::{Html};
use yew::virtual_dom::{VChild};

#[derive(Clone, PartialEq)]
pub enum DropdownToggleComponents
{
    DropdownToggle(VChild<DropdownToggle>),
    // DropdownToggleAction(VChild<DropdownToggleAction>),
    // DropdownToggleCheckbox(VChild<DropdownToggleCheckbox>),
    KebabToggle(VChild<KebabToggle>),
}


#[derive(Clone, PartialEq)]
pub enum DropdownItemComponentTypes
{
    Custom(Html),
    Default(&'static str),
}
