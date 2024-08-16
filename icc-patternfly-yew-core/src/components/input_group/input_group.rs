use yew::prelude::*;

pub struct InputGroup;

#[derive(Clone, PartialEq, Properties)]
pub struct InputGroupProperties {
    /** Additional classes added to the input group. */
    #[prop_or_default]
    pub classes: Classes,
    /** Content rendered inside the input group. */
    #[prop_or_default]
    pub children: Children,
}

impl Component for InputGroup {
    type Message = ();
    type Properties = InputGroupProperties;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div
                class={classes!(
                    "pf-v5-c-input-group",
                    ctx.props().classes.clone(),
                )}
                // {...props}
            >
            {
                for ctx.props().children.iter()
            }
            </div>
        }
    }
}
