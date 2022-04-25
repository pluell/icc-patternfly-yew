use yew::{
    prelude::*,
};

pub struct Brand;

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

    // Additional properties
    #[prop_or_default]
    pub style: Option<String>,
}

impl Component for Brand
{
    type Message = ();
    type Properties = BrandProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <img
                // {...props}
                style={ctx.props().style.clone()}
                class={classes!(
                    "pf-c-brand",
                    ctx.props().class_name.clone()
                )}
                src={ctx.props().src.to_string()}
                alt={ctx.props().alt.to_string()}
            />
        }
    }
}
