use yew::{
    prelude::*,
};

use crate::{Title, TitleHeadingLevels, TitleSizes};

pub struct AboutModalBoxHeader
{
    props: AboutModalBoxHeaderProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct AboutModalBoxHeaderProps
{
    /** additional classes added to the button  */
    #[prop_or_default]
    pub class_name: String,
    /** Name of the Product  */
    #[prop_or_default]
    pub product_name: String,
    /** id to used for Modal Box header  */
    pub id: String,
}

impl Component for AboutModalBoxHeader
{
    type Message = ();
    type Properties = AboutModalBoxHeaderProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self
    {
        Self {
            props
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
            <div class=classes!("pf-c-about-modal-box__header", self.props.class_name.clone())
                //{...props}
            >
                <Title heading_level=TitleHeadingLevels::H1 size=TitleSizes::X4l id=self.props.id.clone()>
                    {&self.props.product_name}
                </Title>
            </div>
        }
    }
}
