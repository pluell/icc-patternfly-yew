use yew::{
    prelude::*,
};


pub struct CardExpandableContent
{
    props: CardExpandableContentProperties,
}

#[derive(Clone, PartialEq, Properties)]
pub struct CardExpandableContentProperties
{
    /** Content rendered inside the Card Body */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the Card Body */
    #[prop_or_default]
    pub class_name: String,
    /** Flag indicating if a card is expanded. Modifies the card to be expandable. */
    #[prop_or_default]
    pub is_expanded: bool,
}

impl Component for CardExpandableContent
{
    type Message = ();
    type Properties = CardExpandableContentProperties;

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
        if self.props.is_expanded
        {
            html!{
                <div
                    class=classes!(
                        "pf-c-card__expandable-content", 
                        self.props.class_name.clone()
                    )
                    // {...props}
                >
                    { for self.props.children.iter() }
                </div>
            }
        }
        else
        {
            html!{}
        }
    }
}
