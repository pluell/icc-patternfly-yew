use yew::prelude::*;

use crate::{Button, ButtonVariant};


pub struct ModalBoxCloseButton;

#[derive(Clone, PartialEq, Properties)]
pub struct ModalBoxCloseButtonProperties
{
    /** Additional classes added to the close button */
    #[prop_or_default]
    pub classes: Classes,
    /** A callback for when the close button is clicked */
    #[prop_or_default]
    pub onclose: Callback<()>,
    /** Accessible descriptor of the close button. */
    #[prop_or(AttrValue::from("Close"))]
    pub aria_label: AttrValue,
    /** Value to set the data-ouia-component-id.*/
    #[prop_or_default]
    pub ouia_id: Option<AttrValue>,
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
            <div class={classes!("pf-v5-c-modal-box__close", ctx.props().classes.clone())}>
                <Button 
                    variant={ButtonVariant::Plain}
                    onclick={ctx.link().callback(|_| ModalBoxCloseButtonMsg::OnButtonClick)}
                    aria_label={ctx.props().aria_label.clone()}
                    // ouiaId={
                    //     if let Some(ouia_id) = &ctx.props().ouia_id {
                    //         Some(format!("{}-ModalBoxCloseButton"))
                    //     } else {
                    //         None
                    //     }
                    // }
                    // {...props}
                >
                    {icc_patternfly_yew_icons::times_icon!()}
                </Button>
            </div>
        }
    }
}
