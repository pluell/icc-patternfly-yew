use yew::prelude::*;

pub struct Form;

#[derive(Clone, PartialEq, Properties)]
pub struct FormProperties
{
    /** Anything that can be rendered as Form content. */
    pub children: Children,
    /** Additional classes added to the Form. */
    #[prop_or_default]
    pub class_name: String,
    /** Sets the Form to horizontal. */
    #[prop_or_default]
    pub is_horizontal: bool,
    /** Flag to limit the max-width to 500px. */
    #[prop_or_default]
    pub is_width_limited: bool,

    // Additional properties
    #[prop_or_default]
    pub r#ref: NodeRef,
}

impl Component for Form
{
    type Message = ();
    type Properties = FormProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <form
                ref={&ctx.props().r#ref}
                no_validate={true.to_string()}
                class={classes!(
                    "pf-v5-c-form",
                    if ctx.props().is_horizontal {"pf-m-horizontal"} else {""},
                    if ctx.props().is_width_limited {"pf-m-limit-width"} else {""},
                    ctx.props().class_name.to_string(),
                )}
            >
                { ctx.props().children.clone() }
            </form>
        }
    }
}
