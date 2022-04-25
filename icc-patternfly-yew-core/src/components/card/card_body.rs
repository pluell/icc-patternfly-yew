use yew::{
    prelude::*,
    virtual_dom::{VTag},
};


pub struct CardBody;

#[derive(Clone, PartialEq, Properties)]
pub struct CardBodyProperties
{
    /** Content rendered inside the Card Body */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the Card Body */
    #[prop_or_default]
    pub class_name: String,
    /** Sets the base component to render. defaults to div */
    #[prop_or(String::from("div"))]
    pub component: String,
    /** Enables the body Content to fill the height of the card */
    #[prop_or(true)]
    pub is_filled: bool,
}

impl Component for CardBody
{
    type Message = ();
    type Properties = CardBodyProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let mut component = VTag::new(ctx.props().component.clone());

        // Build list of classes
        let mut classes = String::from("pf-c-card__body");

        if !ctx.props().is_filled { classes += " pf-m-no-fill" }
        
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
