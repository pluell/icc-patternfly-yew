use yew::{
    prelude::*,
};


pub struct TabTextTitle;

#[derive(Clone, PartialEq, Properties)]
pub struct TabTextTitleProperties
{
    /** Text to be rendered inside the tab button title. */
    pub children: Children,
    /** additional classes added to the tab title text */
    #[prop_or_default]
    pub class_name: String,
}

impl Component for TabTextTitle
{
    type Message = ();
    type Properties = TabTextTitleProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <span class={classes!("pf-c-tabs__item-text", ctx.props().class_name.to_string())}>
                { ctx.props().children.clone() }
            </span>
        }
    }
}