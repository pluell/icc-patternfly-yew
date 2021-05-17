use yew::{
    prelude::*,
};

use icc_patternfly_yew_icons::{times_icon};

use crate::{Button, ButtonVariant};


pub struct AlertActionCloseButton
{
    link: ComponentLink<Self>,
    props: AlertActionCloseButtonProps,
}

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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self {
            link,
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
    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            AlertActionCloseButtonMsg::OnClick => {
                self.props.onclose.emit(());
            },
        }

        false
    }

    fn view(&self) -> Html
    {
        let aria_label = if let Some(aria_label) = &self.props.aria_label {
            aria_label.clone()
        } else {
            format!("Close {} alert: {}", self.props.variant_label, self.props.title)
        };

        html!{
            <Button
                variant=ButtonVariant::Plain
                onclick=self.link.callback(|_| AlertActionCloseButtonMsg::OnClick)
                aria_label=aria_label
                // {...props}
            >
            {
                times_icon!{}
            }
            </Button>
        }
    }
}
