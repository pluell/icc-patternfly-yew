use yew::{
    prelude::*,
};

use crate::components::{Button, ButtonVariant};


pub struct ChipGroup;

pub enum ChipGroupMsg
{
}

#[derive(Clone, PartialEq, Properties)]
pub struct ChipGroupProperties
{
    #[prop_or_default]
    pub default_is_open: bool,
    #[prop_or_default]
    pub category_name: String,
    #[prop_or_default]
    pub num_chips: i32,
    #[prop_or_default]
    pub is_closable: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    pub children: Children,
}

impl Component for ChipGroup
{
    type Message = ChipGroupMsg;
    type Properties = ChipGroupProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div
                class={classes!(
                    "pf-c-chip-group",
                    if ctx.props().category_name.is_empty() {""} else {"pf-m-category"},
                )}
            >
                <div class={"pf-c-chip-group__main"}>
                    { &ctx.props().category_name }
                    <ul class={"pf-c-chip-group__list"} role={"list"}>
                    {
                        for ctx.props().children.iter().map(|child|
                            html!{
                                <li class={"pf-c-chip-group__list-item"}>
                                {
                                    child.clone()
                                }
                                </li>
                            }
                        )
                    }
                    </ul>
                </div>
                {
                    if ctx.props().is_closable
                    {
                        html!{
                            <div class={"pf-c-chip-group__close"}>
                                <Button
                                    variant={ButtonVariant::Plain}
                                    onclick={ctx.props().onclick.clone()}
                                >
                                    <i class={"fas fa-times"}></i>
                                </Button>
                            </div>
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
