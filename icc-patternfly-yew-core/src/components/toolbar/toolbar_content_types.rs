use yew::{
    prelude::*,
    virtual_dom::{VComp, VChild},
};

use super::*;

#[derive(Clone, PartialEq)]
pub enum ToolbarContentTypes
{
    Item(<ToolbarItem as Component>::Properties),
    Group(<ToolbarGroup as Component>::Properties),
    // ToggleGroup(<ToolbarToggleGroup as Component>::Properties),
}

impl From<ToolbarItemProperties> for ToolbarContentTypes
{
    fn from(props: ToolbarItemProperties) -> Self
    {
        ToolbarContentTypes::Item(props)
    }
}

impl From<ToolbarGroupProperties> for ToolbarContentTypes
{
    fn from(props: ToolbarGroupProperties) -> Self
    {
        ToolbarContentTypes::Group(props)
    }
}

// impl From<ToolbarToggleGroupProperties> for ToolbarContentTypes
// {
//     fn from(props: ToolbarToggleGroupProperties) -> Self
//     {
//         ToolbarContentTypes::ToggleGroup(props)
//     }
// }

#[derive(PartialEq, Clone)]
pub struct ToolbarContentChild
{
    pub props: ToolbarContentTypes,
}

impl<CHILD> From<VChild<CHILD>> for ToolbarContentChild
where
    CHILD: Component,
    CHILD::Properties: Into<ToolbarContentTypes>,
{
    fn from(vchild: VChild<CHILD>) -> Self
    {
        Self {
            props: vchild.props.into(),
        }
    }
}

impl From<ToolbarContentChild> for Html
{
    fn from(variant: ToolbarContentChild) -> Html
    {
        match variant.props
        {
            ToolbarContentTypes::Item(props) => {
                VComp::new::<ToolbarItem>(props, NodeRef::default(), None).into()
            },
            ToolbarContentTypes::Group(props) => {
                VComp::new::<ToolbarGroup>(props, NodeRef::default(), None).into()
            },
            // ToolbarContentTypes::ToggleGroup(props) => {
            //     VComp::new::<ToolbarToggleGroup>(props, NodeRef::default(), None).into()
            // },
        }
    }
}