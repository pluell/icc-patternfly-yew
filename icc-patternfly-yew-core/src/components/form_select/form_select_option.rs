use yew::prelude::*;


pub struct FormSelectOption;

#[derive(Clone, PartialEq, Properties)]
pub struct FormSelectOptionProps {
    /** additional classes added to the Select Option */
    #[prop_or_default]
    pub classes: Classes,
    /** the value for the option */
    #[prop_or_default]
    pub value: Option<AttrValue>,
    /** the label for the option */
    pub label: AttrValue,
    /** flag indicating if the option is disabled */
    #[prop_or_default]
    pub is_disabled: bool,
    /** flag indicating if option will have placeholder styling applied when selected **/
    #[prop_or_default]
    pub is_placeholder: bool,

    // Extra property needed to emulate setting value on select in react
    #[prop_or_default]
    pub is_selected: bool,
}

impl Component for FormSelectOption {
    type Message = ();
    type Properties = FormSelectOptionProps;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <option 
                // {...props} 
                class={ctx.props().classes.clone()} 
                value={ctx.props().value.clone()} 
                disabled={ctx.props().is_disabled}
                selected={ctx.props().is_selected}
            >
                {&ctx.props().label}
            </option>
        }
    }
}
