use yew::{
    prelude::*,
};

use crate::components::{Button, ButtonVariant};


pub struct Chip;

#[derive(Clone, PartialEq, Properties)]
pub struct ChipProperties
{
    pub children: Children,
    #[prop_or_default]
    pub is_read_only: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for Chip
{
    type Message = ();
    type Properties = ChipProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div
                class={"pf-c-chip"}
            >
                <span class={"pf-c-chip__text"}>
                    {ctx.props().children.clone()}
                </span>
                {
                    if !ctx.props().is_read_only
                    {
                        html!{
                            <Button
                                variant={ButtonVariant::Plain}
                                onclick={ctx.props().onclick.clone()}
                            >
                                <i class={"fas fa-times"}></i>
                            </Button>
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
            </div>
        }
    }
}
