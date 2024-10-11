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

impl std::fmt::Display for TextVariants
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self
        {
            Self::H1 => write!(f, "h1"),
            Self::H2 => write!(f, "h2"),
            Self::H3 => write!(f, "h3"),
            Self::H4 => write!(f, "h4"),
            Self::H5 => write!(f, "h5"),
            Self::H6 => write!(f, "h6"),
            Self::P => write!(f, "p"),
            Self::A => write!(f, "a"),
            Self::Small => write!(f, "small"),
            Self::Blockquote => write!(f, "blockquote"),
            Self::Pre => write!(f, "pre"),
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
    pub children: Html,
    /** Additional classes added to the Text */
    #[prop_or_default]
    pub classes: Classes,
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
        html!{
            <@{ctx.props().component.to_string()}
                classes={ctx.props().classes.clone()}
                data-pf-content="true"
            >
                {ctx.props().children.clone()}
            </@>
        }
    }
}
