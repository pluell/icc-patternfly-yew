use yew::{
    prelude::*,
};

use super::{InternalFormFieldGroup};

pub struct FormFieldGroup
{
    props: FormFieldGroupProps,
}

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
            <InternalFormFieldGroup 
                class_name=self.props.class_name.clone()
                header=self.props.header.clone()
                // {...props}
            >
                {for self.props.children.iter()}
            </InternalFormFieldGroup>
        }
    }
}
