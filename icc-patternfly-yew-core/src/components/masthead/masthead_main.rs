use yew::prelude::*;


pub struct MastheadMain;

#[derive(Clone, PartialEq, Properties)]
pub struct MastheadMainProps
{
    /** Content rendered inside of the masthead main block. */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the masthead main. */
    #[prop_or_default]
    pub class_name: String,
}

impl Component for MastheadMain
{
    type Message = ();
    type Properties = MastheadMainProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                class={classes!(
                    "pf-v5-c-masthead__main",
                    ctx.props().class_name.clone(),
                )}
                // {...props}
            >
                {for ctx.props().children.iter()}
            </div>
        }
    }
}
