use yew::{
    prelude::*,
};

use super::*;


pub struct SelectMenu;

#[derive(Clone, PartialEq, Properties)]
pub struct SelectMenuProperties
{
    pub variant: SelectVariant,
    pub children: Children,
}

impl Component for SelectMenu
{
    type Message = ();
    type Properties = SelectMenuProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    /// Called everytime when messages are received
    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool
    {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        if ctx.props().variant != SelectVariant::Checkbox
        {
            html!{
                <ul class="pf-c-select__menu" aria-labelledby="select-single-label">
                    { ctx.props().children.clone() }
                </ul>
            }
        }
        else
        {
            html!{
                <div class="pf-c-select__menu">
                    <fieldset class="pf-c-select__menu-fieldset" aria-label="Select input">
                        { ctx.props().children.clone() }
                    </fieldset>
                </div>
            }
        }
    }
}