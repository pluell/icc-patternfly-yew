mod page;
mod page_breadcrumb;
mod page_context;
mod page_group;
mod page_navigation;
mod page_sidebar;
mod page_sidebar_body;
mod page_section;
mod page_toggle_button;

pub use page::*;
pub use page_breadcrumb::*;
pub use page_group::*;
pub use page_navigation::*;
pub use page_sidebar::*;
pub use page_sidebar_body::*;
pub use page_section::*;
pub use page_toggle_button::*;


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