use yew::prelude::*;


pub struct NotificationDrawer;

#[derive(Clone, PartialEq, Properties)]
pub struct NotificationDrawerProps
{
    /**  Content rendered inside the notification drawer */
    #[prop_or_default]
    pub children: Html,
    /**  Additional classes added to the notification drawer */
    #[prop_or_default]
    pub classes: Classes,
}

impl Component for NotificationDrawer
{
    type Message = ();
    type Properties = NotificationDrawerProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div
                class={classes!(
                    "pf-v5-c-notification-drawer",
                    ctx.props().classes.clone()
                )}
                // {...props}
            >
                {ctx.props().children.clone()}
            </div>
        }
    }
}
