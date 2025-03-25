use yew::prelude::*;


pub struct ModalBoxFooter;

#[derive(Clone, PartialEq, Properties)]
pub struct ModalBoxFooterProperties
{
    /** Content rendered inside the Footer */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the Footer */
    #[prop_or_default]
    pub classes: Classes,
}

impl Component for ModalBoxFooter
{
    type Message = ();
    type Properties = ModalBoxFooterProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <footer
                //{...props} 
                class={classes!(
                    "pf-v5-c-modal-box__footer",
                    ctx.props().classes.clone(),
                )}
            >
                { for ctx.props().children.iter() }
            </footer>
        }
    }
}
