use yew::prelude::*;

use crate::Backdrop;

use super::{
    ModalVariants, ModalBox, ModalBoxBody, ModalBoxCloseButton, ModalBoxDescription, 
    ModalBoxFooter,ModalBoxHeader, ModalBoxTitle, ModalTitleIconVariants
};

const MODAL_TITLE_ICON_CLASS_SUCCESS_IDX: usize = 0;
const MODAL_TITLE_ICON_CLASS_DANGER_IDX: usize = 1;
const MODAL_TITLE_ICON_CLASS_WARNING_IDX: usize = 2;
const MODAL_TITLE_ICON_CLASS_INFO_IDX: usize = 3;
const MODAL_TITLE_ICON_CLASS_DEFAULT_IDX: usize = 4;

const MODAL_TITLE_ICON_CLASSES: &'static [&'static str] = &[
    "pf-m-success",
    "pf-m-danger",
    "pf-m-warning",
    "pf-m-info",
    "pf-m-default",
    ""
];


pub struct ModalContent;

#[derive(Clone, PartialEq, Properties)]
pub struct ModalContentProperties
{
    /** Content rendered inside the Modal. */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the button */
    #[prop_or_default]
    pub class_name: String,
    /** Variant of the modal */
    #[prop_or(ModalVariants::Default)]
    pub variant: ModalVariants,
    /** Alternate position of the modal */
    #[prop_or_default]
    pub position_top: bool,
    /** Offset from alternate position. Can be any valid CSS length/percentage */
    #[prop_or_default]
    pub position_offset: String,
    /** Flag to show the modal */
    #[prop_or_default]
    pub is_open: bool,
    /** Complex header (more than just text), supersedes title for header content */
    #[prop_or_default]
    pub header: Option<Html>,
    /** Optional help section for the Modal Header */
    #[prop_or_default]
    pub help: Option<Html>,
    /** Description of the modal */
    #[prop_or_default]
    pub description: Option<Html>,
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
    /** Id of Modal Box label */
    #[prop_or_default]
    pub aria_labelledby: Option<String>,
    /** Accessible descriptor of modal */
    #[prop_or_default]
    pub aria_label: String,
    /** Id of Modal Box description */
    #[prop_or_default]
    pub aria_describedby: String,
    /** Flag to show the close button in the header area of the modal */
    #[prop_or(true)]
    pub show_close: bool,
    /** Default width of the content. */
    #[prop_or_default]
    pub width: String,
    /** Custom footer */
    #[prop_or_default]
    pub footer: Option<Html>,
    /** Action buttons to add to the standard Modal Footer, ignored if `footer` is given */
    #[prop_or_default]
    pub actions: Option<Html>,
    /** A callback for when the close button is clicked */
    #[prop_or_default]
    pub onclose: Callback<()>,
    /** Id of the ModalBox container */
    #[prop_or_default]
    pub box_id: String,
    /** Id of the ModalBox title */
    #[prop_or_default]
    pub label_id: String,
    /** Id of the ModalBoxBody */
    #[prop_or_default]
    pub descriptor_id: String,
    /** Flag to disable focus trap */
    #[prop_or_default]
    pub disable_focus_trap: bool,
    /** Flag indicating if modal content should be placed in a modal box body wrapper */
    #[prop_or_default]
    pub has_no_body_wrapper: bool,
}

