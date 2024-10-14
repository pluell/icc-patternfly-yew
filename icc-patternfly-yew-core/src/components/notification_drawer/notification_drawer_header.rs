use yew::prelude::*;

use crate::{Button, ButtonVariant, Text, TextVariants};


pub struct NotificationDrawerHeader;

#[derive(Clone, PartialEq, Properties)]
pub struct NotificationDrawerHeaderProps
{
    /**  Content rendered inside the drawer */
    #[prop_or_default]
    pub children: Option<Html>,
    /**  Additional classes for notification drawer header. */
    #[prop_or_default]
    pub classes: Classes,
    /** Adds custom accessible text to the notification drawer close button. */
    #[prop_or(AttrValue::from("Close"))]
    pub close_button_aria_label: AttrValue,
    /**  Notification drawer heading count */
    #[prop_or_default]
    pub count: Option<i32>,
    /**  Notification drawer heading custom text which can be used instead of providing count/unreadText */
    #[prop_or_default]
    pub custom_text: Option<AttrValue>,
    /**  Callback for when close button is clicked */
    #[prop_or_default]
    pub onclose: Option<Callback<()>>,
    /**  Notification drawer heading title */
    #[prop_or(AttrValue::from("Notifications"))]
    pub title: AttrValue,
    /**  Notification drawer heading unread text used in combination with a count */
    #[prop_or(AttrValue::from("unread"))]
    pub unread_text: AttrValue,
}

pub enum NotificationDrawerHeaderMessage
{
    OnClick,
}

impl Component for NotificationDrawerHeader
{
    type Message = NotificationDrawerHeaderMessage;
    type Properties = NotificationDrawerHeaderProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            Self::Message::OnClick => {
                if let Some(onclose) = &ctx.props().onclose {
                    onclose.emit(());
                }
            }
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                //{...props} 
                class={classes!(
                    "pf-v5-c-notification-drawer__header",
                    ctx.props().classes.clone()
                )}
            >
                <Text component={TextVariants::H1} classes="pf-v5-c-notification-drawer__header-title">
                    {ctx.props().title.clone()}
                </Text>
                {
                    html!{
                        if ctx.props().custom_text.is_some() || ctx.props().count.is_some() {
                            <span class="pf-v5-c-notification-drawer__header-status" aria-live="polite">
                            {
                                if let Some(custom_text) = &ctx.props().custom_text {
                                    custom_text.to_string()
                                } else {
                                    format!("{} {}", 
                                        ctx.props().count.map(|i| i.to_string()).unwrap_or(String::new()), 
                                        ctx.props().unread_text
                                    )
                                }
                            }
                            </span>
                        }
                    }
                }
                {
                    html!{
                        if ctx.props().children.is_some() || ctx.props().onclose.is_some() {
                            <div class="pf-v5-c-notification-drawer__header-action">
                                {ctx.props().children.clone()}
                                {
                                    html!{
                                        if ctx.props().onclose.is_some() {
                                            <div>
                                                <Button 
                                                    variant={ButtonVariant::Plain} 
                                                    aria_label={ctx.props().close_button_aria_label.clone()} 
                                                    onclick={ctx.link().callback(|_| Self::Message::OnClick)}
                                                >
                                                    {icc_patternfly_yew_icons::times_icon!{}}
                                                </Button>
                                            </div>
                                        }
                                    }
                                }
                            </div>
                        }
                    }
                }
            </div>
        }
    }
}
