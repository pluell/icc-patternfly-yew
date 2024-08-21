use yew::prelude::*;


pub struct ModalBoxDescription;

#[derive(Clone, PartialEq, Properties)]
pub struct ModalBoxDescriptionProperties
{
    /** Content rendered inside the description */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the description */
    #[prop_or_default]
    pub class_name: String,
    /** ID of the description */
    #[prop_or_default]
    pub id: String,
}

impl Component for ModalBoxDescription
{
    type Message = ();
    type Properties = ModalBoxDescriptionProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div
                // {...props} 
                id={ctx.props().id.clone()}
                class={classes!(
                    "pf-v5-c-modal-box__description",
                    ctx.props().class_name.clone()
                )}
            >
                { for ctx.props().children.iter() }
            </div>
        }
    }
}
