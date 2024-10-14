use yew::prelude::*;


pub struct NotificationDrawerListItemBody;

#[derive(Clone, PartialEq, Properties)]
pub struct NotificationDrawerListItemBodyProps
{
    /**  Content rendered inside the list item body */
    #[prop_or_default]
    pub children: Html,
    /**  Additional classes added to the list item body */
    #[prop_or_default]
    pub classes: Classes,
    /**  List item timestamp */
    #[prop_or_default]
    pub timestamp: Option<Html>,
}

impl Component for NotificationDrawerListItemBody
{
    type Message = ();
    type Properties = NotificationDrawerListItemBodyProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <>
                <div 
                    // {...props}
                    class={classes!(
                        "pf-v5-c-notification-drawer__list-item-description",
                        ctx.props().classes.clone()
                    )}
                >
                    {ctx.props().children.clone()}
                </div>
                {
                    html!{
                        if let Some(timestamp) = &ctx.props().timestamp {
                            <div 
                                class={classes!(
                                    "pf-v5-c-notification-drawer__list-item-timestamp", 
                                    ctx.props().classes.clone()
                                )}
                            >
                                {timestamp.clone()}
                            </div>
                        }
                    }
                }
            </>
        }
    }
}
