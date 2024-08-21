use yew::prelude::*;

pub struct FormAlert;

#[derive(Clone, PartialEq, Properties)]
pub struct FormAlertProperties
{
    /** An inline PatternFly Alert. */
    pub children: Children,
    /** Additional classes added to the FormAlert. */
    #[prop_or_default]
    pub class_name: String,
}

impl Component for FormAlert
{
    type Message = ();
    type Properties = FormAlertProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                // {...props} 
                class={classes!("pf-v5-c-form__alert", ctx.props().class_name.to_string())}
            >
            { ctx.props().children.clone() }
            </div>
        }
    }
}
