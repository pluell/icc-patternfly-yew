use web_sys::HtmlElement;
use yew::prelude::*;

use crate::KeyCodes;
use super::NotificationDrawerVariant;


pub struct NotificationDrawerListItem;

#[derive(Clone, PartialEq, Properties)]
pub struct NotificationDrawerListItemProps
{
    /**  Content rendered inside the list item */
    #[prop_or_default]
    pub children: Html,
    /**  Additional classes added to the list item */
    #[prop_or_default]
    pub classes: Classes,
    /**  Modifies the list item to include hover styles on :hover */
    #[prop_or(true)]
    pub is_hoverable: bool,
    /**  Adds styling to the list item to indicate it has been read */
    #[prop_or_default]
    pub is_read: bool,
    /**  Callback for when a list item is clicked */
    #[prop_or_default]
    pub onclick: Callback<()>,
    /**  Visually hidden text that conveys the current read state of the notification list item */
    #[prop_or_default]
    pub read_state_screen_reader_text: Option<AttrValue>,
    /**  Tab index for the list item */
    #[prop_or(0)]
    pub tab_index: i32,
    /**  Variant indicates the severity level */
    #[prop_or(NotificationDrawerVariant::Custom)]
    pub variant: NotificationDrawerVariant,
}

pub enum NotificationDrawerListItemMessage
{
    OnClick,
    OnKeyDown(KeyboardEvent),
}

impl Component for NotificationDrawerListItem
{
    type Message = NotificationDrawerListItemMessage;
    type Properties = NotificationDrawerListItemProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            Self::Message::OnClick => {
                ctx.props().onclick.emit(());
            }
            Self::Message::OnKeyDown(event) => {
                if let Some(target) = event.target_dyn_into::<HtmlElement>()
                {
                    if let Some(parent_element) = target.parent_element()
                    {
                        if !parent_element.class_list().contains("pf-v5-c-notification-drawer__list-item-action")
                        {
                            if event.key_code() == KeyCodes::Enter as u32 || event.key_code() == KeyCodes::Space as u32
                            {
                                target.click();
                            }
                        }
                    }
                }
            }
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let read_state_sr_text = if let Some(read_state_sr_text) = &ctx.props().read_state_screen_reader_text {
            read_state_sr_text.clone()
        } else {
            if ctx.props().is_read {
                AttrValue::from("read")
            } else {
                AttrValue::from("unread")
            }
        };

        html!{
            <li
                // {...props}
                class={classes!(
                    "pf-v5-c-notification-drawer__list-item",
                    if ctx.props().is_hoverable {"pf-m-hoverable"} else {""},
                    ctx.props().variant.class(), 
                    if ctx.props().is_read {"pf-m-read"} else {""},
                    ctx.props().classes.clone()
                )}
                tabindex={ctx.props().tab_index.to_string()}
                onclick={ctx.link().callback(|_| Self::Message::OnClick)}
                onkeydown={ctx.link().callback(Self::Message::OnKeyDown)}
            >
                <span class="pf-v5-screen-reader">{read_state_sr_text}</span>
                {ctx.props().children.clone()}
            </li>
        }
    }
}
