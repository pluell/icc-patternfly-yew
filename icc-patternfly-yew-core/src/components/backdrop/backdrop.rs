use yew::{
    prelude::*,
};


pub struct Backdrop
{
    props: BackdropProperties,
}

#[derive(Clone, PartialEq, Properties)]
pub struct BackdropProperties
{
    /** content rendered inside the backdrop */
    #[prop_or_default]
    pub children: Children,
    /** additional classes added to the button */
    #[prop_or_default]
    pub class_name: String,
}

impl Component for Backdrop
{
    type Message = ();
    type Properties = BackdropProperties;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self
    {
        Self {
            props,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        if self.props != props
        {
            self.props = props;
            
            true
        }
        else
        {
            false
        }
    }

    /// Called everytime when messages are received
    fn update(&mut self, _: Self::Message) -> ShouldRender
    {
        false
    }

    fn view(&self) -> Html
    {
        html!{
            <div 
                class=classes!(
                    "pf-c-backdrop",
                    self.props.class_name.clone()
                )
            >
            { for self.props.children.iter() }
            </div>
        }
    }
}
