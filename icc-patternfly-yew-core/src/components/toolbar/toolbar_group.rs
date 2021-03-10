use yew::{
    prelude::*,
};


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
    pub children: Children,
    #[prop_or_default]
    pub hidden: bool,
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
                class=(
                    "pf-c-toolbar__group",
                    TOOLBAR_GROUP_STYLE[self.props.variant.clone() as usize],
                    &self.props.class_name,
                )
                hidden=self.props.hidden
                aria-hidden=self.props.hidden
            >
                { self.props.children.clone() }
            </div>
        }
    }
}
