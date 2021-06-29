use yew::{
    prelude::*,
    virtual_dom::{VTag},
};

use super::{TitleHeadingLevels, TitleSizes};


pub struct Title
{
    props: TitleProperties,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TitleProperties
{
    /** The size of the Title  */
    #[prop_or_default]
    pub size: Option<TitleSizes>,
    /** Content rendered inside the Title */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the Title */
    #[prop_or_default]
    pub class_name: String,
    /** The heading level to use */
    pub heading_level: TitleHeadingLevels,

    // Additional Props
    #[prop_or_default]
    pub id: Option<String>,
}

impl Component for Title
{
    type Message = ();
    type Properties = TitleProperties;

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
        let mut component: VTag = self.props.heading_level.clone().into();

        // Build list of classes
        let mut classes = String::from("pf-c-title");

        // Get the size class or default size
        if let Some(size) = &self.props.size
        {
            classes += " ";
            classes += size.get_class();
        }
        else
        {
            classes += " ";
            classes += self.props.heading_level.get_default_size().get_class();
        }
        
        // Add extra classes specified on the parent
        if self.props.class_name.len() > 0
        {
            classes += " ";
            classes += &self.props.class_name;
        }

        component.add_attribute("class", classes);

        //     {...props}
        if let Some(id) = &self.props.id
        {
            component.add_attribute("id", id.clone());
        }

        component.add_children(self.props.children.iter());

        component.into()
    }
}
