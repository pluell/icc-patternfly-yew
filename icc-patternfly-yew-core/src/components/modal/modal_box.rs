use yew::prelude::*;

use super::ModalVariants;


pub struct ModalBox;

#[derive(Clone, PartialEq, Properties)]
pub struct ModalBoxProperties
{
    /** Content rendered inside the ModalBox. */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the ModalBox */
    #[prop_or_default]
    pub classes: Classes,
    /** Variant of the modal */
    #[prop_or(ModalVariants::Default)]
    pub variant: ModalVariants,
    /** Alternate position of the modal */
    #[prop_or_default]
    pub position_top: bool,
    /** Offset from alternate position. Can be any valid CSS length/percentage */
    #[prop_or_default]
    pub position_offset: AttrValue,
    /** Id to use for Modal Box label */
    #[prop_or_default]
    pub aria_labelledby: Option<AttrValue>,
    /** Accessible descriptor of modal */
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,
    /** Id to use for Modal Box description */
    #[prop_or_default]
    pub aria_describedby: Option<AttrValue>,
    /** Id of the ModalBox container */
    #[prop_or_default]
    pub id: AttrValue,
    /** Style of the ModalBox container */
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

impl Component for ModalBox
{
    type Message = ();
    type Properties = ModalBoxProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let mut style = if let Some(style) = &ctx.props().style {
            style.to_string()
        } else {
            String::new()
        };

        if ctx.props().position_offset.len() > 0
        {
            style += &format!(" --pf-v5-c-modal-box--m-align-top--spacer: {};", ctx.props().position_offset);
        }

        html!{
            <div
                // {...props}
                id={ctx.props().id.clone()}
                role="dialog"
                // aria-label={ariaLabel || null}
                // aria-labelledby={ariaLabelledby || null}
                // aria-describedby={ariaDescribedby}
                aria-modal="true"
                class={classes!(
                    "pf-v5-c-modal-box",
                    ctx.props().classes.clone(),
                    if ctx.props().position_top { "pf-m-align-top" } else { "" },
                    if ctx.props().variant == ModalVariants::Large { "pf-m-lg" } else { "" },
                    if ctx.props().variant == ModalVariants::Small { "pf-m-sm" } else { "" },
                    if ctx.props().variant == ModalVariants::Medium { "pf-m-md" } else { "" },
                )}
                style={style}
            >
                { for ctx.props().children.iter() }
            </div>
        }
    }
}
