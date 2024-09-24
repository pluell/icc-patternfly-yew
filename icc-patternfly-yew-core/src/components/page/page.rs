use std::fmt;

use yew::prelude::*;

use super::page_context::PageContext;
use super::{PageGroup, PageGroupProps, PageBreadcrumbProps};


#[derive(Clone, PartialEq)]
pub enum PageMainComponent
{
    Main,
    Div
}

impl fmt::Display for PageMainComponent
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self
        {
            Self::Main => write!(f, "main"),
            Self::Div => write!(f, "div"),
        }
    }
}

pub struct Page;

#[derive(Clone, PartialEq, Properties)]
pub struct PageProps
{
    /** Content rendered inside the main section of the page layout (e.g. <PageSection />) */
    #[prop_or_default]
    pub children: Html,
    /** Additional classes added to the page layout */
    #[prop_or_default]
    pub classes: Classes,
    /** Header component (e.g. <Masthead />) */
    #[prop_or_default]
    pub header: Option<Html>,
    /** Sidebar component for a side nav (e.g. <PageSidebar />) */
    #[prop_or_default]
    pub sidebar: Option<Html>,
    /** Notification drawer component for an optional notification drawer (e.g. <NotificationDrawer />) */
    #[prop_or_default]
    pub notification_drawer: Option<Html>,
    /** Flag indicating Notification drawer in expanded */
    #[prop_or_default]
    pub is_notification_drawer_expanded: bool,
    /** Sets default drawer size */
    #[prop_or_default]
    pub drawer_default_size: Option<AttrValue>,
    /** Sets the minimum drawer size*/
    #[prop_or_default]
    pub drawer_min_size: Option<AttrValue>,
    /** Sets the maximum drawer size */
    #[prop_or_default]
    pub drawer_max_size: Option<AttrValue>,
    /** Flag indicating if breadcrumb width should be limited */
    #[prop_or_default]
    pub is_breadcrumb_width_limited: bool,
    // /** Callback when notification drawer panel is finished expanding. */
    // #[prop_or_default]
    // onNotificationDrawerExpand?: (event: KeyboardEvent | React.MouseEvent | React.TransitionEvent) => void;
    /** Skip to content component for the page */
    #[prop_or_default]
    pub skip_to_content: Option<Html>,
    /** Sets the value for role on the <main> element */
    #[prop_or_default]
    pub role: Option<AttrValue>,
    /** an id to use for the [role="main"] element */
    #[prop_or_default]
    pub main_container_id: Option<AttrValue>,
    /** tabIndex to use for the [role="main"] element, null to unset it */
    #[prop_or(Some(-1))]
    pub main_tab_index: Option<i32>,
    /**
     * If true, manages the sidebar open/close state and there is no need to pass the isSidebarOpen boolean into
     * the sidebar component or add a callback onSidebarToggle function into the Masthead component
     */
    #[prop_or_default]
    pub is_managed_sidebar: bool,
    /** Flag indicating if tertiary nav width should be limited */
    #[prop_or_default]
    pub is_tertiary_nav_width_limited: bool,
    /**
     * If true, the managed sidebar is initially open for desktop view
     */
    #[prop_or_default]
    pub default_managed_sidebar_is_open: bool,
    // /**
    //  * Can add callback to be notified when resize occurs, for example to set the sidebar isSidebarOpen prop to false for a width < 768px
    //  * Returns object { mobileView: boolean, windowSize: number }
    //  */
    // #[prop_or_default]
    // onPageResize?: ((event: MouseEvent | TouchEvent | React.KeyboardEvent, object: any) => void) | null;
    // /**
    //  * The page resize observer uses the breakpoints returned from this function when adding the pf-m-breakpoint-[default|sm|md|lg|xl|2xl] class
    //  * You can override the default getBreakpoint function to return breakpoints at different sizes than the default
    //  * You can view the default getBreakpoint function here:
    //  * https://github.com/patternfly/patternfly-react/blob/main/packages/react-core/src/helpers/util.ts
    //  */
    // #[prop_or_default]
    // getBreakpoint?: (width: number | null) => 'default' | 'sm' | 'md' | 'lg' | 'xl' | '2xl';
    // /**
    //  * The page resize observer uses the breakpoints returned from this function when adding the pf-m-breakpoint-[default|sm|md|lg|xl|2xl] class
    //  * You can override the default getVerticalBreakpoint function to return breakpoints at different sizes than the default
    //  * You can view the default getVerticalBreakpoint function here:
    //  * https://github.com/patternfly/patternfly-react/blob/main/packages/react-core/src/helpers/util.ts
    //  */
    // #[prop_or_default]
    // getVerticalBreakpoint?: (height: number | null) => 'default' | 'sm' | 'md' | 'lg' | 'xl' | '2xl';
    /** Breadcrumb component for the page */
    #[prop_or_default]
    pub breadcrumb: Option<Html>,
    /** Tertiary nav component for the page */
    #[prop_or_default]
    pub tertiary_nav: Option<Html>,
    /** Accessible label, can be used to name main section */
    #[prop_or_default]
    pub main_aria_label: Option<String>,
    /** Flag indicating if the tertiaryNav should be in a group */
    #[prop_or_default]
    pub is_tertiary_nav_grouped: bool,
    /** Flag indicating if the breadcrumb should be in a group */
    #[prop_or_default]
    pub is_breadcrumb_grouped: bool,
    /** Additional content of the group */
    #[prop_or_default]
    pub additional_grouped_content: Option<Html>,
    /** HTML component used as main component of the page. Defaults to 'main', only pass in 'div' if another 'main' element already exists. */
    #[prop_or(PageMainComponent::Main)]
    pub main_component: PageMainComponent,
    /** Additional props of the group */
    #[prop_or_default]
    pub group_props: Option<PageGroupProps>,
    /** Additional props of the breadcrumb */
    #[prop_or_default]
    pub breadcrumb_props: Option<PageBreadcrumbProps>,
}

