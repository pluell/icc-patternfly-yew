use yew::prelude::*;


pub struct EmptyStateBody;

#[derive(Clone, PartialEq, Properties)]
pub struct EmptyStateBodyProps
{
    /** Content rendered inside the EmptyState */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the EmptyState */
    #[prop_or_default]
    pub classes: Classes,
}

impl Component for EmptyStateBody
{
    type Message = ();
    type Properties = EmptyStateBodyProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                class={classes!(
                    "pf-v5-c-empty-state__body",
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
