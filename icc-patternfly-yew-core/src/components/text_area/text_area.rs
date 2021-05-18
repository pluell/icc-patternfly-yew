use yew::{
    prelude::*,
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
    props: TextAreaProperties,
    link: ComponentLink<TextArea>,
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
    OnChange(ChangeData),
}

impl Component for TextArea
{
    type Message = TextAreaMsg;
    type Properties = TextAreaProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self {
            text_area_ref: NodeRef::default(),
            props,
            link,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        if self.props != props
        {
            self.props = props;
            
            true
        }
        else
        {
            false
        }
    }

    /// Called everytime when messages are received
    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            TextAreaMsg::OnChange(change_data) => {
                // TODO: Calculate size for resizing

                // Call onchange property
                if let ChangeData::Value(value) = change_data
                {
                    self.props.onchange.emit(value);
                }
            },
        }

        false
    }

    fn view(&self) -> Html
    {
        html!{
            <textarea
                ref=self.text_area_ref.clone()
                class=classes!(
                    "pf-c-form-control",
                    &self.props.class_name,
                    RESIZE_ORIENTATION_STYLES[self.props.resize_orientation.clone() as usize],
                    if self.props.validated == ValidatedOptions::Success { "pf-m-success" } else { "" },
                    if self.props.validated == ValidatedOptions::Warning { "pf-m-warning" } else { "" },
                )
                onchange=self.link.callback(|data| TextAreaMsg::OnChange(data))
                // {...(typeof this.props.defaultValue !== 'string' && { value })}
                aria-invalid=(self.props.validated == ValidatedOptions::Error).to_string()
                required=self.props.is_required
                disabled=self.props.is_disabled
                readOnly=self.props.is_read_only.to_string()
                // ref={innerRef}
                // {...props}
                id=self.props.id.clone()
            />
        }
    }
}
