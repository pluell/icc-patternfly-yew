use web_sys::HtmlElement;
use yew::{prelude::*, virtual_dom::VNode};

use crate::PageSidebarContext;
use super::{NavContext, NavSelectClickHandler, NavContextOnSelectProps, NavSelectedItem};


pub struct NavItem
{
    nav_context: NavContext,
    _nav_context_listener: ContextHandle<NavContext>,
    sidebar_context: PageSidebarContext,
    _sidebar_context_listener: ContextHandle<PageSidebarContext>,
    nav_item_ref: NodeRef,
    child_item_tabindex_is_null: bool,
}

#[derive(Clone, PartialEq, Properties)]
pub struct NavItemProps
{
    /** Content rendered inside the nav item. */
    #[prop_or_default]  
    pub children: Html,
    /** Whether to set className on children when React.isValidElement(children) */
    #[prop_or_default]
    pub style_children: bool,
    /** Additional classes added to the nav item */
    #[prop_or_default]
    pub classes: Classes,
    /** Target navigation link. Should not be used if the flyout prop is defined. */
    #[prop_or_default]
    pub to: Option<AttrValue>,
    /** Flag indicating whether the item is active */
    #[prop_or_default]
    pub is_active: bool,
    /** Group identifier, will be returned with the onToggle and onSelect callback passed to the Nav component */
    #[prop_or_default]
    pub group_id: Option<AttrValue>,
    /** Item identifier, will be returned with the onToggle and onSelect callback passed to the Nav component */
    #[prop_or_default]
    pub item_id: Option<AttrValue>,
    /** If true prevents the default anchor link action to occur. Set to true if you want to handle navigation yourself. */
    #[prop_or_default]
    pub prevent_default: bool,
    /** Callback for item click */
    #[prop_or_default]
    pub onclick: Option<NavSelectClickHandler>,
    /** Component used to render NavItems if  React.isValidElement(children) is false */
    #[prop_or(String::from("a"))]
    pub component: String,
    /** Flyout of a nav item. This should be a Menu component. Should not be used if the to prop is defined. */
    #[prop_or_default]
    pub flyout: Option<Html>,
    /** Callback when flyout is opened or closed */
    #[prop_or_default]
    pub on_show_flyout: Option<Callback<()>>,
    /** z-index of the flyout nav item */
    #[prop_or_default]
    pub zindex: Option<i32>,
    /** Value to overwrite the randomly generated data-ouia-component-id.*/
    #[prop_or_default]
    pub ouia_id: Option<i32>,
    /** Set the value of data-ouia-safe. Only set to true when the component is in a static state, i.e. no animations are occurring. At all other times, this value must be false. */
    #[prop_or_default]
    pub ouia_safe: bool,
    /** @beta Adds a wrapper around the nav link text. Improves the layout when the text is a react node. */
    #[prop_or_default]
    pub has_nav_link_wrapper: bool,
}

pub enum NavItemMessage
{
    NavContext(NavContext),
    SidebarContext(PageSidebarContext),
    OnClick(MouseEvent),
}

impl Component for NavItem
{
    type Message = NavItemMessage;
    type Properties = NavItemProps;

