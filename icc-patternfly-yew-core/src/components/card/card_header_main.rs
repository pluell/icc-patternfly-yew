use yew::{
    prelude::*,
};


pub struct CardHeaderMain;

#[derive(Clone, PartialEq, Properties)]
pub struct CardHeaderMainProperties
{
    /** Content rendered inside the Card Head Main */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the Card Head Main */
    #[prop_or_default]
    pub class_name: String,
}

impl Component for CardHeaderMain
{
    type Message = ();
    type Properties = CardHeaderMainProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 

                class={ctx.props().class_name.clone()}
            >
            {
                for ctx.props().children.iter()
            }
          </div>
        }
    }
}
