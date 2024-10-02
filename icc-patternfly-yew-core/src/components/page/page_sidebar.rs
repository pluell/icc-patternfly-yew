use yew::prelude::*;

use super::page_context::PageContext;
pub use super::PageTheme;


#[derive(Clone, PartialEq)]
pub struct PageSidebarContext
{
    pub is_sidebar_open: bool,
}

pub struct PageSidebar
{
    context: PageContext,
    _context_listener: ContextHandle<PageContext>,
}

pub enum PageSidebarMsg
{
    Context(PageContext),
}

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
    type Message = PageSidebarMsg;
    type Properties = PageSidebarProps;

    fn create(ctx: &Context<Self>) -> Self
    {
        let (context, _context_listener) = ctx
            .link()
            .context(ctx.link().callback(PageSidebarMsg::Context))
            .expect("No Message Context Provided");

        Self {
            context,
            _context_listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            Self::Message::Context(context) => {
                self.context = context;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let sidebar_open =  if self.context.is_managed_sidebar {
            self.context.is_sidebar_open
        } else {
            ctx.props().is_sidebar_open
        };

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
