use yew::{
    prelude::*,
};

pub struct Brand
{
    props: BrandProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct BrandProps
{
    /** Additional classes added to the Brand. */
    #[prop_or_default]
    pub class_name: String,
    /** Attribute that specifies the URL of the image for the Brand. */
    #[prop_or_default]
    pub src: String,
    /** Attribute that specifies the alt text of the image for the Brand. */
    pub alt: String,
}

impl Component for Brand
{
    type Message = ();
    type Properties = BrandProps;

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
        true
    }

    fn view(&self) -> Html
    {
        html!{
            <img
                // {...props}
                class=classes!(
                    "pf-c-brand",
                    self.props.class_name.clone()
                )
                src=self.props.src.to_string()
                alt=self.props.alt.to_string()
            />
        }
    }
}
