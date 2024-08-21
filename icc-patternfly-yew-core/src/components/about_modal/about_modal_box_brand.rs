use yew::prelude::*;

pub struct AboutModalBoxBrand;

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

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }


    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div class={classes!("pf-v5-c-about-modal-box__brand", ctx.props().class_name.clone())}
                // {...props}
            >
                <img class={classes!("pf-v5-c-about-modal-box__brand-image")} src={ctx.props().src.clone()} alt={ctx.props().alt.clone()} />
            </div>
        }
    }
}
