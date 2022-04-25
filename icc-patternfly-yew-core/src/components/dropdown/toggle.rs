
use yew::{
    prelude::*,
};

use wasm_bindgen::JsCast;
use web_sys::{Event, MouseEvent};
use gloo::events::{EventListener, EventListenerOptions};

use crate::{ButtonType, BTN_TYPES};


pub struct Toggle
{
    _key_listener_handle: EventListener,
    button_ref: NodeRef,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ToggleProperties
{
    /** HTML ID of dropdown toggle */
    #[prop_or_default]
    pub id: String,
    /** Type to put on the button */
    #[prop_or(ButtonType::Button)]
    pub toggle_type: ButtonType,
    /** Anything which can be rendered as dropdown toggle */
    #[prop_or_default]
    pub children: Children,
    /** Classes applied to root element of dropdown toggle */
    #[prop_or_default]
    pub class_name: String,
    /** Flag to indicate if menu is opened */
    #[prop_or_default]
    pub is_open: bool,
    /** Callback called when toggle is clicked */
    #[prop_or_default]
    pub ontoggle: Callback<bool>,
    /** Callback called when the Enter key is pressed */
    #[prop_or_default]
    pub onenter: Callback<()>,
    /** The menu element */
    #[prop_or_default]
    pub menu_ref: NodeRef,
    /** Forces active state */
    #[prop_or_default]
    pub is_active: bool,
    /** Disables the dropdown toggle */
    #[prop_or_default]
    pub is_disabled: bool,
    /** Display the toggle with no border or background */
    #[prop_or_default]
    pub is_plain: bool,
    /** Display the toggle with a primary button style */
    #[prop_or_default]
    pub is_primary: bool,
    /** Style the toggle as a child of a split button */
    #[prop_or_default]
    pub is_split_button: bool,
    /** Flag for aria popup */
    #[prop_or_default]
    pub aria_haspopup: String, // boolean | 'listbox' | 'menu' | 'dialog' | 'grid' | 'tree';
    /** Allows selecting toggle to select parent */
    #[prop_or_default]
    pub bubble_event: bool,
    /** Label Toggle button */
    #[prop_or_default]
    pub aria_label: String,
}

pub enum ToggleMsg
{
    OnToggle,
    OnKeyDown(KeyboardEvent),
    OnDocClick(MouseEvent),
}

impl Component for Toggle
{
    type Message = ToggleMsg;
    type Properties = ToggleProperties;

    fn create(ctx: &Context<Self>) -> Self
    {
        let document = web_sys::window()
            .expect("no global `window` exists")
            .document()
            .expect("should have a document on window");
    
        let callback = ctx.link().callback(|event| ToggleMsg::OnDocClick(event));

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
            button_ref: NodeRef::default()
        }
    }

    /// Called everytime when messages are received
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            ToggleMsg::OnToggle => {
                ctx.props().ontoggle.emit(!ctx.props().is_open);
            },
            ToggleMsg::OnKeyDown(event) => {
                if event.key() != "Tab" || ctx.props().is_open
                {
                    // if !ctx.props().bubble_event
                    // {
                    //     event.stopPropagation();
                    // }

                    // event.preventDefault();
                    
                    if (event.key() == "Tab" || event.key() == "Enter" || event.key() == " ") && ctx.props().is_open
                    {
                        ctx.props().ontoggle.emit(!ctx.props().is_open);
                        // ctx.props().ontoggle.emit(ctx.props().is_open);
                    } 
                    else if (event.key() == "Enter" || event.key() == " " || event.key() == "ArrowDown") && !ctx.props().is_open
                    {
                        ctx.props().ontoggle.emit(!ctx.props().is_open);
                        ctx.props().onenter.emit(());
                    }
                }
            },
            ToggleMsg::OnDocClick(event) => {
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
        html!{
            <button
                ref={self.button_ref.clone()}
                // {...props}
                id={ctx.props().id.clone()}
                class={classes!(
                    if ctx.props().is_split_button { "pf-c-dropdown__toggle-button" } else { "pf-c-dropdown__toggle" },
                    if ctx.props().is_active { "pf-m-active" } else { "" },
                    if ctx.props().is_plain { "pf-m-plain" } else { "" },
                    if ctx.props().is_primary { "pf-m-primary" } else { "" },
                    ctx.props().class_name.clone(),
                )}
                type={BTN_TYPES[ctx.props().toggle_type.clone() as usize]}
                onclick={ctx.link().callback(|_| ToggleMsg::OnToggle)}
                aria-expanded={ctx.props().is_open.to_string()}
                aria-haspopup={ctx.props().aria_haspopup.clone()}
                onkeydown={ctx.link().callback(|event| ToggleMsg::OnKeyDown(event))}
                disabled={ctx.props().is_disabled}
                aria-label={ctx.props().aria_label.clone()}
            >
                { ctx.props().children.clone() }
            </button>
        }
    }
}
