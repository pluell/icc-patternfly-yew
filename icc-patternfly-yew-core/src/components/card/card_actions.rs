use yew::{
    prelude::*,
};


pub struct CardActions
{
    props: CardActionsProperties,
}

#[derive(Clone, PartialEq, Properties)]
pub struct CardActionsProperties
{
    /** Content rendered inside the Card Action */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the Action */
    #[prop_or_default]
    pub class_name: String,
}

impl Component for CardActions
{
    type Message = ();
    type Properties = CardActionsProperties;

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
            
                class=(
                    "pf-c-card__actions",
                    self.props.class_name.clone()
                )
            >
            {
                for self.props.children.iter()
            }
          </div>
        }
    }
}
