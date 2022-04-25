use yew::{
    prelude::*,
};


pub struct ToggleGroup;

#[derive(Clone, PartialEq, Properties)]
pub struct ToggleGroupProps
{
    /** Content rendered inside the toggle group */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the toggle group */
    #[prop_or_default]
    pub class_name: String,
    /** Modifies the toggle group to include compact styling. */
    #[prop_or_default]
    pub is_compact: bool,
    /** Accessible label for the toggle group */
    #[prop_or_default]
    pub aria_label: String,
}

impl Component for ToggleGroup
{
    type Message = ();
    type Properties = ToggleGroupProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div
                class={classes!(
                    "pf-c-toggle-group", 
                    if ctx.props().is_compact { "pf-m-compact" } else { "" },
                    ctx.props().class_name.clone()
                )}
                role="group"
                aria-label={ctx.props().aria_label.clone()}
                // {...props}
            >
                { for ctx.props().children.iter() }
            </div>
        }
    }
}
