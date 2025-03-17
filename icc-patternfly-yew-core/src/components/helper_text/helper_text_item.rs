use yew::prelude::*;
use icc_patternfly_yew_icons::*;

#[derive(Clone, PartialEq)]
pub enum HelperTextItemVariant
{
    Default,
    Indeterminate,
    Warning,
    Error,
    Success,
}

impl std::fmt::Display for HelperTextItemVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self
        {
            HelperTextItemVariant::Default => write!(f, "default"),
            HelperTextItemVariant::Indeterminate => write!(f, "indeterminate"),
            HelperTextItemVariant::Warning => write!(f, "warning"),
            HelperTextItemVariant::Error => write!(f, "error"),
            HelperTextItemVariant::Success => write!(f, "success"),
        }
    }
}

pub struct HelperTextItem;

#[derive(Clone, PartialEq, Properties)]
pub struct HelperTextItemProperties
{
    /** Content rendered inside the helper text item. */
    pub children: Children,
    /** Additional classes applied to the helper text item. */
    #[prop_or_default]
    pub classes: Classes,
    /** Sets the component type of the helper text item. */
    #[prop_or(String::from("div"))]
    pub component: String,
    /** Variant styling of the helper text item. Will also render a default icon, which can be overridden
     * with the icon prop.
     */
    #[prop_or(HelperTextItemVariant::Default)]
    pub variant: HelperTextItemVariant,
    /** Custom icon prefixing the helper text. This property will override the default icon when the variant property is passed in. */
    #[prop_or_default]
    pub icon: Option<Html>,
    /** ID for the helper text item. The value of this prop can be passed into a form component's
     * aria-describedby prop when you intend for only specific helper text items to be announced to
     * assistive technologies.
     */
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /** Text that is only accessible to screen readers in order to announce the variant of a helper text item.
     * This prop can only be used when the variant prop has a value other than "default".
     */
    #[prop_or_default]
    pub screen_reader_text: Option<AttrValue>,
}

impl Component for HelperTextItem
{
    type Message = ();
    type Properties = HelperTextItemProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let icon = if ctx.props().icon.is_some() {
            ctx.props().icon.clone()
        } else {
            match ctx.props().variant {
                HelperTextItemVariant::Default => None,
                HelperTextItemVariant::Indeterminate => Some(minus_icon!{}),
                HelperTextItemVariant::Warning => Some(exclamation_triangle_icon!{}),
                HelperTextItemVariant::Error => Some(exclamation_circle_icon!{}),
                HelperTextItemVariant::Success => Some(check_circle_icon!{}),
            }
        };

        let variant_class = match ctx.props().variant {
            HelperTextItemVariant::Default => String::new(),
            _ => format!("pf-m-{}", ctx.props().variant.to_string())
        };

        let screen_reader_text = if let Some(text) = &ctx.props().screen_reader_text {
            text.to_string()
        } else {
            format!("{} status", ctx.props().variant.to_string())
        };

        html!{
            <@{ctx.props().component.clone()}
                id={ctx.props().id.clone()}
                class={classes!(
                    "pf-v5-c-helper-text__item",
                    variant_class,
                    ctx.props().classes.clone()
                )}
                // {...props}
            >
                {
                    if let Some(icon) = icon {
                        html!{<span class={classes!("pf-v5-c-helper-text__item-icon")}>{icon}</span>}
                    } else {
                        html!{}
                    }
                }

                <span class={classes!("pf-v5-c-helper-text__item-text")}>
                    { ctx.props().children.clone() }
                    {
                        if ctx.props().variant != HelperTextItemVariant::Default {
                            html!{
                                <span class="pf-v5-screen-reader">{format!(": {};", screen_reader_text)}</span>
                            }
                        } else {
                            html!{}
                        }
                    }
                </span>
            </@>
        }
    }
}
