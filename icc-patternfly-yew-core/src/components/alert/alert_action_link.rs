use yew::prelude::*;

use crate::{Button, ButtonVariant};


pub struct AlertActionLink;

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

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <Button
                variant={ButtonVariant::Link}
                onclick={ctx.props().onclick.clone()}
                is_inline={true}
                class_name={ctx.props().class_name.clone()}
                // {...props}
            >
            {
                for ctx.props().children.iter()
            }
            </Button>
        }
    }
}
