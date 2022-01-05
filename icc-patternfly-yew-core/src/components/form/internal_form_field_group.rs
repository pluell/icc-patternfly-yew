use yew::{
    prelude::*,
};

use super::{FormFieldGroupToggle};


pub struct InternalFormFieldGroup
{
    props: InternalFormFieldGroupProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct InternalFormFieldGroupProps
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
    /** Flag indicating if the field group is expandable */
    #[prop_or_default]
    pub is_expandable: bool,
    /** Flag indicate if the form field group is expanded. Modifies the card to be expandable. */
    #[prop_or_default]
    pub is_expanded: bool,
    /** Function callback called when user clicks toggle button */
    #[prop_or_default]
    pub ontoggle: Callback<()>,
    /** Aria-label to use on the form filed group toggle button */
    #[prop_or_default]
    pub toggle_aria_label: Option<String>,
}

impl Component for InternalFormFieldGroup
{
    type Message = ();
    type Properties = InternalFormFieldGroupProps;

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
            <div
                class=classes!(
                    "pf-c-form__field-group", 
                    if self.props.is_expanded && self.props.is_expandable {"pf-m-expanded"} else {""},
                    self.props.class_name.clone(),
                )
                // {...props}
            >
                {
                    if self.props.is_expandable
                    {
                        // <GenerateId prefix="form-field-group-toggle">
                        //     {id => (
                        //     <FormFieldGroupToggle
                        //         onToggle={onToggle}
                        //         isExpanded={isExpanded}
                        //         aria-label={toggleAriaLabel}
                        //         toggleId={id}
                        //         {...(headerTitleText && { 'aria-labelledby': `${header.props.titleText.id} ${id}` })}
                        //     />
                        //     )}
                        // </GenerateId>
                        html!{
                            <FormFieldGroupToggle
                                ontoggle=self.props.ontoggle.clone()
                                is_expanded=self.props.is_expanded
                                aria_label=self.props.toggle_aria_label.clone()
                                // toggle_id={id}
                                // {...(headerTitleText && { 'aria-labelledby': `${header.props.titleText.id} ${id}` })}
                            />
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
                {
                    if let Some(header) = &self.props.header
                    {
                        header.clone()
                    }
                    else
                    {
                        html!{}
                    }
                }
                {
                    if !self.props.is_expandable || (self.props.is_expanded && self.props.is_expandable)
                    {
                        html!{
                            <div class="pf-c-form__field-group-body">
                                {for self.props.children.iter()}
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
