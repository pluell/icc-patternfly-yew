use yew::prelude::*;

use crate::{Button, ButtonVariant, Checkbox, Radio};
use super::{CardContext, CardActions, CardHeaderMain, CardSelectableActions};


#[derive(Clone, PartialEq, Default)]
pub struct CardHeaderActionsObject
{
    /** Actions of the card header */
    pub actions: Html,
    /** Flag indicating that the actions have no offset */
    pub has_no_offset: bool,
    /** Additional classes added to the actions wrapper */
    pub classes: Classes,
}

#[derive(Clone, PartialEq)]
pub enum CardHeaderSelectableActionsObjectVariant
{
    Single,
    Multiple
}
  
#[derive(Clone, PartialEq, Default)]
pub struct CardHeaderSelectableActionsObject
{
    /** Determines the type of input to be used for a selectable card. */
    pub variant: Option<CardHeaderSelectableActionsObjectVariant>,
    /** Flag indicating that the actions have no offset */
    pub has_no_offset: bool,
    /** Additional classes added to the selectable actions wrapper */
    pub classes: Classes,
    /** ID passed to the selectable or clickable input */
    pub selectable_action_id: AttrValue,
    /** Adds an accessible label to the selectable or clickable input */
    pub selectable_action_aria_label: Option<AttrValue>,
    /** Adds an accessible label to the selectable or clickable input by passing in a
     * space separated list of id's.
     */
    pub selectable_action_aria_labelledby: Option<AttrValue>,
    /** Callback for when a selectable card input changes */
    pub onchange: Callback<bool>, // (event: React.FormEvent<HTMLInputElement>, checked: boolean) => void;
    /** Action to call when clickable card is clicked */
    pub on_click_action: Option<Callback<()>>, // (event: React.FormEvent<HTMLInputElement> | React.MouseEvent) => void;
    /** Link to navigate to when clickable card is clicked */
    pub to: Option<String>,
    /** Flag to indicate whether a clickable card's link should open in a new tab/window. */
    pub is_external_link: bool,
    /** Name for the input element of a clickable or selectable card. */
    pub name: Option<AttrValue>
}

pub struct CardHeader
{
    context: CardContext,
    _context_listener: ContextHandle<CardContext>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct CardHeaderProperties
{
    /** Content rendered inside the CardHeader */
    #[prop_or_default]
    pub children: Option<Html>,
    /** Additional classes added to the CardHeader */
    #[prop_or_default]
    pub classes: Classes,
    /** Actions of the card header */
    #[prop_or_default]
    pub actions: Option<CardHeaderActionsObject>,
    /** Selectable actions of the card header */
    #[prop_or_default]
    pub selectable_actions: Option<CardHeaderSelectableActionsObject>,
    /** ID of the card header. */
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /** Callback expandable card */
    #[prop_or_default]
    pub onexpand: Option<Callback<String>>,
    /** Additional props for expandable toggle button */
    #[prop_or_default]
    pub toggle_button_props: String,
    /** Whether to right-align expandable toggle button */
    #[prop_or_default]
    pub is_toggle_right_aligned: bool,
}

pub enum CardHeaderMsg
{
    Context(CardContext),
    OnClickToggle,
    OnClickAction,
}

impl Component for CardHeader
{
    type Message = CardHeaderMsg;
    type Properties = CardHeaderProperties;

    fn create(ctx: &Context<Self>) -> Self
    {
        let (context, _context_listener) = ctx
            .link()
            .context(ctx.link().callback(Self::Message::Context))
            .expect("No Message Context Provided");

        Self {
            context,
            _context_listener,
        }
    }

    /// Called everytime when messages are received
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            Self::Message::Context(context) => {
                self.context = context;
            }
            Self::Message::OnClickToggle => {
                if let Some(onexpand) = &ctx.props().onexpand
                {
                    onexpand.emit(self.context.card_id.to_string());
                }
            }
            Self::Message::OnClickAction => {
                if let Some(selectable_actions) = &ctx.props().selectable_actions {
                    if let Some(on_click_action) = &selectable_actions.on_click_action {
                        on_click_action.emit(());
                    } else if let Some(_to) = &selectable_actions.to {
                        // TODO: Open a new window
                        // window.open(selectableActions.to, selectableActions.isExternalLink ? '_blank' : '_self');
                    }
                }
            }
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let show_selectable_action = ctx.props().selectable_actions.is_some() && 
            (self.context.is_clickable || self.context.is_selectable);

        let has_no_offset= if let Some(actions) = &ctx.props().actions {
            actions.has_no_offset
        } else if let Some(selectable_actions) = &ctx.props().selectable_actions {
            selectable_actions.has_no_offset
        } else {
            false
        };

