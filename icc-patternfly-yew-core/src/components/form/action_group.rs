use yew::prelude::*;

pub struct ActionGroup;

#[derive(Clone, PartialEq, Properties)]
pub struct ActionGroupProps
{
    /** Anything that can be rendered as ActionGroup content. */
    pub children: Children,
    /** Additional classes added to the ActionGroup. */
    #[prop_or_default]
    pub class_name: String,
}

impl Component for ActionGroup
{
    type Message = ();
    type Properties = ActionGroupProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                // {...props} 
                class={classes!("pf-v5-c-form__group", "pf-m-action", ctx.props().class_name.to_string())}
            >
                <div class={classes!("styles.pf-v5-c-form__group-control")}>
                    <div class={classes!("pf-v5-c-form__actions")}>
                        { ctx.props().children.clone() }
                    </div>
                </div>
            </div>
        }
    }
}
