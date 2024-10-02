use yew::prelude::*;

use super::{NavContext, NavContextOnSelectProps};
use crate::styles::Theme;


pub type NavSelectClickHandler = Callback<NavSelectedItem>;

#[derive(Clone, PartialEq)]
pub struct NavSelectedItem
{
    pub group_id: String,
    pub item_id: String,
    pub to: String,
}


#[derive(Clone, PartialEq)]
pub enum NavVariants
{
    Default,
    Horizontal,
    HorizontalSubnav,
}

impl NavVariants
{
    fn get_class(&self) -> &'static str
    {
        match self
        {
            Self::Default => "",
            Self::Horizontal => "pf-m-horizontal",
            Self::HorizontalSubnav => "pf-m-horizontal-subnav"
        }
    }

    fn is_horizontal(&self) -> bool
    {
        self == &Self::Horizontal || self == &Self::HorizontalSubnav
    }
}


pub struct Nav
{
    is_scrollable: bool,
    nav_ref: NodeRef,
}

#[derive(Clone, PartialEq, Properties)]
pub struct NavProps
{
    /** Anything that can be rendered inside of the nav */
    #[prop_or_default]
    pub children: Html,
    /** Additional classes added to the container */
    #[prop_or_default]
    pub classes: Classes,
    /** Callback for updating when item selection changes */
    #[prop_or_default]
    pub onselect: Option<Callback<NavSelectedItem>>,
    /** Callback for when a list is expanded or collapsed */
    #[prop_or_default]
    pub ontoggle: Option<Callback<(String, bool)>>,
    /** Accessible label for the nav when there are multiple navs on the page */
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,
    /** Indicates which theme color to use */
    #[prop_or(Theme::Dark)]
    pub theme: Theme,
    /** For horizontal navs */
    #[prop_or(NavVariants::Default)]
    pub variant: NavVariants,
    /** Value to overwrite the randomly generated data-ouia-component-id.*/
    #[prop_or_default]
    pub ouia_id: Option<AttrValue>,
    /** Set the value of data-ouia-safe. Only set to true when the component is in a static state, i.e. no animations are occurring. At all other times, this value must be false. */
    #[prop_or_default]
    pub ouia_safe: bool,
}

pub enum NavMessage
{
    UpdateScrollable(bool),
    OnSelect(NavContextOnSelectProps),
}

impl Component for Nav
{
    type Message = NavMessage;
    type Properties = NavProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self {
            is_scrollable: false,
            nav_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg : Self::Message) -> bool
    {
        match msg
        {
            Self::Message::UpdateScrollable(is_scrollable) => {
                self.is_scrollable = is_scrollable;

                true
            }
            Self::Message::OnSelect(select_props) => {
                if select_props.prevent_default {
                    select_props.event.prevent_default();
                }

                if let Some(onselect) = &ctx.props().onselect {
                    onselect.emit(select_props.selected_item.clone());
                }

                if let Some(onclick) = select_props.onclick {
                    onclick.emit(select_props.selected_item);
                }

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let aria_label = if let Some(aria_label) = &ctx.props().aria_label {
            aria_label.to_string()
        } else {
            if ctx.props().variant == NavVariants::HorizontalSubnav {
                "Local".to_string()
            } else {
                "Global".to_string()
            }
        };

        html!{
            <ContextProvider<NavContext> context={NavContext{
                    onselect: ctx.link().callback(Self::Message::OnSelect),
                    ontoggle: ctx.props().ontoggle.clone(),
                    update_is_scrollable: ctx.link().callback(Self::Message::UpdateScrollable),
                    is_horizontal: ctx.props().variant.is_horizontal(),
                    flyout_ref: None, //this.state.flyoutRef,
                    // setFlyoutRef: (flyoutRef) => this.setState({ flyoutRef }),
                    nav_ref: self.nav_ref.clone(),
                }}
            >
                <nav
                    class={classes!(
                        "pf-v5-c-nav",
                        ctx.props().theme.get_class(),
                        ctx.props().variant.get_class(),
                        if self.is_scrollable {"pf-m-scrollable"} else {""},
                        ctx.props().classes.clone(),
                    )}
                    aria-label={aria_label}
                    ref={self.nav_ref.clone()}
                    // {...getOUIAProps(Nav.displayName, ouiaId !== undefined ? ouiaId : this.state.ouiaStateId, ouiaSafe)}
                    // {...props}
                >
                    {ctx.props().children.clone()}
                </nav>
            </ContextProvider<NavContext>>
        }
    }
}
