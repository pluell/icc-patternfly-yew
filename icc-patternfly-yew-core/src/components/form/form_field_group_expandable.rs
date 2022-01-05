use yew::{
    prelude::*,
};

use super::{InternalFormFieldGroup};


pub struct FormFieldGroupExpandable
{
    link: ComponentLink<Self>,
    props: FormFieldGroupExpandableProps,
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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        let is_expanded = props.is_expanded.clone();

        Self {
            link,
            props,
            is_expanded,
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
            FormFieldGroupExpandableMsg::OnToggle => {
                self.is_expanded = !self.is_expanded;

                true
            }
        }
    }

    fn view(&self) -> Html
    {
        html!{
            <InternalFormFieldGroup 
                class_name=self.props.class_name.clone()
                header=self.props.header.clone()
                is_expandable=true
                is_expanded=self.is_expanded
                toggle_aria_label=self.props.toggle_aria_label.clone()
                ontoggle=self.link.callback(|_| FormFieldGroupExpandableMsg::OnToggle)
                // {...props}
            >
                {for self.props.children.iter()}
            </InternalFormFieldGroup>
        }
    }
}
