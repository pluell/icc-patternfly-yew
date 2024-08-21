use yew::prelude::*;

use wasm_bindgen::JsCast;
use web_sys::{Event, MouseEvent};
use gloo::events::{EventListener, EventListenerOptions};

use crate::{ButtonType, BTN_TYPES};

use super::SelectVariant;


pub struct SelectToggle
{
    _key_listener_handle: EventListener,
    button_ref: NodeRef,
}

pub enum SelectToggleMsg
{
    OnToggle,
    OnKeyDown(KeyboardEvent),
    OnDocClick(MouseEvent),
}

#[derive(Clone, PartialEq, Properties)]
pub struct SelectToggleProperties
{
    /** HTML ID of dropdown toggle */
    #[prop_or_default]
    pub id: String,
    /** Anything which can be rendered as dropdown toggle */
    #[prop_or_default]
    pub children: Children,
    /** Classes applied to root element of dropdown toggle */
    #[prop_or_default]
    pub class_name: String,
    /** Flag to indicate if select is open */
    #[prop_or_default]
    pub is_open: bool,
    /** Callback called when toggle is clicked */
    #[prop_or_default]
    pub ontoggle: Callback<bool>,
    // /** Callback for toggle open on keyboard entry */
    // onEnter?: () => void;
    // /** Callback for toggle close */
    // onClose?: () => void;
    // /** Internal callback for toggle keyboard navigation */
    // handleTypeaheadKeys?: (position: string) => void;
    // /** Element which wraps toggle */
    // parentRef: React.RefObject<HTMLDivElement>;
    /** The menu element */
    #[prop_or_default]
    pub menu_ref: NodeRef,
    /** Forces active state */
    #[prop_or_default]
    pub is_active: bool,
    /** Display the toggle with no border or background */
    #[prop_or_default]
    pub is_plain: bool,
    /** Flag indicating if select is disabled */
    #[prop_or_default]
    pub is_disabled: bool,
    /** Type of the toggle button, defaults to 'button' */
    #[prop_or(ButtonType::Button)]
    pub toggle_type: ButtonType,
    /** Id of label for the Select aria-labelledby */
    #[prop_or_default]
    pub aria_labelledby: String,
    /** Label for toggle of select variants */
    #[prop_or_default]
    pub aria_label: String,
    /** Flag for variant, determines toggle rules and interaction */
    #[prop_or(SelectVariant::Single)]
    pub variant: SelectVariant,
    /** Flag indicating if select toggle has an clear button */
    #[prop_or_default]
    pub has_clear_button: bool,
    // /** Internal callback for handling focus when typeahead toggle button clicked. */
    // onClickTypeaheadToggleButton?: () => void;
}

impl Component for SelectToggle
{
    type Message = SelectToggleMsg;
    type Properties = SelectToggleProperties;

    fn create(ctx: &Context<Self>) -> Self
    {
        let document = web_sys::window()
            .expect("no global `window` exists")
            .document()
            .expect("should have a document on window");
    
        let callback = ctx.link().callback(|event| SelectToggleMsg::OnDocClick(event));

        let listener = move |event: &Event| {
            // Try to convert event to MouseEvent
            if let Some(event) = event.dyn_ref::<MouseEvent>()
            {
                // Handle the event
                callback.emit(event.clone());
            }
        };

        let event_options = EventListenerOptions::enable_prevent_default();
        
        // Listen for mousedown on the whole document to handle toggle when user
        // does not click on the toggle
        let key_listener_handle = EventListener::new_with_options(
            document.as_ref(),
            "mousedown",
            event_options,
            listener,
        );

        Self {
            _key_listener_handle: key_listener_handle,
            button_ref: NodeRef::default(),
        }
    }

    /// Called everytime when messages are received
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            SelectToggleMsg::OnToggle => {
                ctx.props().ontoggle.emit(!ctx.props().is_open);
            },
            SelectToggleMsg::OnKeyDown(event) => {
                if event.key() == "Enter"
                {
                    ctx.props().ontoggle.emit(!ctx.props().is_open);
                }
            },
            SelectToggleMsg::OnDocClick(event) => {
                if let Some(event_node) = event.target().unwrap().dyn_ref::<web_sys::Node>()
                {
                    if let Some(button_node) = self.button_ref.get()
                    {
                        if let Some(menu_ref) = ctx.props().menu_ref.get()
                        {
                            if ctx.props().is_open
                                && !button_node.contains(Some(&event_node))
                                && !menu_ref.contains(Some(&event_node))
                            {
                                ctx.props().ontoggle.emit(false);
                            }
                        }
                    }
                }
            },
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let is_typeahead = ctx.props().variant == SelectVariant::TypeAhead 
                            || ctx.props().variant == SelectVariant::TypeAheadMulti 
                            || ctx.props().has_clear_button;

        if !is_typeahead
        {
            html!{
                <button 
                    ref={self.button_ref.clone()}
                    id={ctx.props().id.clone()}
                    aria-labelledby={ctx.props().aria_labelledby.clone()}
                    aria-expanded={ctx.props().is_open.to_string()}
                    // aria-haspopup: (variant !== SelectVariant.checkbox && 'listbox') || null
                    type={BTN_TYPES[ctx.props().toggle_type.clone() as usize]}
                    class={classes!(
                        "pf-v5-c-select__toggle",
                        if ctx.props().is_disabled { "pf-m-disabled" } else { "" },
                        if ctx.props().is_plain { "pf-m-plain" } else { "" },
                        if ctx.props().is_active { "pf-m-active" } else { "" },
                        ctx.props().class_name.clone()
                    )}
                    disabled={ctx.props().is_disabled}
                    onclick={ctx.link().callback(|_| SelectToggleMsg::OnToggle)}
                    onkeydown={ctx.link().callback(|event| SelectToggleMsg::OnKeyDown(event))}
                >
                    { ctx.props().children.clone() }
                    <span class="pf-v5-c-select__toggle-arrow">
                        <i class="fas fa-caret-down" aria-hidden="true"></i>
                    </span>
                </button>
            }
        }
        else
        {
            // TODO: render type ahead toggle
            html!{}
        }
    }
}
