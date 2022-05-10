use yew::{
    prelude::*,
};

use super::{InternalDropdownItem, DropdownItemComponentTypes};
use crate::{Divider, DividerVariant};


pub struct DropdownSeparator;

#[derive(Clone, PartialEq, Properties)]
pub struct DropdownSeparatorProperties
{
    /** Classes applied to root element of dropdown item */
    #[prop_or_default]
    pub class_name: String,
    /** Click event to pass to InternalDropdownItem */
    #[prop_or_default]
    pub onclick: Callback<()>,
}

impl Component for DropdownSeparator
{
    type Message = ();
    type Properties = DropdownSeparatorProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <InternalDropdownItem
                // {...props}
                onclick={ctx.props().onclick.clone()}
                // context={context}
                component={DropdownItemComponentTypes::Custom(html!{<Divider component={DividerVariant::Div} />})}
                class_name={ctx.props().class_name.clone()}
                role="separator"
                // {...ouiaProps}
            />
        }
    }
}