impl Component for ModalContent
{
    type Message = ();
    type Properties = ModalContentProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        if !ctx.props().is_open
        {
            html!{}
        }
        else
        {
            // Get the css class for the icon type
            let modal_title_icon_class = if let Some(title_icon_variant) = &ctx.props().title_icon_variant {
                match title_icon_variant
                {
                    ModalTitleIconVariants::Success => MODAL_TITLE_ICON_CLASSES[MODAL_TITLE_ICON_CLASS_SUCCESS_IDX],
                    ModalTitleIconVariants::Danger => MODAL_TITLE_ICON_CLASSES[MODAL_TITLE_ICON_CLASS_DANGER_IDX],
                    ModalTitleIconVariants::Warning => MODAL_TITLE_ICON_CLASSES[MODAL_TITLE_ICON_CLASS_WARNING_IDX],
                    ModalTitleIconVariants::Info => MODAL_TITLE_ICON_CLASSES[MODAL_TITLE_ICON_CLASS_INFO_IDX],
                    ModalTitleIconVariants::Default => MODAL_TITLE_ICON_CLASSES[MODAL_TITLE_ICON_CLASS_DEFAULT_IDX],
                    _ => "",
                }
            } else {
                ""
            };

            // Set the style if width is set
            let box_style = if ctx.props().width.len() > 0 {
                Some(format!("width: {}", ctx.props().width))
            } else {
                None
            };

            html!{
                <Backdrop>
                    // TODO: Convert this to a FocusTrap
                    <div class="pf-v5-l-bullseye">
                        <ModalBox
                            id={ctx.props().box_id.clone()}
                            style={box_style}
                            class_name={format!("{} {}", ctx.props().class_name, modal_title_icon_class)}
                            variant={ctx.props().variant.clone()}
                            position_top={ctx.props().position_top}
                            position_offset={ctx.props().position_offset.clone()}
                            // aria-label={ariaLabel}
                            // aria-labelledby={ariaLabelledbyFormatted()}
                            // aria-describedby={ariaDescribedby || (hasNoBodyWrapper ? null : descriptorId)}
                            // {...getOUIAProps(ModalContent.displayName, ouiaId, ouiaSafe)}
                        >
                            {
                                if ctx.props().show_close
                                {
                                    html!{
                                        <ModalBoxCloseButton onclose={ctx.props().onclose.clone()} />
                                    }
                                }
                                else
                                {
                                    html!{}
                                }
                            }
                            { self.render_box_header(ctx) }
                            { self.render_box_body(ctx) }
                            { self.render_box_footer(ctx) }
                        </ModalBox>
                    </div>
                </Backdrop>
            }
        }
    }
}

impl ModalContent
{
    fn render_box_header(&self, ctx: &Context<Self>) -> Html
    {
        if let Some(header) = &ctx.props().header
        {
            html!{
                <ModalBoxHeader help={ctx.props().help.clone()} >
                    { header.clone() }
                </ModalBoxHeader>
            }
        }
        else
        {
            if ctx.props().title.len() > 0
            {
                html!{
                    <ModalBoxHeader help={ctx.props().help.clone()}>
                        <ModalBoxTitle 
                            title={ctx.props().title.clone()}
                            title_icon_variant={ctx.props().title_icon_variant.clone()}
                            title_label={ctx.props().title_label.clone()}
                            id={ctx.props().label_id.clone()}
                        />
                        {
                            if let Some(description) = &ctx.props().description
                            {
                                html!{
                                    <ModalBoxDescription id={ctx.props().descriptor_id.clone()} >
                                        {description.clone()}
                                    </ModalBoxDescription>
                                }
                            }
                            else
                            {
                                html!{}
                            }
                        }
                    </ModalBoxHeader>
                }
            }
            else
            {
                html!{}
            }
        }
    }

    fn render_box_body(&self, ctx: &Context<Self>) -> Html
    {
        if ctx.props().has_no_body_wrapper
        {
            html!{ for ctx.props().children.iter() }
        }
        else
        {
            html!{
                <ModalBoxBody
                    //{...props} 
                    // {...(!description && !ariaDescribedby && { id: descriptorId })}
                >
                    { for ctx.props().children.iter() }
                </ModalBoxBody>
            }
        }
    }

    fn render_box_footer(&self, ctx: &Context<Self>) -> Html
    {
        if let Some(footer) = &ctx.props().footer
        {
            html!{
                <ModalBoxFooter>{ footer.clone() }</ModalBoxFooter>
            }
        }
        else if let Some(actions) = &ctx.props().actions
        {
            html!{
                <ModalBoxFooter>{ actions.clone() }</ModalBoxFooter>
            }
        }
        else
        {
            html!{}
        }
    }
}
