use yew::prelude::*;

use super::InternalFormFieldGroup;


pub struct FormFieldGroupExpandable
{
    is_expanded: bool,
}

#[derive(Clone, PartialEq, Properties)]
pub struct FormFieldGroupExpandableProps
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
    /** Flag indicating if the form field group is initially expanded */
    #[prop_or_default]
    pub is_expanded: bool,
    /** Aria-label to use on the form filed group toggle button */
    #[prop_or_default]
    pub toggle_aria_label: Option<String>,
}

pub enum FormFieldGroupExpandableMsg
{
    OnToggle,
}

impl Component for FormFieldGroupExpandable
{
    type Message = FormFieldGroupExpandableMsg;
    type Properties = FormFieldGroupExpandableProps;

    fn create(ctx: &Context<Self>) -> Self
    {
        let is_expanded = ctx.props().is_expanded.clone();

        Self {
            is_expanded,
        }
    }

    /// Called everytime when messages are received
    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            FormFieldGroupExpandableMsg::OnToggle => {
                self.is_expanded = !self.is_expanded;

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <InternalFormFieldGroup 
                class_name={ctx.props().class_name.clone()}
                header={ctx.props().header.clone()}
                is_expandable={true}
                is_expanded={self.is_expanded}
                toggle_aria_label={ctx.props().toggle_aria_label.clone()}
                ontoggle={ctx.link().callback(|_| FormFieldGroupExpandableMsg::OnToggle)}
                // {...props}
            >
                {for ctx.props().children.iter()}
            </InternalFormFieldGroup>
        }
    }
}
