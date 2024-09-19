use yew::prelude::*;


#[derive(Clone, PartialEq)]
pub enum PageSectionVariants
{
    Default,
    Light,
    Dark,
    Darker,
}

impl PageSectionVariants
{
    fn get_class(&self) -> &'static str
    {
        match self
        {
            Self::Default => "",
            Self::Light => "pf-m-light",
            Self::Dark => "pf-m-dark-200",
            Self::Darker => "pf-m-dark-100",
        }
    }
}
  
#[derive(Clone, PartialEq)]
pub enum PageSectionTypes
{
    Default,
    Nav,
    SubNav,
    Breadcrumb,
    Tabs,
    Wizard,
}

impl PageSectionTypes
{
    fn get_class(&self) -> &'static str
    {
        match self
        {
            Self::Default => "pf-v5-c-page__main-section",
            Self::Nav => "pf-v5-c-page__main-nav",
            Self::SubNav => "pf-v5-c-page__main-subnav      ",
            Self::Breadcrumb => "pf-v5-c-page__main-breadcrumb",
            Self::Tabs => "pf-v5-c-page__main-tabs",
            Self::Wizard => "pf-v5-c-page__main-wizard",
        }
    }
}

pub struct PageSection;

#[derive(Clone, PartialEq, Properties)]
pub struct PageSectionProps
{
    /** Content rendered inside the section */
    #[prop_or_default]
    pub children: Html,
    /** Additional classes added to the section */
    #[prop_or_default]
    pub classes: Classes,
    /** Section background color variant */
    #[prop_or(PageSectionVariants::Default)]
    pub variant: PageSectionVariants,
    /** Section type variant */
    #[prop_or(PageSectionTypes::Default)]
    pub section_type: PageSectionTypes,
    /** Enables the page section to fill the available vertical space */
    #[prop_or_default]
    pub is_filled: Option<bool>,
    /** Limits the width of the section */
    #[prop_or_default]
    pub is_width_limited: bool,
    /** Flag indicating if the section content is center aligned. isWidthLimited must be set for this to work  */
    #[prop_or_default]
    pub is_center_aligned: bool,
    // /** Padding at various breakpoints. */
    // padding?: {
    //     default?: 'padding' | 'noPadding';
    //     sm?: 'padding' | 'noPadding';
    //     md?: 'padding' | 'noPadding';
    //     lg?: 'padding' | 'noPadding';
    //     xl?: 'padding' | 'noPadding';
    //     '2xl'?: 'padding' | 'noPadding';
    // };
    // /** Modifier indicating if the PageBreadcrumb is sticky to the top or bottom at various breakpoints */
    // stickyOnBreakpoint?: {
    //     default?: 'top' | 'bottom';
    //     sm?: 'top' | 'bottom';
    //     md?: 'top' | 'bottom';
    //     lg?: 'top' | 'bottom';
    //     xl?: 'top' | 'bottom';
    //     '2xl'?: 'top' | 'bottom';
    // };
    /** Modifier indicating if PageSection should have a shadow at the top */
    #[prop_or_default]
    pub has_shadow_top: bool,
    /** Modifier indicating if PageSection should have a shadow at the bottom */
    #[prop_or_default]
    pub has_shadow_bottom: bool,
    /** Flag indicating if the PageSection has a scrolling overflow */
    #[prop_or_default]
    pub has_overflow_scroll: bool,
    /** Adds an accessible name to the page section. Required when the hasOverflowScroll prop is set to true.
     * This prop should also be passed in if a heading is not being used to describe the content of the page section.
     */
    #[prop_or_default]
    pub aria_label: Option<String>,
    /** Sets the base component to render. Defaults to section */
    #[prop_or(String::from("section"))]
    pub component: String,  
}

impl Component for PageSection
{
    type Message = ();
    type Properties = PageSectionProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let fill_cls = if let Some(is_filled) = ctx.props().is_filled {
            if is_filled {
                "pf-m-fill"
            } else {
                "pf-m-no-fill"
            }
        } else {
            ""
        };

        let align_center_cls = if ctx.props().is_width_limited && 
                                        ctx.props().is_center_aligned && 
                                        ctx.props().section_type != PageSectionTypes::SubNav {
            "pf-m-align-center"
        } else {
            ""
        };

        html!{
            <@{ctx.props().component.clone()}
                // {...props}
                class={classes!(
                    ctx.props().section_type.get_class(),
                    // formatBreakpointMods(padding, styles),
                    // formatBreakpointMods(stickyOnBreakpoint, styles, 'sticky-', getVerticalBreakpoint(height), true),
                    ctx.props().variant.get_class(),
                    fill_cls,
                    if ctx.props().is_width_limited {"pf-m-limit-width"} else {""},
                    align_center_cls,
                    if ctx.props().has_shadow_top {"pf-m-shadow-top"} else {""},
                    if ctx.props().has_shadow_bottom {"pf-m-shadow-bottom"} else {""},
                    if ctx.props().has_overflow_scroll {"pf-m-overflow-scroll"} else {""},
                    ctx.props().classes.clone(),
                )}
                tabindex={if ctx.props().has_overflow_scroll{Some("0")} else {None}}
                aria-label={ctx.props().aria_label.clone()}
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
            </@>
        }
    }
}
