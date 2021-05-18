use yew::{
    prelude::*,
};


#[derive(Clone, PartialEq)]
pub enum ToggleGroupItemVariant
{
    Icon,
    Text,
}

pub struct ToggleGroupItemElement
{
    props: ToggleGroupItemElementProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ToggleGroupItemElementProps
{
    /** Content rendered inside the toggle group item */
    #[prop_or_default]
    pub children: Children,
    /** Adds toggle group item variant styles */
    pub variant: ToggleGroupItemVariant,
}

impl Component for ToggleGroupItemElement
{
    type Message = ();
    type Properties = ToggleGroupItemElementProps;

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
            <span 
                class=classes!(
                    if self.props.variant == ToggleGroupItemVariant::Icon { "pf-c-toggle-group__icon" } else { "" },
                    if self.props.variant == ToggleGroupItemVariant::Text { "pf-c-toggle-group__text" } else { "" },
                )
            >
                { for self.props.children.iter() }
            </span>
        }
    }
}
