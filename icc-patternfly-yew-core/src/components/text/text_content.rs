use yew::prelude::*;


pub struct TextContent;

#[derive(Clone, PartialEq, Properties)]
pub struct TextContentProperties
{
    /** Content rendered within the TextContent */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the TextContent */
    #[prop_or_default]
    pub class_name: String,
}

impl Component for TextContent
{
    type Message = ();
    type Properties = TextContentProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                // {...props}
                class={classes!(
                    "pf-v5-c-content", 
                    ctx.props().class_name.clone(),
                )}
            >
                { for ctx.props().children.iter() }
            </div>
        }
    }
}
