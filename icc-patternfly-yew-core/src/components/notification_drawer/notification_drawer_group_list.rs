use yew::prelude::*;


pub struct NotificationDrawerGroupList;

#[derive(Clone, PartialEq, Properties)]
pub struct NotificationDrawerGroupListProps
{
    /**  Content rendered inside the notification drawer list body */
    #[prop_or_default]
    pub children: Html,
    /**  Additional classes added to the notification drawer list body */
    #[prop_or_default]
    pub classes: Classes,
}

impl Component for NotificationDrawerGroupList
{
    type Message = ();
    type Properties = NotificationDrawerGroupListProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div
                class={classes!(
                    "pf-v5-c-notification-drawer__group-list",
                    ctx.props().classes.clone()
                )}
                // {...props}
            >
                {ctx.props().children.clone()}
            </div>
        }
    }
}
