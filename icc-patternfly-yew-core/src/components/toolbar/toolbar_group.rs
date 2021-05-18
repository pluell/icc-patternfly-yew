use yew::{
    prelude::*,
    html::{ChildrenRenderer},
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

pub struct ToolbarGroup
{
    props: ToolbarGroupProperties,
}

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

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self
    {
        Self {
            props,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        if self.props != props
        {
            self.props = props;
            
            true
        }
        else
        {
            false
        }
    }

    /// Called everytime when messages are received
    fn update(&mut self, _: Self::Message) -> ShouldRender
    {
        false
    }

    fn view(&self) -> Html
    {
        html!{
            <div
                class=classes!(
                    "pf-c-toolbar__group",
                    TOOLBAR_GROUP_STYLE[self.props.variant.clone() as usize],
                    &self.props.class_name,
                )
                hidden=self.props.hidden
                aria-hidden=self.props.hidden.to_string()
            >
            {
                for self.props.children.iter().map(|mut child| {
                    match child.props
                    {
                        ToolbarGroupTypes::Filter(ref mut props) => {
                            if let Some(chip_group_content_ref) = &self.props.chip_group_content_ref
                            {
                                props.chip_group_content_ref = chip_group_content_ref.clone();
                            }

                            props.update_number_filters = self.props.update_number_filters.clone();
                        },
                        _ => {}
                    }
                    
                    child
                })
            }
            </div>
        }
    }
}
