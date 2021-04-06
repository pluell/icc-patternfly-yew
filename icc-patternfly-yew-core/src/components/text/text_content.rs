use yew::{
    prelude::*,
};


pub struct TextContent
{
    props: TextContentProperties,
}

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
                // {...props}
                class=(
                    "pf-c-content", 
                    self.props.class_name.clone(),
                )
            >
                { for self.props.children.iter() }
            </div>
        }
    }
}
