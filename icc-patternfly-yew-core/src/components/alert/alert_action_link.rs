use yew::{
    prelude::*,
};

use crate::{Button, ButtonVariant};


pub struct AlertActionLink
{
    props: AlertActionLinkProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct AlertActionLinkProps
{
    /** Content rendered inside the AlertLinkAction  */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the AlertActionLink  */
    #[prop_or_default]
    pub class_name: String,

    /** Button props */
    pub onclick: Callback<MouseEvent>,
}

pub enum AlertActionLinkMsg
{
    OnClick,
}

impl Component for AlertActionLink
{
    type Message = AlertActionLinkMsg;
    type Properties = AlertActionLinkProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self
    {
        Self {
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
    fn update(&mut self, _: Self::Message) -> ShouldRender
    {
        false
    }

    fn view(&self) -> Html
    {
        html!{
            <Button
                variant=ButtonVariant::Link
                onclick=self.props.onclick.clone()
                is_inline=true
                class_name=&self.props.class_name
                // {...props}
            >
            {
                for self.props.children.iter()
            }
            </Button>
        }
    }
}
