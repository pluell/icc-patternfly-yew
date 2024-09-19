use yew::prelude::*;

pub use super::PageTheme;


#[derive(Clone, PartialEq)]
pub struct PageSidebarContext
{
    is_sidebar_open: bool,
}

pub struct PageSidebar;

#[derive(Clone, PartialEq, Properties)]
pub struct PageSidebarProps
{
    /** Additional classes added to the page sidebar */
    #[prop_or_default]
    pub classes: Classes,
    /** Content rendered inside the page sidebar (e.g. <PageSidebarBody /> */
    #[prop_or_default]
    pub children: Html,
    /**
     * If true, manages the sidebar open/close state and there is no need to pass the isSidebarOpen boolean into
     * the sidebar component or add a callback onSidebarToggle function into the PageHeader component
     */
    #[prop_or_default]
    pub is_managed_sidebar: bool,
    /** Programmatically manage if the sidebar is shown, if isManagedSidebar is set to true in the Page component, this prop is managed */
    #[prop_or_default]
    pub is_sidebar_open: bool,
    /** Indicates the color scheme of the sidebar */
    #[prop_or(PageTheme::Dark)]
    pub theme: PageTheme,
    /** Sidebar id */
    #[prop_or_default]
    pub id: Option<String>,
}

impl Component for PageSidebar
{
    type Message = ();
    type Properties = PageSidebarProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        // TODO: Implement managed is open
        let sidebar_open = ctx.props().is_sidebar_open;

        html!{
            <div
                id={ctx.props().id.clone()}
                class={classes!(
                    "pf-v5-c-page__sidebar",
                    ctx.props().theme.get_class(),
                    if sidebar_open {"pf-m-expanded"} else {"pf-m-collapsed"},
                    ctx.props().classes.clone() 
                )}
                aria-hidden={(!sidebar_open).to_string()}
                // {...props}
            >
                <ContextProvider<PageSidebarContext> 
                    context={PageSidebarContext{
                        is_sidebar_open: sidebar_open
                    }}
                >
                    {ctx.props().children.clone()}
                </ContextProvider<PageSidebarContext>>
            </div>
        }
    }
}
