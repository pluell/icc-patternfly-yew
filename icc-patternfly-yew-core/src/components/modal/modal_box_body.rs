use yew::{
    prelude::*,
};


pub struct ModalBoxBody;

#[derive(Clone, PartialEq, Properties)]
pub struct ModalBoxBodyProperties
{
    /** Content rendered inside the ModalBoxBody */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the ModalBoxBody */
    #[prop_or_default]
    pub class_name: String,
}

impl Component for ModalBoxBody
{
    type Message = ();
    type Properties = ModalBoxBodyProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                //{...props} 
                class={classes!(
                    "pf-c-modal-box__body",
                    ctx.props().class_name.clone(),
                )}
            >
                { for ctx.props().children.iter() }
            </div>
        }
    }
}
