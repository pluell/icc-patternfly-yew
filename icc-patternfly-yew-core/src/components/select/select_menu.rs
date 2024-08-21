use yew::prelude::*;

use super::*;


pub struct SelectMenu;

#[derive(Clone, PartialEq, Properties)]
pub struct SelectMenuProperties
{
    pub variant: SelectVariant,
    pub children: Children,
    pub menu_ref: NodeRef,
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
                <ul ref={&ctx.props().menu_ref} class="pf-v5-c-select__menu" aria-labelledby="select-single-label">
                    { ctx.props().children.clone() }
                </ul>
            }
        }
        else
        {
            html!{
                <div ref={&ctx.props().menu_ref} class="pf-v5-c-select__menu">
                    <fieldset class="pf-v5-c-select__menu-fieldset" aria-label="Select input">
                        { ctx.props().children.clone() }
                    </fieldset>
                </div>
            }
        }
    }
}