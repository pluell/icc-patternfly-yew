use yew::prelude::*;


pub struct EmptyStateActions;

#[derive(Clone, PartialEq, Properties)]
pub struct EmptyStateActionsProps
{
    /** Content rendered inside the empty state actions */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the empty state actions */
    #[prop_or_default]
    pub classes: Classes,
}

impl Component for EmptyStateActions
{
    type Message = ();
    type Properties = EmptyStateActionsProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                class={classes!(
                    "pf-v5-c-empty-state__actions",
                    ctx.props().classes.clone()
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
