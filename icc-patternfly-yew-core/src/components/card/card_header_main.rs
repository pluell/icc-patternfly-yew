use yew::{
    prelude::*,
};


pub struct CardHeaderMain
{
    props: CardHeaderMainProperties,
}

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

                class=self.props.class_name.clone()
            >
            {
                for self.props.children.iter()
            }
          </div>
        }
    }
}
