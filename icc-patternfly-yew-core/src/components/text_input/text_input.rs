use yew::{
    prelude::*,
};

use crate::{ValidatedOptions};


#[derive(Clone, PartialEq)]
pub enum TextInputType
{
    Text,
    Date,
    DatetimeLocal,
    Email,
    Month,
    Number,
    Password,
    Search,
    Tel,
    Time,
    Url,
}

const INPUT_TYPES: &'static [&'static str] = &[
    "text",
    "date",
    "datetime-local",
    "email",
    "month",
    "number",
    "password",
    "search",
    "tel",
    "time",
    "url",
];

pub struct TextInput
{
    link: ComponentLink<Self>,
    props: TextInputProperties,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TextInputProperties
{
    #[prop_or_default]
    pub id: String,
    /** Additional classes added to the TextInput. */
    #[prop_or_default]
    pub class_name: String,
    /** Flag to show if the input is disabled. */
    #[prop_or_default]
    pub is_disabled: bool,
    /** Flag to show if the input is read only. */
    #[prop_or_default]
    pub is_read_only: bool,
    /** Flag to show if the input is required. */
    #[prop_or_default]
    pub is_required: bool,
    /** Value to indicate if the input is modified to show that validation state.
     * If set to success, input will be modified to indicate valid state.
     * If set to error,  input will be modified to indicate error state.
     */
    #[prop_or(ValidatedOptions::Default)]
    pub validated: ValidatedOptions,
    /** A callback for when the input value changes. */
    #[prop_or_default]
    pub onchange: Callback<String>, // (value: string, event: React.FormEvent<HTMLInputElement>) => void;
    /** Type that the input accepts. */
    // #[prop_or_default]
    pub input_type: TextInputType,
    // /** Value of the input. */
    #[prop_or_default]
    pub value: String,  // | number;
    /** Aria-label. The input requires an associated id or aria-label. */
    #[prop_or_default]
    pub aria_label: String,
    // /** A reference object to attach to the input box. */
    // #[prop_or_default]
    // innerRef?: React.RefObject<any>;
    /** Trim text on left */
    #[prop_or_default]
    pub is_left_truncated: bool,
    // /** Callback function when input is focused */
    // #[prop_or_default]
    // onFocus?: (event?: any) => void;
    // /** Callback function when input is blurred (focus leaves) */
    // #[prop_or_default]
    // onBlur?: (event?: any) => void;
    // /** icon variant */
    // #[prop_or_default]
    // iconVariant?: 'calendar' | 'clock' | 'search';
    // /** Custom icon url to set as the input's background-image */
    // #[prop_or_default]
    // customIconUrl?: string;
    // /** Dimensions for the custom icon set as the input's background-size */
    // #[prop_or_default]
    // customIconDimensions?: string;
}

pub enum TextInputMsg
{
    OnInput(InputData),
}

impl Component for TextInput
{
    type Message = TextInputMsg;
    type Properties = TextInputProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self {
            link,
            props,
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
            TextInputMsg::OnInput(input_data) => {
                self.props.onchange.emit(input_data.value);
            }
        }

        false
    }

    fn view(&self) -> Html
    {
        html!{
            <input
                id=self.props.id
                // {...props}
                // onFocus={this.onFocus}
                // onBlur={this.onBlur}
                class=(
                    "pf-c-form-control",
                    if self.props.validated == ValidatedOptions::Success { "pf-m-success" } else { "" },
                    if self.props.validated == ValidatedOptions::Warning { "pf-m-warning" } else { "" },
                    // ((iconVariant && iconVariant !== 'search') || customIconUrl) && styles.modifiers.icon,
                    // iconVariant && styles.modifiers[iconVariant],
                    self.props.class_name.clone(),
                )
                oninput=self.link.callback(|input_data| TextInputMsg::OnInput(input_data))
                type=INPUT_TYPES[self.props.input_type.clone() as usize]
                value=self.props.value
                aria-invalid={self.props.validated == ValidatedOptions::Error}
                required=self.props.is_required
                disabled=self.props.is_disabled
                read_only=self.props.is_read_only
                // ref={innerRef || this.inputRef}
                // {...((customIconUrl || customIconDimensions) && { style: customIconStyle })}
            />
        }
    }
}
