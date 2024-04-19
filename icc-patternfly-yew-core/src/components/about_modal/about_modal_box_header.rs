use yew::{
    prelude::*,
};

use crate::{Title, TitleHeadingLevels, TitleSizes};

pub struct AboutModalBoxHeader;

#[derive(Clone, PartialEq, Properties)]
pub struct AboutModalBoxHeaderProps
{
    /** additional classes added to the button  */
    #[prop_or_default]
    pub class_name: String,
    /** Name of the Product  */
    #[prop_or_default]
    pub product_name: AttrValue,
    /** id to used for Modal Box header  */
    pub id: String,
}

impl Component for AboutModalBoxHeader
{
    type Message = ();
    type Properties = AboutModalBoxHeaderProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div class={classes!("pf-c-about-modal-box__header", ctx.props().class_name.clone())}
                //{...props}
            >
                <Title heading_level={TitleHeadingLevels::H1} size={TitleSizes::X4l} id={ctx.props().id.clone()}>
                    {ctx.props().product_name.clone()}
                </Title>
            </div>
        }
    }
}
