use yew::prelude::*;


pub struct FormSelectOptionGroup;

#[derive(Clone, PartialEq, Properties)]
pub struct FormSelectOptionGroupProps {
    /** content rendered inside the Select Option Group */
    #[prop_or_default]
    pub children: Children,
    /** additional classes added to the Select Option */
    #[prop_or_default]
    pub classes: Classes,
    /** the label for the option */
    pub label: AttrValue,
    /** flag indicating if the Option Group is disabled */
    #[prop_or_default]
    pub is_disabled: bool,
}

impl Component for FormSelectOptionGroup {
    type Message = ();
    type Properties = FormSelectOptionGroupProps;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <optgroup 
                // {...props}
                disabled={ctx.props().is_disabled}
                class={ctx.props().classes.clone()}
                label={&ctx.props().label}
            >
                {ctx.props().children.clone()}
            </optgroup>
        }
    }
}
