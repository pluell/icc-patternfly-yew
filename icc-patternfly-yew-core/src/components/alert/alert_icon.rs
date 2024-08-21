use yew::prelude::*;

use super::AlertVariant;


pub struct AlertIcon;

#[derive(Clone, PartialEq, Properties)]
pub struct AlertIconProps
{
    /** variant */
    #[prop_or(AlertVariant::Default)]
    pub variant: AlertVariant,
    /** className */
    #[prop_or_default]
    pub class_name: String,
    /** A custom icon. If not set the icon is set according to the variant */
    #[prop_or_default]
    pub custom_icon: Option<Html>,
}

impl Component for AlertIcon
{
    type Message = ();
    type Properties = AlertIconProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div
                // {...props}
                class={classes!(
                    "pf-v5-c-alert__icon", 
                    &ctx.props().class_name
                )}
            >
            {
                if let Some(custom_icon) = &ctx.props().custom_icon
                {
                    custom_icon.clone()
                }
                else
                {
                    ctx.props().variant.view()
                }
            }
            </div>
        }
    }
}
