use yew::{
    prelude::*,
};

use super::{AlertVariant};


pub struct AlertIcon
{
    props: AlertIconProps,
}

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
        html!{
            <div
                // {...props}
                class=classes!(
                    "pf-c-alert__icon", 
                    &self.props.class_name
                )
            >
            {
                if let Some(custom_icon) = &self.props.custom_icon
                {
                    custom_icon.clone()
                }
                else
                {
                    self.props.variant.view()
                }
            }
            </div>
        }
    }
}
