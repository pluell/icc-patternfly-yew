use yew::prelude::*;

use super::ModalTitleIconVariants;


pub struct ModalBoxTitle;

#[derive(Clone, PartialEq, Properties)]
pub struct ModalBoxTitleProperties
{
    /** Content rendered inside the modal box header title. */
    #[prop_or_default]
    pub title: String,  //React.ReactNode,
    /** Optional alert icon (or other) to show before the title of the Modal Header
     * When the predefined alert types are used the default styling
     * will be automatically applied */
    #[prop_or_default]
    pub title_icon_variant: Option<ModalTitleIconVariants>,
    /** Optional title label text for screen readers */
    #[prop_or_default]
    pub title_label: String,
    /** Additional classes added to the modal box header title. */
    #[prop_or_default]
    pub class_name: String,
    /** id of the modal box header title. */
    #[prop_or_default]
    pub id: String,
}

impl Component for ModalBoxTitle
{
    type Message = ();
    type Properties = ModalBoxTitleProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let label = if ctx.props().title_label.len() > 0
        {
            ctx.props().title_label.clone() 
        }
        else
        { 
            // (isVariantIcon(titleIconVariant) ? `${capitalize(titleIconVariant)} alert:` : titleLabel);
            String::new()
        };

        html!{
            <h1
                id={ctx.props().id.clone()}
                // ref={h1}
                class={classes!(
                    "pf-v5-c-modal-box__title", 
                    if ctx.props().title_icon_variant.is_some() { "pf-m-icon" } else { "" },
                    ctx.props().class_name.clone()
                )}
                // {...props}
            >
                {
                    if let Some(title_icon) = &ctx.props().title_icon_variant
                    {
                        html!{
                            <span class="pf-v5-c-modal-box__title-icon">
                            {
                                match title_icon
                                {
                                    ModalTitleIconVariants::Success => {
                                        html!{<i class="fas fa-fw fa-check-circle" aria-hidden="true"></i>}
                                    },
                                    ModalTitleIconVariants::Danger => {
                                        html!{<i class="fas fa-fw fa-exclamation-circle" aria-hidden="true"></i>}
                                    },
                                    ModalTitleIconVariants::Warning => {
                                        html!{<i class="fas fa-fw fa-exclamation-triangle" aria-hidden="true"></i>}
                                    },
                                    ModalTitleIconVariants::Info => {
                                        html!{<i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>}
                                    },
                                    ModalTitleIconVariants::Default => {
                                        html!{<i class="fas fa-fw fa-bell" aria-hidden="true"></i>}
                                    },
                                    ModalTitleIconVariants::Custom(custom_icon) => {
                                        custom_icon.clone()
                                    }
                                }
                            }
                            </span>
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
                {
                    if label.len() > 0
                    {
                        html!{
                            <span class="pf-u-screen-reader">{&ctx.props().title_label}</span>
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
                <span class="pf-v5-c-modal-box__title">{&ctx.props().title}</span>
            </h1>
        }
    }
}
