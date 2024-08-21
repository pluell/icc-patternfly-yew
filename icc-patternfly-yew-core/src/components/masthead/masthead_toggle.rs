use yew::prelude::*;


pub struct MastheadToggle;

#[derive(Clone, PartialEq, Properties)]
pub struct MastheadToggleProps
{
    /** Content rendered inside of the masthead toggle. */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the masthead toggle. */
    #[prop_or_default]
    pub class_name: String,
}

impl Component for MastheadToggle
{
    type Message = ();
    type Properties = MastheadToggleProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                class={classes!(
                    "pf-v5-c-masthead__toggle",
                    ctx.props().class_name.clone(),
                )}
                // {...props}
            >
                {for ctx.props().children.iter()}
            </div>
        }
    }
}
