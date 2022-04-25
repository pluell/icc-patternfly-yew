use yew::{
    prelude::*,
};

pub struct AboutModalBoxHero;

#[derive(Clone, PartialEq, Properties)]
pub struct AboutModalBoxHeroProps
{
    /** additional classes added to the About Modal Hero  */
    #[prop_or_default]
    pub class_name: String,
    /** background image data or file path  */
    #[prop_or_default]
    pub background_image_src: Option<String>,
}

impl Component for AboutModalBoxHero
{
    type Message = ();
    type Properties = AboutModalBoxHeroProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div
                style={
                    if let Some(background_image_src) = &ctx.props().background_image_src {
                        Some(format!("--pf-c-about-modal-box__hero--sm--BackgroundImage: {}", background_image_src))
                    } else {
                        None
                    }
                }
                class={classes!("pf-c-about-modal-box__hero", ctx.props().class_name.clone())}
                // {...props}
            />
        }
    }
}
