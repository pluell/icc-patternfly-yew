mod title;

pub use title::*;


#[derive(Clone, PartialEq)]
pub enum TitleSizes
{
    Md,
    Lg,
    Xl,
    X2l,
    X3l,
    X4l,
}

impl TitleSizes
{
    fn get_class(&self) -> &'static str
    {
        match self
        {
            TitleSizes::Md => "pf-m-md",
            TitleSizes::Lg => "pf-m-lg",
            TitleSizes::Xl => "pf-m-xl",
            TitleSizes::X2l => "pf-m-2xl",
            TitleSizes::X3l => "pf-m-3xl",
            TitleSizes::X4l => "pf-m-4xl",
        }
    }
}

use yew::virtual_dom::VTag;

#[derive(Clone, PartialEq)]
pub enum TitleHeadingLevels
{
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl Into<VTag> for TitleHeadingLevels
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
        }
    }
}

impl TitleHeadingLevels
{
    fn get_default_size(&self) -> TitleSizes
    {
        match self
        {
            Self::H1 => TitleSizes::X2l,
            Self::H2 => TitleSizes::Xl,
            Self::H3 => TitleSizes::Lg,
            Self::H4 => TitleSizes::Md,
            Self::H5 => TitleSizes::Md,
            Self::H6 => TitleSizes::Md,
        }
    }
}