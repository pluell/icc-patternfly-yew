use yew::{
    prelude::*,
};

use crate::{
    Button, ButtonVariant, 
    InputGroup, 
    Spinner, SpinnerSize,
    TextArea, TextAreResizeOrientation,
    TextInput,
    ValidatedOptions
};


pub struct FileUploadField
{
    props: FileUploadFieldProperties,
    link: ComponentLink<FileUploadField>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct FileUploadFieldProperties
{
    /** Unique id for the TextArea, also used to generate ids for accessible labels */
    pub id: String,
    // /** What type of file. Determines what is is expected by `value`
    //  * (a string for 'text' and 'dataURL', or a File object otherwise). */
    // #[prop_or_default]
    // pub type?: 'text' | 'dataURL';
    /** Value of the file's contents
     * (string if text file, File object otherwise) */
    #[prop_or_default]
    pub value: String,  //string | File;
    /** Value to be shown in the read-only filename field. */
    #[prop_or_default]
    pub filename: String,
    /** A callback for when the TextArea value changes. */
    #[prop_or_default]
    pub onchange: Callback<(String, String)>,
    //     value: string,
    //     filename: string,
    //     event:
    //     | React.ChangeEvent<HTMLTextAreaElement> // User typed in the TextArea
    //     | React.MouseEvent<HTMLButtonElement, MouseEvent> // User clicked Clear button
    // ) => void;
    /** Additional classes added to the FileUploadField container element. */
    #[prop_or_default]
    pub class_name: String,
    /** Flag to show if the field is disabled. */
    #[prop_or_default]
    pub is_disabled: bool,
    /** Flag to show if the field is read only. */
    #[prop_or_default]
    pub is_read_only: bool,
    /** Flag to show if a file is being loaded. */
    #[prop_or_default]
    pub is_loading: bool,
    /** Aria-valuetext for the loading spinner */
    #[prop_or_default]
    pub spinner_aria_value_text: String,
    /** Flag to show if the field is required. */
    #[prop_or_default]
    pub is_required: bool,
    /** Value to indicate if the field is modified to show that validation state.
     * If set to success, field will be modified to indicate valid state.
     * If set to error,  field will be modified to indicate error state.
     */
    #[prop_or(ValidatedOptions::Default)]
    pub validated: ValidatedOptions,
    /** Aria-label for the TextArea. */
    #[prop_or_default]
    pub aria_label: String,
    /** Placeholder string to display in the empty filename field */
    #[prop_or(String::from("Drag a file here or browse to upload"))]
    pub filename_placeholder: String,
    /** Aria-label for the read-only filename field */
    #[prop_or_default]
    pub filename_aria_label: String,
    /** Text for the Browse button */
    #[prop_or(String::from("Browse..."))]
    pub browse_button_text: String,
    /** Text for the Clear button */
    #[prop_or(String::from("Clear"))]
    pub clear_button_text: String,
    /** Flag to disable the Clear button */
    #[prop_or_default]
    pub is_clear_button_disabled: bool,
    /** Flag to hide the built-in preview of the file (where available).
     * If true, you can use children to render an alternate preview. */
    #[prop_or_default]
    pub hide_default_preview: bool,
    /** Flag to allow editing of a text file's contents after it is selected from disk */
    #[prop_or_default]
    pub allow_editing_uploaded_text: bool,
    /** Additional children to render after (or instead of) the file preview. */
    #[prop_or_default]
    pub children: Children,

    // Props available in FileUploadField but not FileUpload:

    /** A callback for when the Browse button is clicked. */
    #[prop_or_default]
    pub on_browse_button_click: Callback<()>,
    /** A callback for when the Clear button is clicked. */
    #[prop_or_default]
    pub on_clear_button_click: Callback<()>,
    /** Flag to show if a file is being dragged over the field */
    #[prop_or_default]
    pub is_drag_active: bool,
    // /** A reference object to attach to the FileUploadField container element. */
    // containerRef?: React.Ref<HTMLDivElement>;
}

pub enum FileUploadFieldMsg
{
    OnBrowseButtonClick,
    OnClearButtonClick,
}

impl Component for FileUploadField
{
    type Message = FileUploadFieldMsg;
    type Properties = FileUploadFieldProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self {
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
            FileUploadFieldMsg::OnBrowseButtonClick => {
                self.props.on_browse_button_click.emit(());
            },
            FileUploadFieldMsg::OnClearButtonClick => {
                self.props.on_clear_button_click.emit(());
            },
        }

        false
    }

    fn view(&self) -> Html
    {
        html!{
            <div
                class=(
                    "pf-c-file-upload",
                    // isDragActive && styles.modifiers.dragHover,
                    // isLoading && styles.modifiers.loading,
                    &self.props.class_name
                )
                // ref={containerRef}
                // {...props}
            >
                <div class="pf-c-file-upload__file-select">
                    <InputGroup>
                        <TextInput
                            is_read_only=true // Always read-only regardless of isReadOnly prop (which is just for the TextArea)
                            is_disabled=self.props.is_disabled
                            id=format!("{}-filename", self.props.id)
                            // name={`${id}-filename`}
                            // aria-label={filenameAriaLabel}
                            // placeholder={filenamePlaceholder}
                            // aria-describedby={`${id}-browse-button`}
                            value=&self.props.filename
                        />
                        <Button
                            id=format!("{}-browse-button", self.props.id)
                            variant=ButtonVariant::Control
                            onclick=self.link.callback(|_| FileUploadFieldMsg::OnBrowseButtonClick)
                            is_disabled=self.props.is_disabled
                        >
                            {&self.props.browse_button_text}
                        </Button>
                        <Button
                            variant=ButtonVariant::Control
                            is_disabled={self.props.is_disabled || self.props.is_clear_button_disabled}
                            onclick=self.link.callback(|_| FileUploadFieldMsg::OnClearButtonClick)
                        >
                            {&self.props.clear_button_text}
                        </Button>
                    </InputGroup>
                </div>
                <div class="pf-c-file-upload__file-details">
                {
                    if !self.props.hide_default_preview // && type is text
                    {
                        html!{
                            <TextArea
                                is_read_only=self.props.is_read_only    // {isReadOnly || (!!filename && !allowEditingUploadedText)}
                                is_disabled=self.props.is_disabled
                                is_required=self.props.is_required
                                resize_orientation=TextAreResizeOrientation::Vertical
                                validated=&self.props.validated
                                id=&self.props.id
                                // name=&self.props.id
                                aria_label=&self.props.aria_label
                                value=&self.props.value
                                // onchange={onTextAreaChange}
                            />
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
                {
                    if self.props.is_loading
                    {
                        html!{
                            <div class="pf-c-file-upload__file-details-spinner">
                                <Spinner
                                    size=SpinnerSize::Lg 
                                    aria_valuetext=&self.props.spinner_aria_value_text 
                                />
                            </div>
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
                </div>
                {
                    for self.props.children.iter()
                }
            </div>
        }
    }
}
