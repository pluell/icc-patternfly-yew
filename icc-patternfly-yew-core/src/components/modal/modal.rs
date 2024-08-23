use yew::prelude::*;

use wasm_bindgen::JsCast;
use web_sys::Event;
use gloo::events::{EventListener, EventListenerOptions};

use web_sys::HtmlElement;

use crate::KeyCodes;
use super::{ModalVariants, ModalContent, ModalTitleIconVariants};


pub struct Modal
{
    html_body: HtmlElement,
    container_node: NodeRef,
    key_listener_handle: Option<EventListener>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ModalProperties
{
    /** Content rendered inside the Modal. */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the Modal */
    #[prop_or_default]
    pub class_name: String,
    /** Flag to show the modal */
    #[prop_or_default]
    pub is_open: bool,
    /** Complex header (more than just text), supersedes title for header content */
    #[prop_or_default]
    pub header: Option<Html>,
    /** Optional help section for the Modal Header */
    #[prop_or_default]
    pub help: Option<Html>,
    /** Simple text content of the Modal Header, also used for aria-label on the body */
    #[prop_or_default]
    pub title: String,
    /** Optional alert icon (or other) to show before the title of the Modal Header
     * When the predefined alert types are used the default styling
     * will be automatically applied */
     #[prop_or_default]
    pub title_icon_variant: Option<ModalTitleIconVariants>,
    /** Optional title label text for screen readers */
    #[prop_or_default]
    pub title_label: String,
    /** Id to use for Modal Box label */
    #[prop_or_default]
    pub aria_labelledby: Option<String>,
    /** Accessible descriptor of modal */
    #[prop_or_default]
    pub aria_label: String,
    /** Id to use for Modal Box descriptor */
    #[prop_or_default]
    pub aria_describedby: String,
    /** Flag to show the close button in the header area of the modal */
    #[prop_or(true)]
    pub show_close: bool,
    /** Custom footer */
    #[prop_or_default]
    pub footer: Option<Html>,
    // /** Action buttons to add to the standard Modal Footer, ignored if `footer` is given */
    #[prop_or_default]
    pub actions: Option<Html>,
    /** A callback for when the close button is clicked */
    #[prop_or_default]
    pub onclose: Callback<()>,
    /** Default width of the Modal. */
    #[prop_or_default]
    pub width: String,
    /** The parent container to append the modal to. Defaults to document.body */
    #[prop_or_default]
    pub append_to: Option<HtmlElement>,
    /** Flag to disable focus trap */
    #[prop_or_default]
    pub disable_focus_trap: bool,
    /** Description of the modal */
    #[prop_or_default]
    pub description: Option<Html>,
    /** Variant of the modal */
    #[prop_or(ModalVariants::Default)]
    pub variant: ModalVariants,
    /** Alternate position of the modal */
    #[prop_or_default]
    pub position_top: bool,
    /** Offset from alternate position. Can be any valid CSS length/percentage */
    #[prop_or_default]
    pub position_offset: String,
    /** Flag indicating if modal content should be placed in a modal box body wrapper */
    #[prop_or_default]
    pub has_no_body_wrapper: bool,
    /** An ID to use for the ModalBox container */
    #[prop_or_default]
    pub id: String,
    /** Modal handles pressing of the Escape key and closes the modal. If you want to handle this yourself you can use this callback function */
    #[prop_or_default]
    pub onescapepress: Option<Callback<KeyboardEvent>>,
}

pub enum ModalMsg
{
    OnKeyDown(KeyboardEvent),
}

impl Component for Modal
{
    type Message = ModalMsg;
    type Properties = ModalProperties;

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
            ModalMsg::OnKeyDown(event) => {
                if event.key_code() == KeyCodes::EscapeKey as u32 && ctx.props().is_open
                {
                    if let Some(onescapepress) = &ctx.props().onescapepress
                    {
                        onescapepress.emit(event);
                    }
                    else
                    {
                        ctx.props().onclose.emit(());
                    }
                }
            },
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div ref={self.container_node.clone()}>
                <ModalContent
                    // {...props}
                    children={ctx.props().children.clone()}
                    is_open={ctx.props().is_open}
                    header={ctx.props().header.clone()}
                    show_close={ctx.props().show_close}
                    onclose={ctx.props().onclose.clone()}
                    box_id={ctx.props().id.clone()}
                    // labelId={this.labelId}
                    // descriptorId={this.descriptorId}
                    title={ctx.props().title.clone()}
                    title_icon_variant={ctx.props().title_icon_variant.clone()}
                    title_label={ctx.props().title_label.clone()}
                    aria_label={ctx.props().aria_label.clone()}
                    aria_describedby={ctx.props().aria_describedby.clone()}
                    aria_labelledby={ctx.props().aria_labelledby.clone()}
                    // ouiaId={ouiaId !== undefined ? ouiaId : this.state.ouiaStateId}
                    // ouiaSafe={ouiaSafe}
                    footer={ctx.props().footer.clone()}
                    actions={ctx.props().actions.clone()}
                    width={ctx.props().width.clone()}
                    description={ctx.props().description.clone()}
                    variant={ctx.props().variant.clone()}
                    has_no_body_wrapper={ctx.props().has_no_body_wrapper}
                    position_top={ctx.props().position_top}
                    position_offset={ctx.props().position_offset.clone()}
                />
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool)
    {
        // Get target element to attach the modal dialog to
        let target = if let Some(append_to) = &ctx.props().append_to {
            append_to.clone()
        } else {
            self.html_body.clone()
        };

        // Handle backdrop class for target element
        let mut target_classes = target.class_name();

        if !target_classes.contains("pf-v5-c-backdrop__open")
        {
            if ctx.props().is_open
            {
                target_classes += " pf-v5-c-backdrop__open";
            }
        }
        else
        {
            // Remove the backdrop open class if the modal is closed now
            if !ctx.props().is_open
            {
                let class_len = " pf-v5-c-backdrop__open".len();
                let class_offset = target_classes.find(" pf-v5-c-backdrop__open").unwrap_or(target_classes.len());

                target_classes.replace_range(class_offset..class_len, "");
            }
        }

        target.set_class_name(&target_classes);

        // Set up the key handler event on the target for the user hitting "esc"
        if self.key_listener_handle.is_none()
        {
            let callback = ctx.link().callback(|event| ModalMsg::OnKeyDown(event));

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

        // Move the Modal component to the target container
        target.append_child(&self.container_node.get().unwrap())
            .expect("Unable to attach modal dialog to target element");
    }
}
