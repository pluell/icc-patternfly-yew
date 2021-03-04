use yew::{
    prelude::*,
};

use super::*;


pub struct SelectMenu
{
    // link: ComponentLink<Self>,
    props: SelectMenuProperties,
}

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

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self
    {
        Self {
            // link,
            props,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        self.props = props;

        true
    }

    /// Called everytime when messages are received
    fn update(&mut self, _msg: Self::Message) -> ShouldRender
    {
        false
    }

    fn view(&self) -> Html
    {
        if self.props.variant != SelectVariant::Checkbox
        {
            html!{
                <ul class="pf-c-select__menu" aria-labelledby="select-single-label">
                    { self.props.children.clone() }
                </ul>
            }
        }
        else
        {
            html!{
                <div class="pf-c-select__menu">
                    <fieldset class="pf-c-select__menu-fieldset" aria-label="Select input">
                        { self.props.children.clone() }
                    </fieldset>
                </div>
            }
        }
    }
}