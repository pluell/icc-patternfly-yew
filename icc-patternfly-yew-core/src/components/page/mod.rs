
mod page_sidebar;
mod page_sidebar_body;
mod page_section;

pub use page_sidebar::*;
pub use page_sidebar_body::*;
pub use page_section::*;


#[derive(Clone, PartialEq)]
pub enum PageTheme
{
    Light,
    Dark
}

impl PageTheme
{
    pub fn get_class(&self) -> &'static str
    {
        match self
        {
            Self::Light => "pf-m-light",
            Self::Dark => ""
        }
    }
}