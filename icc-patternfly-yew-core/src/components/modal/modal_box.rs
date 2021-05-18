use yew::{
    prelude::*,
};

use super::{ModalVariants};


pub struct ModalBox
{
    props: ModalBoxProperties,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ModalBoxProperties
{
    /** Content rendered inside the ModalBox. */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the ModalBox */
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
    /** Id to use for Modal Box label */
    #[prop_or_default]
    pub aria_labelledby: Option<String>,
    /** Accessible descriptor of modal */
    #[prop_or_default]
    pub aria_label: String,
    /** Id to use for Modal Box description */
    #[prop_or_default]
    pub aria_describedby: String,
    /** Id of the ModalBox container */
    #[prop_or_default]
    pub id: String,
    /** Style of the ModalBox container */
    #[prop_or_default]
    pub style: Option<String>,
}

impl Component for ModalBox
{
    type Message = ();
    type Properties = ModalBoxProperties;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self
    {
        Self {
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
    fn update(&mut self, _: Self::Message) -> ShouldRender
    {
        false
    }

    fn view(&self) -> Html
    {
        let mut style = if let Some(style) = &self.props.style {
            style.clone()
        } else {
            String::new()
        };

        if self.props.position_offset.len() > 0
        {
            style += &format!(" --pf-c-modal-box--m-align-top--spacer: {};", self.props.position_offset);
        }

        html!{
            <div
                // {...props}
                id=self.props.id.clone()
                role="dialog"
                // aria-label={ariaLabel || null}
                // aria-labelledby={ariaLabelledby || null}
                // aria-describedby={ariaDescribedby}
                aria-modal="true"
                class=classes!(
                    "pf-c-modal-box",
                    self.props.class_name.clone(),
                    if self.props.position_top { "pf-m-align-top" } else { "" },
                    if self.props.variant == ModalVariants::Large { "pf-m-lg" } else { "" },
                    if self.props.variant == ModalVariants::Small { "pf-m-sm" } else { "" },
                    if self.props.variant == ModalVariants::Medium { "pf-m-md" } else { "" },
                )
                style=style
            >
                { for self.props.children.iter() }
            </div>
        }
    }
}
