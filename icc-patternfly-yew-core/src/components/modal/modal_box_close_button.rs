use yew::{
    prelude::*,
};

use crate::{Button, ButtonVariant};


pub struct ModalBoxCloseButton
{
    props: ModalBoxCloseButtonProperties,
    link: ComponentLink<ModalBoxCloseButton>,
}

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
            ModalBoxCloseButtonMsg::OnButtonClick => {
                self.props.onclose.emit(());
            }
        }

        false
    }

    fn view(&self) -> Html
    {
        html!{
            <Button 
                class_name=self.props.class_name.clone() 
                variant=ButtonVariant::Plain
                onclick=self.link.callback(|_| ModalBoxCloseButtonMsg::OnButtonClick)
                aria_label="Close".to_string()
                // {...props}
            >
                <i class="fas fa-times" aria-hidden="true"></i>
            </Button>
        }
    }
}
