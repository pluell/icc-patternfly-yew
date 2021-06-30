use yew::{
    prelude::*,
};

use icc_patternfly_yew_icons::{times_icon};

use crate::{Button, ButtonVariant};


pub struct AboutModalBoxCloseButton
{
    props: AboutModalBoxCloseButtonProps,
    link: ComponentLink<AboutModalBoxCloseButton>,
}

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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self {
            props,
            link,
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
            AboutModalBoxCloseButtonMsg::OnCloseClick => {
                self.props.onclose.emit(());
            }
        }

        false
    }

    fn view(&self) -> Html
    {
        html!{
            <div 
                class=classes!("pf-c-about-modal-box__close", self.props.class_name.clone())
                // {...props}
            >
                <Button 
                    variant=ButtonVariant::Plain
                    onclick=self.link.callback(|_| AboutModalBoxCloseButtonMsg::OnCloseClick)
                    aria_label=self.props.aria_label.clone()
                >
                {
                    times_icon!{}
                }
                </Button>
            </div>
        }
    }
}
