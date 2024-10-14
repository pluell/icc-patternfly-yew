use yew::prelude::*;

use crate::{Badge, KeyCodes, TitleHeadingLevels};


pub struct NotificationDrawerGroup;

#[derive(Clone, PartialEq, Properties)]
pub struct NotificationDrawerGroupProps
{
    /**  Content rendered inside the group */
    #[prop_or_default]
    pub children: Html,
    /**  Additional classes added to the group */
    #[prop_or_default]
    pub classes: Classes,
    /**  Notification drawer group count */
    pub count: i32,
    /**  Adds styling to the group to indicate expanded state */
    #[prop_or_default]
    pub is_expanded: bool,
    /**  Adds styling to the group to indicate whether it has been read */
    #[prop_or_default]
    pub is_read: bool,
    /**  Callback for when group button is clicked to expand */
    #[prop_or_default]
    pub onexpand: Callback<bool>,
    /**  Notification drawer group title */
    pub title: Html,
    /** Truncate title to number of lines */
    #[prop_or(0)]
    pub truncate_title: i32,
    // /** Position of the tooltip which is displayed if text is truncated */
    // #[prop_or_default]
    // tooltipPosition?:
    /** Sets the heading level to use for the group title. Default is h1. */
    #[prop_or(TitleHeadingLevels::H1)]
    pub heading_level: TitleHeadingLevels,
}

pub enum NotificationDrawerGroupMessage
{
    OnClick,
    OnKeyDown(KeyboardEvent),
}

impl Component for NotificationDrawerGroup
{
    type Message = NotificationDrawerGroupMessage;
    type Properties = NotificationDrawerGroupProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            Self::Message::OnClick => {
                ctx.props().onexpand.emit(!ctx.props().is_expanded);
            }
            Self::Message::OnKeyDown(event) => {
                if event.key_code() == KeyCodes::Enter as u32 || event.key_code() == KeyCodes::Space as u32
                {
                    event.prevent_default();

                    ctx.props().onexpand.emit(!ctx.props().is_expanded);
                }
            }
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <section
                // {...props}
                class={classes!(
                    "pf-v5-c-notification-drawer__group", 
                    if ctx.props().is_expanded {"pf-m-expanded"} else {""},
                    ctx.props().classes.clone()
                )}
            >
                <@{ctx.props().heading_level.to_string()}>
                    <button
                        class="pf-v5-c-notification-drawer__group-toggle"
                        aria-expanded={ctx.props().is_expanded.to_string()}
                        onclick={ctx.link().callback(|_| Self::Message::OnClick)}
                        onkeydown={ctx.link().callback(Self::Message::OnKeyDown)}
                    >
                        // TODO: Implement tooltips
                        // {isTooltipVisible ? (
                        //     <Tooltip content={title} position={tooltipPosition}>
                        //     {Title}
                        //     </Tooltip>
                        // ) : (
                        //     Title
                        // )}
                        {self.view_title(ctx)}
                        <div class="pf-v5-c-notification-drawer__group-toggle-count">
                            <Badge is_read={ctx.props().is_read}>{ctx.props().count}</Badge>
                        </div>
                        <span class=".pf-v5-c-notification-drawer__group-toggle-icon">
                            {icc_patternfly_yew_icons::angle_right_icon!{}}
                        </span>
                    </button>
                </@>
                {ctx.props().children.clone()}
            </section>
        }
    }
}

impl NotificationDrawerGroup
{
    fn view_title(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div
                // {...(isTooltipVisible && { tabIndex: 0 })}
                // ref={titleRef}
                class="pf-v5-c-notification-drawer__group-toggle-title"
            >
                {ctx.props().title.clone()}
            </div>
        }
    }
}