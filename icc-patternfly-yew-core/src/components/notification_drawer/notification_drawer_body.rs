use yew::prelude::*;


pub struct NotificationDrawerBody;

#[derive(Clone, PartialEq, Properties)]
pub struct NotificationDrawerBodyProps
{
    /**  Content rendered inside the body of the notification drawer */
    #[prop_or_default]
    pub children: Html,
    /**  Additional classes added to the notification drawer body */
    #[prop_or_default]
    pub classes: Classes,
}

impl Component for NotificationDrawerBody
{
    type Message = ();
    type Properties = NotificationDrawerBodyProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div
                class={classes!(
                    "pf-v5-c-notification-drawer__body",
                    ctx.props().classes.clone()
                )}
                // {...props}
            >
                {ctx.props().children.clone()}
            </div>
        }
    }
}
