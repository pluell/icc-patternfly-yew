use yew::{
    prelude::*,
    virtual_dom::VTag,
};


#[derive(Clone, PartialEq)]
pub enum TextVariants
{
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    P,
    A,
    Small,
    Blockquote,
    Pre,
}

impl Into<VTag> for TextVariants
{
    fn into(self) -> VTag
    {
        match self
        {
            Self::H1 => VTag::new("h1"),
            Self::H2 => VTag::new("h2"),
            Self::H3 => VTag::new("h3"),
            Self::H4 => VTag::new("h4"),
            Self::H5 => VTag::new("h5"),
            Self::H6 => VTag::new("h6"),
            Self::P => VTag::new("p"),
            Self::A => VTag::new("a"),
            Self::Small => VTag::new("small"),
            Self::Blockquote => VTag::new("blockquote"),
            Self::Pre => VTag::new("pre"),
        }
    }
}

pub struct Text;

#[derive(Clone, PartialEq, Properties)]
pub struct TextProperties
{
    /** The text component */
    #[prop_or(TextVariants::P)]
    pub component: TextVariants,
    /** Content rendered within the Text */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the Text */
    #[prop_or_default]
    pub class_name: String,
}

impl Component for Text
{
    type Message = ();
    type Properties = TextProperties;

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
