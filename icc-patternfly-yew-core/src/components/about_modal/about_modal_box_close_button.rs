use yew::prelude::*;

use icc_patternfly_yew_icons::times_icon;

use crate::{Button, ButtonVariant};


pub struct AboutModalBoxCloseButton;

#[derive(Clone, PartialEq, Properties)]
pub struct AboutModalBoxCloseButtonProps
{
    /** additional classes added to the About Modal Close button  */
    #[prop_or_default]
    pub class_name: String,
    /** A callback for when the close button is clicked  */
    #[prop_or_default]
    pub onclose: Callback<()>,
    /** Set close button aria label */
    pub aria_label: Option<String>,
}

pub enum AboutModalBoxCloseButtonMsg
{
    OnCloseClick,
}


impl Component for AboutModalBoxCloseButton
{
    type Message = AboutModalBoxCloseButtonMsg;
    type Properties = AboutModalBoxCloseButtonProps;

    fn create(_ctx: &Context<Self>) -> Self
    {
        Self
    }

    /// Called everytime when messages are received
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            AboutModalBoxCloseButtonMsg::OnCloseClick => {
                ctx.props().onclose.emit(());
            }
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                class={classes!("pf-v5-c-about-modal-box__close", ctx.props().class_name.clone())}
                // {...props}
            >
                <Button 
                    variant={ButtonVariant::Plain}
                    onclick={ctx.link().callback(|_| AboutModalBoxCloseButtonMsg::OnCloseClick)}
                    aria_label={ctx.props().aria_label.clone()}
                >
                {
                    times_icon!{}
                }
                </Button>
            </div>
        }
    }
}
