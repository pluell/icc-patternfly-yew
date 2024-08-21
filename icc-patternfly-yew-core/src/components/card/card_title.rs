use yew::{
    prelude::*,
    virtual_dom::VTag,
};


pub struct CardTitle;

#[derive(Clone, PartialEq, Properties)]
pub struct CardTitleProperties
{
    /** Content rendered inside the CardTitle */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the CardTitle */
    #[prop_or_default]
    pub class_name: String,
    /** Sets the base component to render. defaults to div */
    #[prop_or(String::from("div"))]
    pub component: String,
}

impl Component for CardTitle
{
    type Message = ();
    type Properties = CardTitleProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let mut component = VTag::new(ctx.props().component.clone());

        // Build list of classes
        let mut classes = String::from("pf-v5-c-card__title");
        
        // Add extra classes specified on the parent
        if ctx.props().class_name.len() > 0
        {
            classes += " ";
            classes += &ctx.props().class_name;
        }

        component.add_attribute("class", classes);

        //     {...props}

        component.add_children(ctx.props().children.iter());

        component.into()
    }
}
