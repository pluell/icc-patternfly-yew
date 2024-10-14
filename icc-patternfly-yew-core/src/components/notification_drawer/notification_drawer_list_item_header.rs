use yew::prelude::*;

use crate::TitleHeadingLevels;
use super::NotificationDrawerVariant;


pub struct NotificationDrawerListItemHeader;

#[derive(Clone, PartialEq, Properties)] 
pub struct NotificationDrawerListItemHeaderProps
{
    /**  Actions rendered inside the notification drawer list item header */
    #[prop_or_default]
    pub children: Option<Html>,
    /**  Additional classes for notification drawer list item header. */
    #[prop_or_default]
    pub classes: Classes,
    /**  Add custom icon for notification drawer list item header */
    #[prop_or_default]
    pub icon: Option<Html>,
    /**  Notification drawer list item header screen reader title */
    #[prop_or_default]
    pub sr_title: Option<AttrValue>,
    /**  Notification drawer list item title */
    pub title: AttrValue,
    /**  Variant indicates the severity level */
    #[prop_or(NotificationDrawerVariant::Custom)]
    pub variant: NotificationDrawerVariant,
    /** Truncate title to number of lines */
    #[prop_or(0)]
    pub truncate_title: i32,
    // /** Position of the tooltip which is displayed if text is truncated */
    // #[prop_or_default]
    // tooltipPosition?:
    /** Sets the heading level to use for the list item header title. Default is h2. */
    #[prop_or(TitleHeadingLevels::H2)]
    pub heading_level: TitleHeadingLevels,
}

impl Component for NotificationDrawerListItemHeader
{
    type Message = ();
    type Properties = NotificationDrawerListItemHeaderProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <>
                <div 
                    //{...props}
                    class={classes!("pf-v5-c-notification-drawer__list-item-header", ctx.props().classes.clone())}
                >
                    <span class="pf-v5-c-notification-drawer__list-item-header-icon">
                    {
                        // icon ? icon : <Icon />
                        if let Some(icon) = &ctx.props().icon {
                            icon.clone()
                        } else {
                            ctx.props().variant.view_icon()
                        }
                    }
                    </span>
                    // TODO: Implement tooltips
                    // {isTooltipVisible ? (
                    //     <Tooltip content={title} position={tooltipPosition}>
                    //         {Title}
                    //     </Tooltip>
                    //     ) : (
                    //     Title
                    // )}
                    {
                        self.view_title(ctx)
                    }
                </div>
                {
                    html!{
                        if let Some(children) = &ctx.props().children {
                            <div class="pf-v5-c-notification-drawer__list-item-action">{children.clone()}</div>
                        }
                    }
                }
            </>
        }
    }
}

impl NotificationDrawerListItemHeader
{
    fn view_title(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <@{ctx.props().heading_level.to_string()}
                // {...(isTooltipVisible && { tabIndex: 0 })}
                // ref={titleRef}
                class={classes!(
                    "pf-v5-c-notification-drawer__list-item-header-title", 
                    if ctx.props().truncate_title > 0 {"pf-m-truncate"} else {""},
                )}
            >
            {
                html!{
                    if let Some(sr_title) = &ctx.props().sr_title {
                        <span class="pf-v5-screen-reader">{sr_title.clone()}</span>
                    }
                }
            }
            {
                ctx.props().title.clone()
            }
            </@>
        }
    }
}