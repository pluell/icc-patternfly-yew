// use yew::prelude::*;
use yew::{prelude::*, virtual_dom::VNode};

use icc_patternfly_yew_icons::angle_right_icon;

use crate::{ButtonProperties, PageSidebarContext};
use crate::utils_get_unique_id;

use super::NavContext;


pub struct NavExpandable
{
    expanded_state: bool,
    nav_context: NavContext,
    _nav_context_listener: ContextHandle<NavContext>,
    sidebar_context: PageSidebarContext,
    _sidebar_context_listener: ContextHandle<PageSidebarContext>,
    id: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct NavExpandableProps
{
    /** Title content shown for the expandable list */
    #[prop_or_default]
    pub title: Html,
    /** If defined, screen readers will read this text instead of the list title */
    #[prop_or_default]
    pub sr_text: Option<AttrValue>,
    /** Boolean to programatically expand or collapse section */
    #[prop_or_default]
    pub is_expanded: bool,
    /** Anything that can be rendered inside of the expandable list */
    #[prop_or_default]
    pub children: Html,
    /** Additional classes added to the container */
    #[prop_or_default]
    pub classes: Classes,
    /** Group identifier, will be returned with the onToggle and onSelect callback passed to the Nav component */
    #[prop_or_default]
    pub group_id: Option<AttrValue>,
    /** If true makes the expandable list title active */
    #[prop_or_default]
    pub is_active: bool,
    /** Identifier to use for the section aria label */
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /** allow consumer to optionally override this callback and manage expand state externally. if passed will not call Nav's onToggle. */
    #[prop_or_default]
    pub onexpand: Option<Callback<bool>>,
    /** Additional props added to the NavExpandable <button> */
    #[prop_or_default]
    pub button_props: Option<ButtonProperties>,
    /** Value to overwrite the randomly generated data-ouia-component-id.*/
    #[prop_or_default]
    pub ouia_id: Option<AttrValue>,         
}

pub enum NavExpandableMessage
{
    NavContext(NavContext),
    SidebarContext(PageSidebarContext),
    OnExpand,
}

impl Component for NavExpandable
{
    type Message = NavExpandableMessage;
    type Properties = NavExpandableProps;

    fn create(ctx: &Context<Self>) -> Self
    {
        let (nav_context, _nav_context_listener) = ctx
            .link()
            .context(ctx.link().callback(Self::Message::NavContext))
            .expect("No Message Context Provided");
        
        let (sidebar_context, _sidebar_context_listener) = ctx
            .link()
            .context(ctx.link().callback(Self::Message::SidebarContext))
            .expect("No Message Context Provided");

        // Generate a unique id if one is not provided
        let id = if let Some(id) = &ctx.props().id {
            id.to_string()
        } else {
            utils_get_unique_id(None)
        };

        Self {
            expanded_state: ctx.props().is_expanded,
            nav_context,
            _nav_context_listener,
            sidebar_context,
            _sidebar_context_listener,
            id,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool
    {
        self.expanded_state = ctx.props().is_expanded;

        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg : Self::Message) -> bool
    {
        match msg
        {
            Self::Message::SidebarContext(context) => {
                self.sidebar_context = context;
                true
            }
            Self::Message::NavContext(context) => {
                self.nav_context = context;
                true
            }
            Self::Message::OnExpand => {
                if let Some(onexpand) = &ctx.props().onexpand {
                    onexpand.emit(!self.expanded_state);

                    false
                } else {
                    let group_id = if let Some(group_id) = &ctx.props().group_id {
                        group_id.to_string()
                    } else {
                        "".to_string()
                    };

                    self.expanded_state = !self.expanded_state;

                    if let Some(ontoggle) = &self.nav_context.ontoggle
                    {
                        ontoggle.emit((group_id, !self.expanded_state));
                    }

                    true
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <li
                class={classes!(
                    "pf-v5-c-nav__item",
                    "pf-m-expandable",
                    if self.expanded_state {"pf-m-expanded"} else {""},
                    if ctx.props().is_active {"pf-m-current"} else {""},
                    ctx.props().classes.clone(),
                )}
                // {...getOUIAProps(NavExpandable.displayName, ouiaId !== undefined ? ouiaId : ouiaStateId)}
                // {...props}
            >
                <button
                    class="pf-v5-c-nav__link"
                    id={if ctx.props().sr_text.is_some() {None} else {Some(self.id.clone())}}
                    onclick={ctx.link().callback(|_| Self::Message::OnExpand)}
                    aria-expanded={self.expanded_state.to_string()}
                    tabindex={if self.sidebar_context.is_sidebar_open {None} else {Some("-1")}}
                    // {...buttonProps}
                >
                    {
                        match &ctx.props().children
                        {
                            VNode::VText(_) => ctx.props().title.clone(),
                            _ => html!{
                                <span class="pf-v5-c-nav__link-text">{ctx.props().title.clone()}</span>
                            },
                        }
                    }
                    <span class="pf-v5-c-nav__toggle">
                        <span class="pf-v5-c-nav__toggle-icon">
                            {angle_right_icon!{}}
                        </span>
                    </span>
                </button>
                <section class="pf-v5-c-nav__subnav" aria-labelledby={self.id.clone()} hidden={!self.expanded_state}>
                    {
                        html!{
                            if let Some(sr_text) = &ctx.props().sr_text {
                                <h2 class="pf-v5-screen-reader" id={self.id.clone()}>
                                    {sr_text}
                                </h2>
                            }
                        }
                    }
                    <ul class="pf-v5-c-nav__list" role="list">
                        {ctx.props().children.clone()}
                    </ul>
                </section>
            </li>
        }
    }
}
