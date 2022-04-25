use yew::{
    prelude::*,
    virtual_dom::{VTag},
};


#[derive(Clone, PartialEq)]
pub enum InputGroupTextVariant
{
    Default,
    Plain,
}

pub struct InputGroupText;

#[derive(Clone, PartialEq, Properties)]
pub struct InputGroupTextProperties
{
    /** Additional classes added to the input group text. */
    #[prop_or_default]
    pub class_name: String,
    /** Content rendered inside the input group text. */
    #[prop_or_default]
    pub children: Children,
    /** Component that wraps the input group text. */
    #[prop_or(String::from("span"))]
    pub component: String,
    /** Input group text variant */
    #[prop_or(InputGroupTextVariant::Default)]
    pub variant: InputGroupTextVariant,
}

impl Component for InputGroupText
{
    type Message = ();
    type Properties = InputGroupTextProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let mut component = VTag::new(ctx.props().component.clone());

        // Build list of classes
        let mut classes = String::from("pf-c-input-group__text");

        if ctx.props().variant == InputGroupTextVariant::Plain
        {
            classes += " pf-m-plain";
        }
        
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
