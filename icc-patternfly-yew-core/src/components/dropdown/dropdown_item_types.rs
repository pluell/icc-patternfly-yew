use yew::{
    prelude::*,
    virtual_dom::{VChild},
};

use super::*;


#[derive(Clone, derive_more::From, PartialEq)]
pub enum DropdownItemTypes
{
    DropdownItem(VChild<DropdownItem>),
    Separator(VChild<DropdownSeparator>),
}

#[allow(clippy::from_over_into)]
impl Into<Html> for DropdownItemTypes
{
    fn into(self) -> Html
    {
        match self
        {
            Self::DropdownItem(child) => child.into(),
            Self::Separator(child) => child.into(),
        }
    }
}