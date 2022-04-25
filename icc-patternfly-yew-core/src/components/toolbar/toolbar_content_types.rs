use yew::{
    prelude::*,
    virtual_dom::{VChild},
};

use super::*;


#[derive(Clone, derive_more::From, PartialEq)]
pub enum ToolbarContentChild
{
    Item(VChild<ToolbarItem>),
    Group(VChild<ToolbarGroup>),
    // ToggleGroup(VChild<ToolbarToggleGroup>),
}

#[allow(clippy::from_over_into)]
impl Into<Html> for ToolbarContentChild
{
    fn into(self) -> Html
    {
        match self
        {
            Self::Item(child) => child.into(),
            Self::Group(child) => child.into(),
            // Self::ToggleGroup(child) => child.into(),
        }
    }
}