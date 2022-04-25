use yew::{
    prelude::*,
};

use icc_patternfly_yew_icons::{times_icon};

use crate::{Button, ButtonVariant};


pub struct AlertActionCloseButton;

#[derive(Clone, PartialEq, Properties)]
pub struct AlertActionCloseButtonProps
{
    /** Additional classes added to the AlertActionCloseButton */
    #[prop_or_default]
    pub class_name: String,
    /** A callback for when the close button is clicked */
    #[prop_or_default]
    pub onclose: Callback<()>,
    /** Aria Label for the Close button */
    #[prop_or_default]
    pub aria_label: Option<String>,
    /** Variant Label for the Close button */
    #[prop_or_default]
    pub variant_label: String,

    /** Context variables */
    #[prop_or_default]
    pub title: String,
}

pub enum AlertActionCloseButtonMsg
{
    OnClick,
}

impl Component for AlertActionCloseButton
{
    type Message = AlertActionCloseButtonMsg;
    type Properties = AlertActionCloseButtonProps;

    fn create(_ctx: &Context<Self>) -> Self
    {
        Self
    }

    /// Called everytime when messages are received
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            AlertActionCloseButtonMsg::OnClick => {
                ctx.props().onclose.emit(());
            },
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let aria_label = if let Some(aria_label) = &ctx.props().aria_label {
            aria_label.clone()
        } else {
            format!("Close {} alert: {}", ctx.props().variant_label, ctx.props().title)
        };

        html!{
            <Button
                variant={ButtonVariant::Plain}
                onclick={ctx.link().callback(|_| AlertActionCloseButtonMsg::OnClick)}
                {aria_label}
                // {...props}
            >
            {
                times_icon!{}
            }
            </Button>
        }
    }
}
