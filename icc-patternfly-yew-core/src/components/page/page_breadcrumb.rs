use yew::prelude::*;


pub struct PageBreadcrumb;

#[derive(Clone, PartialEq, Properties)]
pub struct PageBreadcrumbProps
{
    /** Additional classes to apply to the PageBreadcrumb */
    #[prop_or_default]
    pub classes: Classes,
    /** Content rendered inside of the PageBreadcrumb */
    #[prop_or_default]
    pub children: Html,
    /** Limits the width of the breadcrumb */
    #[prop_or_default]
    pub is_width_limited: bool,
    // /** Modifier indicating if the PageBreadcrumb is sticky to the top or bottom at various breakpoints */
    // stickyOnBreakpoint?: {
    //     default?: 'top' | 'bottom';
    //     sm?: 'top' | 'bottom';
    //     md?: 'top' | 'bottom';
    //     lg?: 'top' | 'bottom';
    //     xl?: 'top' | 'bottom';
    //     '2xl'?: 'top' | 'bottom';
    // };
    /** Flag indicating if Page Breadcrumb should have a shadow at the top */
    #[prop_or_default]
    pub has_shadow_top: bool,
    /** Flag indicating if PageBreadcrumb should have a shadow at the bottom */
    #[prop_or_default]
    pub has_shadow_bottom: bool,
    /** Flag indicating if the PageBreadcrumb has a scrolling overflow */
    #[prop_or_default]
    pub has_overflow_scroll: bool,
    /** Adds an accessible name to the breadcrumb section. Required when the hasOverflowScroll prop is set to true. */
    #[prop_or_default]
    pub aria_label: Option<String>,
}

impl Component for PageBreadcrumb
{
    type Message = ();
    type Properties = PageBreadcrumbProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <section
                class={classes!(
                    "pf-v5-c-page__main-breadcrumb",
                    // formatBreakpointMods(stickyOnBreakpoint as Mods, styles, 'sticky-', getVerticalBreakpoint(height), true),
                    if ctx.props().is_width_limited {"pf-m-limit-width"} else {""},
                    if ctx.props().has_shadow_top {"pf-m-shadow-top"} else {""},
                    if ctx.props().has_shadow_bottom {"pf-m-shadow-bottom"} else {""},
                    if ctx.props().has_overflow_scroll {"pf-m-overflow-scroll"} else {""},
                    ctx.props().classes.clone(),
                )}
                tabindex={if ctx.props().has_overflow_scroll{Some("0")} else {None}}
                aria-label={ctx.props().aria_label.clone()}
                // {...props}
            >
            {
                if ctx.props().is_width_limited {
                    html!{
                        <div class="pf-v5-c-page__main-body">{ctx.props().children.clone()}</div>
                    }
                } else {
                    ctx.props().children.clone()
                }
            }
            </section>
        }
    }
}
