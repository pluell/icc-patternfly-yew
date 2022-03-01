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
    VNode(Html),
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

impl From<Html> for ToolbarGroupTypes
{
    fn from(node: Html) -> Self
    {
        ToolbarGroupTypes::VNode(node)
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

impl From<Html> for ToolbarGroupChild
{
    fn from(node: Html) -> Self
    {
        Self {
            props: node.into()
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
            ToolbarGroupTypes::VNode(node) => {
                node
            }
        }
    }
}