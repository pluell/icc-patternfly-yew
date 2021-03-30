mod modal;
mod modal_box;
mod modal_box_body;
mod modal_box_close_button;
mod modal_box_description;
mod modal_box_footer;
mod modal_box_header;
mod modal_box_title;
mod modal_content;

pub use modal::*;
pub use modal_box::*;
pub use modal_box_body::*;
pub use modal_box_close_button::*;
pub use modal_box_footer::*;
pub use modal_box_header::*;
pub use modal_content::*;

use modal_box_description::*;
use modal_box_title::*;

use yew::{Html};

#[derive(Clone, PartialEq)]
pub enum ModalVariants
{
    Small,
    Medium,
    Large,
    Default,
}

#[derive(Clone, PartialEq)]
pub enum ModalTitleIconVariants
{
    Success,
    Danger,
    Warning,
    Info,
    Default,
    Custom(Html),
}
