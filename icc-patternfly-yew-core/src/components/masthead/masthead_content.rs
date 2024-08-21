use yew::prelude::*;


pub struct MastheadContent;

#[derive(Clone, PartialEq, Properties)]
pub struct MastheadContentProps
{
    /** Content rendered inside of the masthead content block. */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the masthead content. */
    #[prop_or_default]
    pub class_name: String,
}

impl Component for MastheadContent
{
    type Message = ();
    type Properties = MastheadContentProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                class={classes!(
                    "pf-v5-c-masthead__content",
                    ctx.props().class_name.clone(),
                )}
                // {...props}
            >
                {for ctx.props().children.iter()}
            </div>
        }
    }
}
