use yew::prelude::*;

pub use super::PageTheme;


pub struct PageGroup;

#[derive(Clone, PartialEq, Properties)]
pub struct PageGroupProps
{
    /** Additional classes to apply to the PageGroup */
    #[prop_or_default]
    pub classes: Classes,
    /** Content rendered inside of the PageGroup */
    #[prop_or_default]
    pub children: Html,
    /** Modifier indicating if the PageBreadcrumb is sticky to the top or bottom at various breakpoints */
    // stickyOnBreakpoint?: {
    //     default?: 'top' | 'bottom';
    //     sm?: 'top' | 'bottom';
    //     md?: 'top' | 'bottom';
    //     lg?: 'top' | 'bottom';
    //     xl?: 'top' | 'bottom';
    //     '2xl'?: 'top' | 'bottom';
    // };
    /** Modifier indicating if PageGroup should have a shadow at the top */
    #[prop_or_default]
    pub has_shadow_top: bool,
    /** Modifier indicating if PageGroup should have a shadow at the bottom */
    #[prop_or_default]
    pub has_shadow_bottom: bool,
    /** Flag indicating if the PageGroup has a scrolling overflow */
    #[prop_or_default]
    pub has_overflow_scroll: bool,
    /** Adds an accessible name to the page group when the hasOverflowScroll prop is set to true. */
    #[prop_or_default]
    pub aria_label: Option<String>,
}

impl Component for PageGroup
{
    type Message = ();
    type Properties = PageGroupProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div
                // {...props}
                class={classes!(
                    "pf-v5-c-page__main-group",
                    // formatBreakpointMods(stickyOnBreakpoint, styles, 'sticky-', getVerticalBreakpoint(height), true),
                    if ctx.props().has_shadow_top {"pf-m-shadow-top"} else {""},
                    if ctx.props().has_shadow_bottom {"pf-m-shadow-bottom"} else {""},
                    if ctx.props().has_overflow_scroll {"pf-m-overflow-scroll"} else {""},
                    ctx.props().classes.clone(),
                )}
                tabindex={if ctx.props().has_overflow_scroll{Some("0")} else {None}}
                role={if ctx.props().has_overflow_scroll{Some("region")} else {None}}
                aria-label={if ctx.props().has_overflow_scroll{ctx.props().aria_label.clone()} else {None}}
            >
                {ctx.props().children.clone()}
            </div>
        }
    }
}
