use yew::{
    prelude::*,
    virtual_dom::{VChild},
};

use super::*;


#[derive(Clone, derive_more::From, PartialEq)]
pub enum CardChildVariant
{
    Header(VChild<CardHeader>),
    Body(VChild<CardBody>),
    Footer(VChild<CardFooter>),
    Title(VChild<CardTitle>),
    ExpandableContent(VChild<CardExpandableContent>),
    VNode(Html),
}

#[allow(clippy::from_over_into)]
impl Into<Html> for CardChildVariant
{
    fn into(self) -> Html
    {
        match self
        {
            Self::Header(child) => child.into(),
            Self::Body(child) => child.into(),
            Self::Footer(child) => child.into(),
            Self::Title(child) => child.into(),
            Self::ExpandableContent(child) => child.into(),
            Self::VNode(child) => child,
        }
    }
}