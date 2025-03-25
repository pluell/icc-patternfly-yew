use yew::prelude::*;


pub struct ModalBoxHeader;

#[derive(Clone, PartialEq, Properties)]
pub struct ModalBoxHeaderProperties
{
    /** Content rendered inside the Header */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the button */
    #[prop_or_default]
    pub classes: Classes,
    /** Optional help section for the Modal Header */
    #[prop_or_default]
    pub help: Option<Html>,
}

impl Component for ModalBoxHeader
{
    type Message = ();
    type Properties = ModalBoxHeaderProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <header 
                class={classes!(
                    "pf-v5-c-modal-box__header", 
                    if ctx.props().help.is_some() { "pf-m-help" } else { "" },
                    ctx.props().classes.clone(),
                )}
                // {...props}
            >
            {
                if let Some(help) = &ctx.props().help
                {
                    html!{
                        <>
                            <div class="pf-v5-c-modal-box__header-main">
                                { for ctx.props().children.iter() }
                            </div>
                            <div class="pf-v5-c-modal-box__header-help">
                                { help.clone() }
                            </div>
                        </>
                    }
                }
                else
                {
                    html!{
                        for ctx.props().children.iter()
                    }
                }
            }
            </header>
        }
    }
}
