use yew::{
    prelude::*,
};

use super::{InternalFormFieldGroup};

pub struct FormFieldGroup;

#[derive(Clone, PartialEq, Properties)]
pub struct FormFieldGroupProps
{
    /** Anything that can be rendered as form field group content. */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the form field group. */
    #[prop_or_default]
    pub class_name: String,
    /** Form filed group header */
    #[prop_or_default]
    pub header: Option<Html>,
}

impl Component for FormFieldGroup
{
    type Message = ();
    type Properties = FormFieldGroupProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <InternalFormFieldGroup 
                class_name={ctx.props().class_name.clone()}
                header={ctx.props().header.clone()}
                // {...props}
            >
                {for ctx.props().children.iter()}
            </InternalFormFieldGroup>
        }
    }
}
