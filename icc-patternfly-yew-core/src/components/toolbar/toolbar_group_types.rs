use yew::{
    prelude::*,
    virtual_dom::{VComp, VChild},
};

use super::*;

#[derive(Clone, PartialEq)]
pub enum ToolbarGroupTypes
{
    Item(<ToolbarItem as Component>::Properties),
    Filter(<ToolbarFilter as Component>::Properties),
}

impl From<ToolbarItemProperties> for ToolbarGroupTypes
{
    fn from(props: ToolbarItemProperties) -> Self
    {
        ToolbarGroupTypes::Item(props)
    }
}

impl From<ToolbarFilterProperties> for ToolbarGroupTypes
{
    fn from(props: ToolbarFilterProperties) -> Self
    {
        ToolbarGroupTypes::Filter(props)
    }
}

#[derive(PartialEq, Clone)]
pub struct ToolbarGroupChild
{
    pub props: ToolbarGroupTypes,
}

impl<CHILD> From<VChild<CHILD>> for ToolbarGroupChild
where
    CHILD: Component,
    CHILD::Properties: Into<ToolbarGroupTypes>,
{
    fn from(vchild: VChild<CHILD>) -> Self
    {
        Self {
            props: vchild.props.into(),
        }
    }
}

impl From<ToolbarGroupChild> for Html
{
    fn from(variant: ToolbarGroupChild) -> Html
    {
        match variant.props
        {
            ToolbarGroupTypes::Item(props) => {
                VComp::new::<ToolbarItem>(props, NodeRef::default(), None).into()
            },
            ToolbarGroupTypes::Filter(props) => {
                VComp::new::<ToolbarFilter>(props, NodeRef::default(), None).into()
            },
        }
    }
}