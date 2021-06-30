use yew::{
    prelude::*,
};

pub struct AboutModalBoxHero
{
    props: AboutModalBoxHeroProps,
}

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
            <div
                style={
                    if let Some(background_image_src) = &self.props.background_image_src {
                        Some(format!("--pf-c-about-modal-box__hero--sm--BackgroundImage: {}", background_image_src))
                    } else {
                        None
                    }
                }
                class=classes!("pf-c-about-modal-box__hero", self.props.class_name.clone())
                // {...props}
            />
        }
    }
}
