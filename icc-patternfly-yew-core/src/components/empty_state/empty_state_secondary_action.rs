use yew::{
    prelude::*,
};


pub struct EmptyStateSecondaryAction;

#[derive(Clone, PartialEq, Properties)]
pub struct EmptyStateSecondaryActionProps
{
    /** Content rendered inside the EmptyState */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the EmptyState */
    #[prop_or_default]
    pub class_name: String,
}

impl Component for EmptyStateSecondaryAction
{
    type Message = ();
    type Properties = EmptyStateSecondaryActionProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                class={classes!(
                    "pf-c-empty-state__secondary",
                    ctx.props().class_name.clone()
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
