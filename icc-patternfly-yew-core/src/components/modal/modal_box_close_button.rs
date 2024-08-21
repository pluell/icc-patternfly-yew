use yew::prelude::*;

use crate::{Button, ButtonVariant};


pub struct ModalBoxCloseButton;

#[derive(Clone, PartialEq, Properties)]
pub struct ModalBoxCloseButtonProperties
{
    /** Additional classes added to the close button */
    #[prop_or_default]
    pub class_name: String,
    /** A callback for when the close button is clicked */
    #[prop_or_default]
    pub onclose: Callback<()>,
}

pub enum ModalBoxCloseButtonMsg
{
    OnButtonClick,
}

impl Component for ModalBoxCloseButton
{
    type Message = ModalBoxCloseButtonMsg;
    type Properties = ModalBoxCloseButtonProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    /// Called everytime when messages are received
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            ModalBoxCloseButtonMsg::OnButtonClick => {
                ctx.props().onclose.emit(());
            }
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <Button 
                class_name={ctx.props().class_name.clone()} 
                variant={ButtonVariant::Plain}
                onclick={ctx.link().callback(|_| ModalBoxCloseButtonMsg::OnButtonClick)}
                aria_label="Close"
                // {...props}
            >
                <i class="fas fa-times" aria-hidden="true"></i>
            </Button>
        }
    }
}
