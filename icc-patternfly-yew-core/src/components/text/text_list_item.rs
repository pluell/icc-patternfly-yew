use yew::{
    prelude::*,
    virtual_dom::VTag,
};


#[derive(Clone, PartialEq)]
pub enum TextListItemVariants
{
    Li,
    Dt,
    Dd,
}

impl Into<VTag> for TextListItemVariants
{
    fn into(self) -> VTag
    {
        match self
        {
            Self::Li => VTag::new("li"),
            Self::Dt => VTag::new("dt"),
            Self::Dd => VTag::new("dd"),
        }
    }
}


pub struct TextListItem;

#[derive(Clone, PartialEq, Properties)]
pub struct TextListItemProperties
{
    /** Content rendered within the TextListItem */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the TextListItem */
    #[prop_or_default]
    pub class_name: String,
    /** The text list item component */
    #[prop_or(TextListItemVariants::Li)]
    pub component: TextListItemVariants,
}

impl Component for TextListItem
{
    type Message = ();
    type Properties = TextListItemProperties;

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
