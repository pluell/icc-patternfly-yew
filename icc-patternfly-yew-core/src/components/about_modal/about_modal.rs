use yew::{
    prelude::*,
};

use wasm_bindgen::JsCast;
use web_sys::{Event};
use gloo::events::{EventListener, EventListenerOptions};

use web_sys::{HtmlElement};

use crate::{KeyCodes};

use super::{AboutModalContainer};


pub struct AboutModal
{
    html_body: HtmlElement,
    container_node: NodeRef,
    key_listener_handle: Option<EventListener>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct AboutModalProps
{
    /** Content rendered inside the about modal  */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the about modal  */
    #[prop_or_default]
    pub class_name: String,
    /** Flag to show the about modal  */
    #[prop_or_default]
    pub is_open: bool,
    /** A callback for when the close button is clicked  */
    #[prop_or_default]
    pub onclose: Callback<()>,
    /** Product name  */
    #[prop_or_default]
    pub product_name: Option<String>,
    /** Trademark information  */
    #[prop_or_default]
    pub trademark: String,
    /** The URL of the image for the brand  */
    pub brand_image_src: String,
    /** The alternate text of the brand image  */
    pub brand_image_alt: String,
    /** The URL of the image for the background  */
    #[prop_or_default]
    pub background_image_src: Option<String>,
    /** Prevents the about modal from rendering content inside a container; allows for more flexible layouts  */
    #[prop_or_default]
    pub no_about_modal_box_content_container: bool,
    /** The parent container to append the modal to. Defaults to document.body */
    #[prop_or_default]
    pub append_to: Option<HtmlElement>,
    /** Set aria label to the close button */
    #[prop_or_default]
    pub close_button_aria_label: Option<String>,
}

pub enum AboutModalMsg
{
    OnKeyDown(KeyboardEvent),
}

impl Component for AboutModal
{
    type Message = AboutModalMsg;
    type Properties = AboutModalProps;

    fn create(_: &Context<Self>) -> Self
    {
        // Get the body component as a default target container
        let html_body = web_sys::window()
            .expect("No global `window` exists!")
            .document()
            .expect("Unable to load the Html document on window!")
            .body()
            .expect("Unable to load the HTML body element!");

        Self {
            html_body,
            container_node: NodeRef::default(),
            key_listener_handle: None,
        }
    }

    /// Called everytime when messages are received
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            AboutModalMsg::OnKeyDown(event) => {
                if event.key_code() == KeyCodes::EscapeKey as u32 && ctx.props().is_open
                {
                    ctx.props().onclose.emit(());
                }
            },
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let id = 1;
        let aria_labelled_by = format!("pf-about-modal-title-{}", id);
        let aria_described_by = format!("pf-about-modal-content-{}", id);

        html!{
            <div ref={self.container_node.clone()}>
                <AboutModalContainer
                    about_modal_box_header_id={aria_labelled_by}
                    about_modal_box_content_id={aria_described_by}
                    // {...props}
                    children={ctx.props().children.clone()}
                    class_name={ctx.props().class_name.clone()}
                    is_open={ctx.props().is_open}
                    onclose={ctx.props().onclose.clone()}
                    product_name={ctx.props().product_name.clone()}
                    trademark={ctx.props().trademark.clone()}
                    brand_image_src={ctx.props().brand_image_src.clone()}
                    brand_image_alt={ctx.props().brand_image_alt.clone()}
                />
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool)
    {
        // Get target element to attach the AboutModal to
        let target = if let Some(append_to) = &ctx.props().append_to {
            append_to.clone()
        } else {
            self.html_body.clone()
        };

        // Handle backdrop class for target element
        let mut target_classes = target.class_name();

        if !target_classes.contains("pf-c-backdrop__open")
        {
            if ctx.props().is_open
            {
                target_classes += " pf-c-backdrop__open";
            }
        }
        else
        {
            // Remove the backdrop open class if the AboutModal is closed now
            if !ctx.props().is_open
            {
                let class_len = " pf-c-backdrop__open".len();
                let class_offset = target_classes.find(" pf-c-backdrop__open").unwrap_or(target_classes.len());

                target_classes.replace_range(class_offset..class_len, "");
            }
        }

        target.set_class_name(&target_classes);

        // Set up the key handler event on the target for the user hitting "esc"
        if self.key_listener_handle.is_none()
        {
            let callback = ctx.link().callback(|event| AboutModalMsg::OnKeyDown(event));

            let listener = move |event: &Event| {
                // Try to convert event to KeyboardEvent
                if let Some(event) = event.dyn_ref::<KeyboardEvent>()
                {
                    // Handle the event
                    callback.emit(event.clone());
                }
            };

            let event_options = EventListenerOptions::enable_prevent_default();
            
            // Listen for keydown on the target container
            let key_listener_handle = EventListener::new_with_options(
                target.as_ref(),
                "keydown",
                event_options,
                listener,
            );

            self.key_listener_handle = Some(key_listener_handle);
        }

        // Move the AboutModal component to the target container
        target.append_child(&self.container_node.get().unwrap())
            .expect("Unable to attach AboutModal to target element");
    }
}
