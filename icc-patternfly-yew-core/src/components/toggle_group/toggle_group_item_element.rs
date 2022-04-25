use yew::{
    prelude::*,
};


#[derive(Clone, PartialEq)]
pub enum ToggleGroupItemVariant
{
    Icon,
    Text,
}

pub struct ToggleGroupItemElement;

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

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <span 
                class={classes!(
                    if ctx.props().variant == ToggleGroupItemVariant::Icon { "pf-c-toggle-group__icon" } else { "" },
                    if ctx.props().variant == ToggleGroupItemVariant::Text { "pf-c-toggle-group__text" } else { "" },
                )}
            >
                { for ctx.props().children.iter() }
            </span>
        }
    }
}
