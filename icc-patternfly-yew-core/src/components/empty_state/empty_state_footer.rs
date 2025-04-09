use yew::prelude::*;


pub struct EmptyStateFooter;

#[derive(Clone, PartialEq, Properties)]
pub struct EmptyStateFooterProps
{
    /** Content rendered inside the empty state footer */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the empty state footer */
    #[prop_or_default]
    pub classes: Classes,
}

impl Component for EmptyStateFooter
{
    type Message = ();
    type Properties = EmptyStateFooterProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                class={classes!(
                    "pf-v5-c-empty-state__footer",
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