impl Component for Page
{
    type Message = ();
    type Properties = PageProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let context = PageContext {
            is_managed_sidebar: ctx.props().is_managed_sidebar,
            on_sidebar_toggle: None, //TODO
            is_sidebar_open: false, //TODO
            width: 0, //TODO
            height: 0, //TODO
        };

        html!{
            <ContextProvider<PageContext> context={context}>
                <div
                    // ref={this.pageRef}
                    // {...rest}
                    class={classes!(
                        "pf-v5-c-page",
                        // width !== null && height !== null && 'pf-m-resize-observer',
                        // width !== null && `pf-m-breakpoint-${getBreakpoint(width)}`,
                        // height !== null && `pf-m-height-breakpoint-${getVerticalBreakpoint(height)}`,
                        ctx.props().classes.clone(),
                    )}
                >
                {
                    ctx.props().skip_to_content.clone()
                }
                {
                    ctx.props().header.clone()
                }
                {
                    ctx.props().sidebar.clone()
                }
                {
                    if let Some(_notification_drawer) = &ctx.props().notification_drawer {
                        html!{
                            //TODO: Implement Drawer
                            // <div class="pf-v5-c-page__drawer">
                            //     <Drawer isExpanded={isNotificationDrawerExpanded} onExpand={(event) => onNotificationDrawerExpand(event)}>
                            //         <DrawerContent panelContent={panelContent}>
                            //         <DrawerContentBody>{main}</DrawerContentBody>
                            //         </DrawerContent>
                            //     </Drawer>
                            // </div>
                        }
                    } else {
                        self.view_main(ctx)
                    }
                }
                </div>
            </ContextProvider<PageContext>>
        }
    }
}

impl Page
{
    fn view_main(&self, ctx: &Context<Self>) -> Html
    {
        let tabindex = if let Some(tabindex) = &ctx.props().main_tab_index {
            Some(tabindex.to_string())
        } else {
            None
        };

        html!{
            <@{ctx.props().main_component.to_string()}
                // ref={this.mainRef}
                role={ctx.props().role.clone()}
                id={ctx.props().main_container_id.clone()}
                class="pf-v5-c-page__main"
                {tabindex}
                aria-label={ctx.props().main_aria_label.clone()}
            >
            {
                self.view_group(ctx)
            }
            {
                if !ctx.props().is_tertiary_nav_grouped {
                    self.view_nav(ctx)
                } else {
                    html!{}
                }
            }
            {
                if !ctx.props().is_breadcrumb_grouped {
                    self.view_breadcrumb(ctx)
                } else {
                    html!{}
                }
            }
            {
                ctx.props().children.clone()
            }
            </@>
        }
    }

    fn view_group(&self, ctx: &Context<Self>) -> Option<Html>
    {
        if ctx.props().is_tertiary_nav_grouped ||
            ctx.props().is_breadcrumb_grouped ||
            ctx.props().additional_grouped_content.is_some()
        {
            let page_group_props = if let Some(group_props) = &ctx.props().group_props {
                group_props.clone()
            } else {
                yew::props!(PageGroupProps {})
            };
            
            Some(html!{
                <PageGroup ..page_group_props>
                {
                    if ctx.props().is_tertiary_nav_grouped {
                        self.view_nav(ctx)
                    } else {
                        html!{}
                    }
                }
                {
                    if ctx.props().is_breadcrumb_grouped {
                        self.view_breadcrumb(ctx)
                    } else {
                        html!{}
                    }
                }
                {
                    if ctx.props().is_breadcrumb_grouped {
                        self.view_breadcrumb(ctx)
                    } else {
                        html!{}
                    }
                }
                {
                    ctx.props().additional_grouped_content.clone()
                }
                </PageGroup>
            })
        }
        else
        {
            None
        }
    }

    fn view_nav(&self, ctx: &Context<Self>) ->  Html
    {
        if let Some(tertiary_nav) = &ctx.props().tertiary_nav {
            if ctx.props().is_tertiary_nav_width_limited {
                html!{
                    <div class={classes!("pf-v5-c-page__main-nav", "pf-m-limit-width")}>
                        <div class="pf-v5-c-page__main-body">{tertiary_nav.clone()}</div>
                    </div>
                }
            } else {
                html!{
                    <div class="pf-v5-c-page__main-nav">{tertiary_nav.clone()}</div>
                }
            }
        } else {
            html!{}
        }
    }

    fn view_breadcrumb(&self, ctx: &Context<Self>) -> Html
    {
        if let Some(breadcrumb) = &ctx.props().breadcrumb {
            html!{
                <section
                    class={classes!(
                        "pf-v5-c-page__main-breadcrumb",
                        if ctx.props().is_breadcrumb_width_limited {"pf-m-limit-width"} else {""},
                        // formatBreakpointMods(
                        //     breadcrumbProps?.stickyOnBreakpoint,
                        //     styles,
                        //     'sticky-',
                        //     getVerticalBreakpoint(height),
                        //     true
                        // )
                    )}
                >
                {
                    if ctx.props().is_breadcrumb_width_limited {
                        html!{<div class="pf-v5-c-page__main-body">{breadcrumb.clone()}</div>}
                    } else {
                        breadcrumb.clone()
                    }
                }
                </section>
            }
        } else {
            html!{}
        }
    }
}