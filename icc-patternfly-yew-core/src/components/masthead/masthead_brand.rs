use yew::{
    prelude::*,
    virtual_dom::VTag,
};


pub struct MastheadBrand;

#[derive(Clone, PartialEq, Properties)]
pub struct MastheadBrandProps
{
    /** Content rendered inside of the masthead brand. */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the masthead brand. */
    #[prop_or_default]
    pub class_name: String,
    /** Component type of the masthead brand. */
    #[prop_or(String::from("a"))]
    pub component: String,
}

impl Component for MastheadBrand
{
    type Message = ();
    type Properties = MastheadBrandProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let mut component = VTag::new(ctx.props().component.clone());

        // Build list of classes
        let mut classes = String::from("pf-v5-c-masthead__brand");
        
        // Add extra classes specified on the parent
        if ctx.props().class_name.len() > 0
        {
            classes += " ";
            classes += &ctx.props().class_name;
        }

        component.add_attribute("class", classes);

        component.add_attribute("tabIndex", "0");

        //     {...props}

        component.add_children(ctx.props().children.iter());

        component.into()
    }
}
