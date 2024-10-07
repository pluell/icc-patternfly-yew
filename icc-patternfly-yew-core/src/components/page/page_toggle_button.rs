use yew::prelude::*;

use super::page_context::PageContext;
use crate::{Button, ButtonVariant};


pub struct PageToggleButton
{
    context: PageContext,
    _context_listener: ContextHandle<PageContext>,
}

pub enum PageToggleButtonMsg
{
    Context(PageContext),
    OnClick,
}

#[derive(Clone, PartialEq, Properties)]
pub struct PageToggleButtonProps
{
    /** Content of the page toggle button */
    #[prop_or_default]
    pub children: Html,
    /** True if the sidebar is shown  */
    #[prop_or_default]
    pub is_sidebar_open: bool,
    /** Callback function to handle the sidebar toggle button, managed by the Page component if the Page isManagedSidebar prop is set to true */
    #[prop_or_default]
    pub on_sidebar_toggle: Option<Callback<()>>,
    /** Button id */
    #[prop_or(AttrValue::from("nav-toggle"))]
    pub id: AttrValue,
}

impl Component for PageToggleButton
{
    type Message = PageToggleButtonMsg;
    type Properties = PageToggleButtonProps;

    fn create(ctx: &Context<Self>) -> Self
    {
        let (context, _context_listener) = ctx
            .link()
            .context(ctx.link().callback(PageToggleButtonMsg::Context))
            .expect("No Message Context Provided");

        Self {
            context,
            _context_listener,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            Self::Message::Context(context) => {
                self.context = context;
                true
            }
            Self::Message::OnClick => { 
                let sidebar_toggle =  if self.context.is_managed_sidebar {
                    &self.context.on_sidebar_toggle
                } else {
                    &ctx.props().on_sidebar_toggle
                };

                if let Some(on_sidebar_toggle) = sidebar_toggle
                {
                    on_sidebar_toggle.emit(());
                }

                false
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

        let onclick = ctx.link().callback(|_| Self::Message::OnClick);

        html!{
            <Button
                id={ctx.props().id.clone()}
                {onclick}
                aria_label="Side navigation toggle"
                aria_expanded={if sidebar_open {"true"} else {"false"}}
                variant={ButtonVariant::Plain}
                // {...props}
            >
                {ctx.props().children.clone()}
            </Button>
        }
    }
}
