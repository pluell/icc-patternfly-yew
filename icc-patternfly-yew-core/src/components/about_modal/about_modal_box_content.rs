use yew::{
    prelude::*,
};


pub struct AboutModalBoxContent;

#[derive(Clone, PartialEq, Properties)]
pub struct AboutModalBoxContentProps
{
    /** content rendered inside the AboutModalBoxContent  */
    #[prop_or_default]
    pub children: Children,
    /** additional classes added to the AboutModalBoxContent  */
    #[prop_or_default]
    pub class_name: String,
    /** id to use for About Modal Box aria described by  */
    pub id: String,
    /** The Trademark info for the product  */
    pub trademark: String,
    /** Prevents the about modal from rendering content inside a container; allows for more flexible layouts */
    #[prop_or_default]
    pub no_about_modal_box_content_container: bool,
}

impl Component for AboutModalBoxContent
{
    type Message = ();
    type Properties = AboutModalBoxContentProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div class={classes!("pf-c-about-modal-box__content", ctx.props().class_name.clone())}
                id={ctx.props().id.clone()}
                // {...props}
            >
                <div class={classes!("pf-c-about-modal-box__body")}>
                {
                    if ctx.props().no_about_modal_box_content_container
                    {
                        html!{
                            for ctx.props().children.iter()
                        }
                    }
                    else
                    {
                        html!{
                            <div class={classes!("pf-c-content")}>{for ctx.props().children.iter()}</div>
                        }
                    }
                }
                </div>
                <p class={classes!("pf-c-about-modal-box__strapline")}>{ctx.props().trademark.clone()}</p>
            </div>
        }
    }
}
