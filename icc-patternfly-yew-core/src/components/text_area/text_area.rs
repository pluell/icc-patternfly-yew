
use web_sys::HtmlInputElement;
use yew::{
    prelude::*,
    events::Event,
    html,
    Component, Context, Html, TargetCast,
};

use crate::{ValidatedOptions};


#[derive(Clone, PartialEq)]
pub enum TextAreResizeOrientation
{
    Horizontal,
    Vertical,
    Both,
}

const RESIZE_ORIENTATION_STYLES: &'static [&'static str] = &[
    "pf-m-resize-horizontal",
    "pf-m-resize-vertical",
    "",
];

pub struct TextArea
{
    text_area_ref: NodeRef,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TextAreaProperties
{
    /** Additional classes added to the TextArea. */
    #[prop_or_default]
    pub class_name: String,
    /** Flag to show if the TextArea is required. */
    #[prop_or_default]
    pub is_required: bool,
    /** Flag to show if the TextArea is disabled. */
    #[prop_or_default]
    pub is_disabled: bool,
    /** Flag to show if the TextArea is read only. */
    #[prop_or_default]
    pub is_read_only: bool,
    /** Flag to modify height based on contents. */
    #[prop_or_default]
    pub auto_resize: bool,
    /** Value to indicate if the textarea is modified to show that validation state.
     * If set to success, textarea will be modified to indicate valid state.
     * If set to error, textarea will be modified to indicate error state.
     */
    #[prop_or(ValidatedOptions::Default)]
    pub validated: ValidatedOptions,    //'success' | 'warning' | 'error' | 'default';
    /** Value of the TextArea. */
    #[prop_or_default]
    pub value: String,
    /** A callback for when the TextArea value changes. */
    #[prop_or_default]
    pub onchange: Callback<String>, //  (value: string, event: React.ChangeEvent<HTMLTextAreaElement>) => void;
    /** Sets the orientation to limit the resize to */
    #[prop_or(TextAreResizeOrientation::Both)]
    pub resize_orientation: TextAreResizeOrientation,
    /** Custom flag to show that the TextArea requires an associated id or aria-label. */
    #[prop_or_default]
    pub aria_label: String,
    // /** A reference object to attach to the textarea. */
    // #[prop_or_default]
    // innerRef: React.RefObject<any>;

    /** textarea props */
    #[prop_or_default]
    pub id: String,
}

pub enum TextAreaMsg
{
    OnChange(String),
}

impl Component for TextArea
{
    type Message = TextAreaMsg;
    type Properties = TextAreaProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self {
            text_area_ref: NodeRef::default(),
        }
    }

    /// Called everytime when messages are received
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            TextAreaMsg::OnChange(value) => {
                // TODO: Calculate size for resizing

                // Call onchange property
                ctx.props().onchange.emit(value);
            },
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let onchange = ctx.link().batch_callback(|e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            input.map(|input| TextAreaMsg::OnChange(input.value()))
        });

        html!{
            <textarea
                ref={self.text_area_ref.clone()}
                class={classes!(
                    "pf-c-form-control",
                    &ctx.props().class_name,
                    RESIZE_ORIENTATION_STYLES[ctx.props().resize_orientation.clone() as usize],
                    if ctx.props().validated == ValidatedOptions::Success { "pf-m-success" } else { "" },
                    if ctx.props().validated == ValidatedOptions::Warning { "pf-m-warning" } else { "" },
                )}
                {onchange}
                // {...(typeof this.props.defaultValue !== 'string' && { value })}
                aria-invalid={(ctx.props().validated == ValidatedOptions::Error).to_string()}
                required={ctx.props().is_required}
                disabled={ctx.props().is_disabled}
                readOnly={ctx.props().is_read_only.to_string()}
                // ref={innerRef}
                // {...props}
                id={ctx.props().id.clone()}
            />
        }
    }
}
