use yew::prelude::*;

use crate::{Divider, DividerVariant};

pub struct NavItemSeparator;

#[derive(Clone, PartialEq, Properties)]
pub struct NavItemSeparatorProps
{
    #[prop_or(DividerVariant::Li)]
    pub component: DividerVariant,
}

impl Component for NavItemSeparator
{
    type Message = ();
    type Properties = NavItemSeparatorProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <Divider component={ctx.props().component.clone()} />
        }
    }
}