    fn create(ctx: &Context<Self>) -> Self
    {
        let (nav_context, _nav_context_listener) = ctx
            .link()
            .context(ctx.link().callback(Self::Message::NavContext))
            .expect("No Nav Context Provided");
        
        let (sidebar_context, _sidebar_context_listener) = ctx
            .link()
            .context(ctx.link().callback(Self::Message::SidebarContext))
            .expect("No Sidebar Context Provided");

        Self {
            nav_context,
            _nav_context_listener,
            sidebar_context,
            _sidebar_context_listener,
            nav_item_ref: NodeRef::default(),
            child_item_tabindex_is_null: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg : Self::Message) -> bool
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
            Self::Message::OnClick(event) => {
                let prevent_default = ctx.props().prevent_default || ctx.props().to.is_none();

                let group_id = ctx.props().group_id.as_ref().map(|group_id| group_id.to_string()).unwrap_or(String::new());

                let item_id = ctx.props().item_id.as_ref().map(|item_id| item_id.to_string()).unwrap_or(String::new());

                let to = ctx.props().to.as_ref().map(|to| to.to_string()).unwrap_or(String::new());

                self.nav_context.onselect.emit(NavContextOnSelectProps {
                    event,
                    selected_item: NavSelectedItem {
                        group_id,
                        item_id,
                        to,
                    },
                    prevent_default,
                    onclick: ctx.props().onclick.clone(),
                });
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let has_flyout = ctx.props().flyout.is_some();

        html!{
            <>
                <li
                    // onMouseOver={onMouseOver}
                    class={classes!(
                        "pf-v5-c-nav__item", 
                        if has_flyout {"pf-m-flyout"} else {""},
                        ctx.props().classes.clone(),
                    )}
                    ref={self.nav_item_ref.clone()}
                    // {...ouiaProps}
                >
                {
                    if self.is_child_valid_element(ctx) {
                        ctx.props().children.clone()
                    } else {
                        self.view_default_link(ctx)
                    }
                }
                </li>
                // {flyout && flyoutPopper}
            </>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool)
    {
        // Only modify child element if not using the default link
        if self.is_child_valid_element(ctx)
        {
            if let Some(nav_item) = self.nav_item_ref.cast::<HtmlElement>()
            {
                if let Some(child) = nav_item.first_element_child()
                {
                    let mut classes = classes!("pf-v5-c-nav__link", 
                        if ctx.props().is_active {"pf-m-current"} else {""},
                    );

                    // Add the original child's classes. Only need to do this once
                    // to avoid adding the nav item classes every time
                    if first_render
                    {
                        classes.push(child.class_name())
                    }

                    child.set_class_name(&classes.to_string());

                    // Check if the original tabindex is set
                    if first_render
                    {
                        self.child_item_tabindex_is_null = child.get_attribute("tabIndex").is_none()
                    }

                    // Set the tabIndex if the child is not set
                    if self.child_item_tabindex_is_null
                    {
                        if self.sidebar_context.is_sidebar_open {
                            let _ = child.remove_attribute("tabIndex");
                        } else {
                            let _ = child.set_attribute("tabIndex", "-1");    
                        }
                    }

                    if ctx.props().is_active {
                        let _ = child.set_attribute("aria-current", "page");
                    } else {
                        let _ = child.remove_attribute("aria-current");
                    }
                }
            }
        }
    }
}

impl NavItem
{
    fn is_child_valid_element(&self, ctx: &Context<Self>) -> bool
    {
        // If the child node is text, then the child is not a valid element
        match &ctx.props().children
        {
            VNode::VText(_) => false,
            _ => true,
        }
    }

    fn view_default_link(&self, ctx: &Context<Self>) -> Html
    {
        let has_flyout = ctx.props().flyout.is_some();
        
        let component = if has_flyout {
            "button".to_string()
        } else {
            ctx.props().component.clone()
        };

        let tabindex = if self.sidebar_context.is_sidebar_open {
            None
        } else {
            Some("-1")
        };

        html!{
            <@{component}
                href={ctx.props().to.clone()}
                onclick={ctx.link().callback(NavItemMessage::OnClick)}
                class={classes!(
                    "pf-v5-c-nav__link",
                    if ctx.props().is_active {"pf-m-current"} else {""},
                    // isHovered && styles.modifiers.hover,
                    ctx.props().classes.clone()
                )}
                aria-current={if ctx.props().is_active {Some("page")} else {None}}
                {tabindex}
                // {...(hasFlyout && { ...ariaFlyoutProps })}
                // {...props}
            >
            {
                if ctx.props().has_nav_link_wrapper {
                    html!{
                        <span class="pf-v5-nav__link-text">{ctx.props().children.clone()}</span>
                    }
                } else {
                    ctx.props().children.clone()
                }
            }
            // {flyout && flyoutButton} 
            </@>
        }
    }
}
