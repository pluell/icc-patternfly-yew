use yew::prelude::*;

pub struct FormHelperText;

#[derive(Clone, PartialEq, Properties)]
pub struct FormHelperTextProperties
{
    /** Content rendered inside the Helper Text Item */
    pub children: Children,
    /** Adds error styling to the Helper Text  * */
    #[prop_or_default]
    pub is_error: bool,
    /** Hides the helper text * */
    #[prop_or_default]
    pub is_hidden: bool,
    /** Additional classes added to the Helper Text Item  */
    #[prop_or_default]
    pub class_name: String,
    /** Icon displayed to the left of the helper text. */
    #[prop_or_default]
    pub icon: Option<Html>,
}

impl Component for FormHelperText
{
    type Message = ();
    type Properties = FormHelperTextProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <p
                class={classes!(
                    "pf-v5-c-form__helper-text",
                    if ctx.props().is_error {"pf-m-error"} else {""},
                    if ctx.props().is_hidden {"pf-m-hidden"} else {""},
                    ctx.props().class_name.to_string()
                )}
                // {...props}
            >
                {
                    if let Some(icon) = &ctx.props().icon
                    {
                        html!{
                            <span class="pf-v5-c-form__helper-text-icon">{icon.clone()}</span>
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
                { ctx.props().children.clone() }
            </p>
        }
    }
}
