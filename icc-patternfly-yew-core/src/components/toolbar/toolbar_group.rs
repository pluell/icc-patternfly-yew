use yew::{
    prelude::*,
    html::ChildrenRenderer,
};

use super::*;


#[derive(Clone, PartialEq)]
pub enum ToolbarGroupVariant
{
    None,
    FilterGroup,
    IconButtonGroup,
    ButtonGroup,
}

const TOOLBAR_GROUP_STYLE: &'static [&'static str] = &[
    "",
    "pf-m-filter-group",
    "pf-m-icon-button-group",
    "pf-m-button-group",
];

pub struct ToolbarGroup;

#[derive(Clone, PartialEq, Properties)]
pub struct ToolbarGroupProperties
{
    /** Classes applied to root element of the data toolbar group */
    #[prop_or_default]
    pub class_name: String,
    /** A type modifier which modifies spacing specifically depending on the type of group */
    #[prop_or(ToolbarGroupVariant::None)]
    pub variant: ToolbarGroupVariant,
    /** Content to be rendered inside the data toolbar group */
    #[prop_or_default]
    pub children: ChildrenRenderer<ToolbarGroupChild>,
    #[prop_or_default]
    pub hidden: bool,
    /** Chip group content reference for passing to data toolbar children */
    #[prop_or_default]
    pub chip_group_content_ref: Option<NodeRef>,
    #[prop_or_default]
    pub update_number_filters: Callback<(String, i32)>,
}

impl Component for ToolbarGroup
{
    type Message = ();
    type Properties = ToolbarGroupProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        // Update the child properties if necessary
        for child in ctx.props().children.iter()
        {
            match child
            {
                ToolbarGroupChild::Filter(mut child) => {
                    let mut props = (&*child.props).clone();
                    
                    if let Some(chip_group_content_ref) = &ctx.props().chip_group_content_ref
                    {
                        props.chip_group_content_ref = chip_group_content_ref.clone();
                    }

                    props.update_number_filters = ctx.props().update_number_filters.clone();

                    child.props = std::rc::Rc::new(props);
                },
                _ => {}
            }
        }

        html!{
            <div
                class={classes!(
                    "pf-v5-c-toolbar__group",
                    TOOLBAR_GROUP_STYLE[ctx.props().variant.clone() as usize],
                    &ctx.props().class_name,
                )}
                hidden={ctx.props().hidden}
                aria-hidden={ctx.props().hidden.to_string()}
            >
            {
                for ctx.props().children.iter()
            }
            </div>
        }
    }
}
