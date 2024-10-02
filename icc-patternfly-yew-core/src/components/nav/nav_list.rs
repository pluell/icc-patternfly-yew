use web_sys::Element;
use yew::prelude::*;

use icc_patternfly_yew_icons::{angle_left_icon, angle_right_icon};

use crate::helpers::is_element_in_view;
use crate::PageSidebarContext;
use super::NavContext;


pub struct NavList
{
    nav_context: NavContext,
    _nav_context_listener: ContextHandle<NavContext>,
    sidebar_context: PageSidebarContext,
    _sidebar_context_listener: ContextHandle<PageSidebarContext>,
    nav_list_ref: NodeRef,
    scroll_view_at_start: bool,
    scroll_view_at_end: bool,
}

#[derive(Clone, PartialEq, Properties)]
pub struct NavListProps
{
    /** Children nodes */
    #[prop_or_default]
    pub children: Html,
    /** Additional classes added to the list */
    #[prop_or_default]
    pub classes: Classes,
    /** Aria-label for the back scroll button */
    #[prop_or(AttrValue::from("Scroll back"))]
    pub back_scroll_aria_label: AttrValue,
    /** Aria-label for the forward scroll button */
    #[prop_or(AttrValue::from("Scroll forward"))]
    pub forward_scroll_aria_label: AttrValue,
}

pub enum NavListMessage
{
    NavContext(NavContext),
    SidebarContext(PageSidebarContext),
    HandleScrollButtons,
    ScrollBack,
    ScrollForward,
}

impl Component for NavList
{
    type Message = NavListMessage;
    type Properties = NavListProps;

    fn create(ctx: &Context<Self>) -> Self
    {
        let (nav_context, _nav_context_listener) = ctx
            .link()
            .context(ctx.link().callback(Self::Message::NavContext))
            .expect("No Nav Context Provided");

        let (sidebar_context, _sidebar_context_listener) = ctx
            .link()
            .context(ctx.link().callback(Self::Message::SidebarContext))
            .expect("No Message Context Provided");

        Self {
            nav_context,
            _nav_context_listener,
            sidebar_context,
            _sidebar_context_listener,
            nav_list_ref: NodeRef::default(),
            scroll_view_at_start: false,
            scroll_view_at_end: false,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool
    {
        // Recalculate scrollbars
        ctx.link().send_message(Self::Message::HandleScrollButtons);

        true
    }

    fn update(&mut self, _ctx: &Context<Self>, msg : Self::Message) -> bool
    {
        match msg
        {
            Self::Message::NavContext(context) => {
                self.nav_context = context;
                true
            }
            Self::Message::SidebarContext(context) => {
                self.sidebar_context = context;
                true
            }
            Self::Message::HandleScrollButtons => {
                if let Some(container) = self.nav_list_ref.cast::<Element>()
                {
                    // get first element and check if it is in view
                    let scroll_view_at_start = if let Some(first_child) = container.first_element_child() {
                        !is_element_in_view(&container, &first_child, false)
                    } else {
                        false
                    };
        
                    // get last element and check if it is in view
                    let scroll_view_at_end = if let Some(first_child) = container.last_element_child() {
                        !is_element_in_view(&container, &first_child, false)
                    } else {
                        false
                    };
        
                    self.nav_context.update_is_scrollable.emit(!scroll_view_at_start || !scroll_view_at_end);
            
                    self.scroll_view_at_start = scroll_view_at_start;
                    self.scroll_view_at_end = scroll_view_at_end;
                }

                true
            }
            Self::Message::ScrollBack => {
                // find first Element that is fully in view at the start, then scroll to the element before it
                if let Some(container) = self.nav_list_ref.cast::<Element>()
                {
                    let child_arr = container.children();
                    
                    let mut last_element_out_of_view = None;
                    
                    for i in 0..child_arr.length()
                    {
                        if let Some(child_element) = child_arr.get_with_index(i)
                        {
                            if is_element_in_view(&container, &child_element, false)
                            {
                                last_element_out_of_view = child_arr.get_with_index(i - 1);

                                break;
                            }
                        }
                    }
                    
                    if let Some(last_element) = last_element_out_of_view
                    {
                        container.set_scroll_left(container.scroll_left() - last_element.scroll_width());
                    }
                }
                true
            },
            Self::Message::ScrollForward => {
                // find last Element that is fully in view at the end, then scroll to the element after it
                if let Some(container) = self.nav_list_ref.cast::<Element>()
                {
                    let child_arr = container.children();
                    
                    let mut first_element_out_of_view = None;
                    
                    for i in 0..child_arr.length()
                    {
                        if let Some(child_element) = child_arr.get_with_index(i)
                        {
                            if is_element_in_view(&container, &child_element, false)
                            {
                                first_element_out_of_view = child_arr.get_with_index(i + 1);

                                break;
                            }
                        }
                    }

                    if let Some(first_element) = first_element_out_of_view
                    {
                        container.set_scroll_left(container.scroll_left() + first_element.scroll_width());
                    }
                }
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let tabindex = if self.sidebar_context.is_sidebar_open {
            None
        } else {
            Some("-1")
        };

        html!{
            <>
            {
                html!{
                    if self.nav_context.is_horizontal {
                        <button
                            class="pf-v5-c-nav__scroll-button"
                            aria-label={ctx.props().back_scroll_aria_label.clone()}
                            onclick={ctx.link().callback(|_| NavListMessage::ScrollBack)}
                            disabled={self.scroll_view_at_start}
                            tabindex={tabindex.clone()}
                        >
                            {angle_left_icon!{}}
                        </button>
                    }
                }
            }
            <ul
                ref={self.nav_list_ref.clone()}
                class={classes!("pf-v5-c-nav__list", ctx.props().classes.clone())}
                onscroll={ctx.link().callback(|_| NavListMessage::HandleScrollButtons)}
                role="list"
                // {...props}
            >
                {ctx.props().children.clone()}
            </ul>
            {
                html!{
                    if self.nav_context.is_horizontal {
                        <button
                            class="pf-v5-c-nav__scroll-button"
                            aria-label={ctx.props().forward_scroll_aria_label.clone()}
                            onclick={ctx.link().callback(|_| NavListMessage::ScrollForward)}
                            disabled={self.scroll_view_at_end}
                            tabindex={tabindex.clone()}
                        >
                            {angle_right_icon!{}}
                        </button>
                    }
                }
            }
            </>
        }
    }
}
