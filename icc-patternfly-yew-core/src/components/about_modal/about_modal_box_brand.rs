use yew::{
    prelude::*,
};

pub struct AboutModalBoxBrand
{
    props: AboutModalBoxBrandProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct AboutModalBoxBrandProps
{
    /** additional classes added to the About Modal Brand  */
    #[prop_or_default]
    pub class_name: String,
    /** the URL of the image for the Brand.  */
    #[prop_or_default]
    pub src: String,
    /** the alternate text of the Brand image.  */
    pub alt: String,
}

impl Component for AboutModalBoxBrand
{
    type Message = ();
    type Properties = AboutModalBoxBrandProps;

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
            <div class=classes!("pf-c-about-modal-box__brand", self.props.class_name.clone())
                // {...props}
            >
                <img class=classes!("pf-c-about-modal-box__brand-image") src=self.props.src.clone() alt=self.props.alt.clone() />
            </div>
        }
    }
}