        html!{
            <div 
                class={classes!(
                    "pf-v5-c-card__header",
                    if ctx.props().is_toggle_right_aligned {"pf-m-toggle-right"} else {""},
                    ctx.props().classes.clone(),
                )}
                id={ctx.props().id.clone()}
                // {...props}
            >
            {
                if ctx.props().onexpand.is_some() && !ctx.props().is_toggle_right_aligned {
                    self.view_card_header_toggle(ctx)
                } else {
                    html!{}
                }
            }
            {
                html!{
                    if ctx.props().actions.is_some() || show_selectable_action {
                        <CardActions
                            classes={if let Some(actions) = &ctx.props().actions {actions.classes.clone()} else {classes!{}}}
                            {has_no_offset}
                        >
                        {
                            if let Some(actions) = &ctx.props().actions {
                                actions.actions.clone()
                            } else{
                                html!{}
                            }
                        }
                        {
                            if let Some(selectable_actions) = &ctx.props().selectable_actions {
                                if self.context.is_clickable || self.context.is_selectable {
                                    self.view_card_header_selectable_actions(ctx, selectable_actions)
                                } else {
                                    html!{}
                                }
                            } else {
                                html!{}
                            }
                        }
                        </CardActions>
                    }
                }
            }
            {
                html!{
                    if let Some(children) = &ctx.props().children {
                        <CardHeaderMain>{children.clone()}</CardHeaderMain>
                    }
                }
            }
            {
                if ctx.props().onexpand.is_some() && ctx.props().is_toggle_right_aligned {
                    self.view_card_header_toggle(ctx)
                } else {
                    html!{}
                }
            }
          </div>
        }
    }
}

impl CardHeader
{
    fn view_card_header_toggle(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div class="pf-v5-c-card__header-toggle">
                <Button
                    variant={ButtonVariant::Plain}
                    // type="button"
                    onclick={ctx.link().callback(|_| CardHeaderMsg::OnClickToggle)}
                    // {...toggleButtonProps}
                >
                    <span class="pf-v5-c-card__header-toggle-icon">
                        {icc_patternfly_yew_icons::angle_right_icon!{}}
                    </span>
                </Button>
            </div>
        }
    }

    fn view_card_header_selectable_actions(&self, ctx: &Context<Self>, selectable_actions: &CardHeaderSelectableActionsObject) -> Html
    {
        let is_selectable_action_single = selectable_actions.variant
                .as_ref()
                .map(|variant| variant == &CardHeaderSelectableActionsObjectVariant::Single)
                .unwrap_or(false);

        // Props..
        let classes="pf-m-standalone";
        let input_classes=if self.context.is_clickable && !self.context.is_selectable
            {"pf-v5-screen-reader"} else {""};
        let label = html!{<></>};
        let aria_label = selectable_actions.selectable_action_aria_label.clone();
        let aria_labelledby = selectable_actions.selectable_action_aria_labelledby.clone();
        let id = selectable_actions.selectable_action_id.clone();
        let name = selectable_actions.name.clone().unwrap_or(AttrValue::from(""));
        let is_disabled = self.context.is_disabled;

        let onclick = if self.context.is_clickable && !self.context.is_selectable {
            ctx.link().callback(|_| CardHeaderMsg::OnClickAction)
        } else {
            Callback::default()
        };

        let onchange = if self.context.is_selectable {selectable_actions.onchange.clone()} else {Callback::default()};

        let is_checked = if self.context.is_selectable {
            Some(self.context.is_clicked)
        } else if self.context.is_clickable && !self.context.is_selectable {
            Some(self.context.is_clicked)
        } else {
            None
        };

        html!{
            <CardSelectableActions
                classes={selectable_actions.classes.clone()}
            >
            {
                if is_selectable_action_single || 
                    (self.context.is_clickable && !self.context.is_selectable)
                {
                    html!{
                        <Radio
                            {classes}
                            {input_classes}
                            {label}
                            {aria_label}
                            {aria_labelledby}
                            {id}
                            {name}
                            {is_disabled}
                            {onclick}
                            {onchange}
                            {is_checked}
                        />
                    }
                } else {
                    html!{
                        <Checkbox
                            {classes}
                            {input_classes}
                            {label}
                            {aria_label}
                            {aria_labelledby}
                            {id}
                            {name}
                            {is_disabled}
                            {onclick}
                            {onchange}
                            {is_checked}
                        />
                    }
                }
            }
            </CardSelectableActions>
        }
    }
}
