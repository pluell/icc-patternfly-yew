use yew::{
    prelude::*,
    virtual_dom::{VChild},
};

use super::*;


#[derive(Clone, derive_more::From, PartialEq)]
pub enum ToolbarGroupChild
{
    Item(VChild<ToolbarItem>),
    Filter(VChild<ToolbarFilter>),
    VNode(Html)
}

#[allow(clippy::from_over_into)]
impl Into<Html> for ToolbarGroupChild
{
    fn into(self) -> Html
    {
        match self
        {
            Self::Item(child) => child.into(),
            Self::Filter(child) => child.into(),
            Self::VNode(child) => child,
        }
    }
}