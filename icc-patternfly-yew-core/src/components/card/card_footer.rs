use yew::{
    prelude::*,
    virtual_dom::{VTag},
};


pub struct CardFooter
{
    props: CardFooterProperties,
}

#[derive(Clone, PartialEq, Properties)]
pub struct CardFooterProperties
{
    /** Content rendered inside the Card Footer */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the Footer */
    #[prop_or_default]
    pub class_name: String,
    /** Sets the base component to render. defaults to div */
    #[prop_or(String::from("div"))]
    pub component: String,
}

impl Component for CardFooter
{
    type Message = ();
    type Properties = CardFooterProperties;

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
        let mut component = VTag::new(self.props.component.clone());

        // Build list of classes
        let mut classes = String::from("pf-c-card__footer");
        
        // Add extra classes specified on the parent
        if self.props.class_name.len() > 0
        {
            classes += " ";
            classes += &self.props.class_name;
        }

        component.add_attribute("class", classes);

        //     {...props}

        component.add_children(self.props.children.iter());

        component.into()
    }
}
