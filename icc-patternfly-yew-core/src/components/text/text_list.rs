use yew::{
    prelude::*,
    virtual_dom::{VTag},
};


#[derive(Clone, PartialEq)]
pub enum TextListVariants
{
    Ul,
    Ol,
    Dl,
}

impl Into<VTag> for TextListVariants
{
    fn into(self) -> VTag
    {
        match self
        {
            Self::Ul => VTag::new("ul"),
            Self::Ol => VTag::new("ol"),
            Self::Dl => VTag::new("dl"),
        }
    }
}


pub struct TextList;

#[derive(Clone, PartialEq, Properties)]
pub struct TextListProperties
{
    /** Content rendered within the TextList */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the TextList */
    #[prop_or_default]
    pub class_name: String,
    /** The text list component */
    #[prop_or(TextListVariants::Ul)]
    pub component: TextListVariants,
}

impl Component for TextList
{
    type Message = ();
    type Properties = TextListProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let mut component: VTag = ctx.props().component.clone().into();

        if ctx.props().class_name.len() > 0
        {
            component.add_attribute("class", ctx.props().class_name.clone());
        }

        component.add_attribute("data-pf-content", true.to_string());

        component.add_children(ctx.props().children.iter());

        component.into()
    }
}
