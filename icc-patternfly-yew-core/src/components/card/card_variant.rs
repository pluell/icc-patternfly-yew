use yew::{
    prelude::*,
    virtual_dom::{VComp, VChild},
};

use super::*;

#[derive(Clone, PartialEq)]
pub enum CardVariants
{
    Header(<CardHeader as Component>::Properties),
    Body(<CardBody as Component>::Properties),
    Footer(<CardFooter as Component>::Properties),
    Title(<CardTitle as Component>::Properties),
    ExpandableContent(<CardExpandableContent as Component>::Properties),
    VNode(Html),
}

impl From<CardHeaderProperties> for CardVariants
{
    fn from(props: CardHeaderProperties) -> Self
    {
        CardVariants::Header(props)
    }
}

impl From<CardBodyProperties> for CardVariants
{
    fn from(props: CardBodyProperties) -> Self
    {
        CardVariants::Body(props)
    }
}

impl From<CardFooterProperties> for CardVariants
{
    fn from(props: CardFooterProperties) -> Self
    {
        CardVariants::Footer(props)
    }
}

impl From<CardTitleProperties> for CardVariants
{
    fn from(props: CardTitleProperties) -> Self
    {
        CardVariants::Title(props)
    }
}

impl From<CardExpandableContentProperties> for CardVariants
{
    fn from(props: CardExpandableContentProperties) -> Self
    {
        CardVariants::ExpandableContent(props)
    }
}

impl From<Html> for CardVariants
{
    fn from(node: Html) -> Self
    {
        CardVariants::VNode(node)
    }
}

#[derive(PartialEq, Clone)]
pub struct CardChildVariant
{
    pub props: CardVariants,
}

impl<CHILD> From<VChild<CHILD>> for CardChildVariant
where
    CHILD: Component,
    CHILD::Properties: Into<CardVariants>,
{
    fn from(vchild: VChild<CHILD>) -> Self
    {
        Self {
            props: vchild.props.into(),
        }
    }
}

impl From<Html> for CardChildVariant
{
    fn from(node: Html) -> Self
    {
        Self {
            props: node.into()
        }
    }
}

impl From<CardChildVariant> for Html
{
    fn from(variant: CardChildVariant) -> Html
    {
        match variant.props
        {
            CardVariants::Header(props) => {
                VComp::new::<CardHeader>(props, NodeRef::default(), None).into()
            },
            CardVariants::Body(props) => {
                VComp::new::<CardBody>(props, NodeRef::default(), None).into()
            },
            CardVariants::Footer(props) => {
                VComp::new::<CardFooter>(props, NodeRef::default(), None).into()
            },
            CardVariants::Title(props) => {
                VComp::new::<CardTitle>(props, NodeRef::default(), None).into()
            },
            CardVariants::ExpandableContent(props) => {
                VComp::new::<CardExpandableContent>(props, NodeRef::default(), None).into()
            },
            CardVariants::VNode(node) => {
                node
            }
        }
    }
}