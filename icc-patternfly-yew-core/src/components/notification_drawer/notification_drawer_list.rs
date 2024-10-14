use yew::prelude::*;


pub struct NotificationDrawerList;

#[derive(Clone, PartialEq, Properties)]
pub struct NotificationDrawerListProps
{
    /**  Content rendered inside the notification drawer list body */
    #[prop_or_default]
    pub children: Html,
    /**  Additional classes added to the notification drawer list body */
    #[prop_or_default]
    pub classes: Classes,
    /**  Adds styling to the notification drawer list to indicate expand/hide state */
    #[prop_or_default]
    pub is_hidden: bool,
    /** Adds an accessible label to the notification drawer list. */
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,
}

impl Component for NotificationDrawerList
{
    type Message = ();
    type Properties = NotificationDrawerListProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <ul
                // {...props}
                class={classes!(
                    "pf-v5-c-notification-drawer__list",
                    ctx.props().classes.clone()
                )}
                hidden={ctx.props().is_hidden}
                role="list"
                aria-label={ctx.props().aria_label.clone()}
            >
                {ctx.props().children.clone()}
            </ul>
        }
    }
}
