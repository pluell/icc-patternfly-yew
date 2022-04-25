use yew::{
    prelude::*,
    virtual_dom::{VTag},
};

use super::{TitleHeadingLevels, TitleSizes};


pub struct Title;

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

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let mut component: VTag = ctx.props().heading_level.clone().into();

        // Build list of classes
        let mut classes = String::from("pf-c-title");

        // Get the size class or default size
        if let Some(size) = &ctx.props().size
        {
            classes += " ";
            classes += size.get_class();
        }
        else
        {
            classes += " ";
            classes += ctx.props().heading_level.get_default_size().get_class();
        }
        
        // Add extra classes specified on the parent
        if ctx.props().class_name.len() > 0
        {
            classes += " ";
            classes += &ctx.props().class_name;
        }

        component.add_attribute("class", classes);

        //     {...props}
        if let Some(id) = &ctx.props().id
        {
            component.add_attribute("id", id.clone());
        }

        component.add_children(ctx.props().children.iter());

        component.into()
    }
}
