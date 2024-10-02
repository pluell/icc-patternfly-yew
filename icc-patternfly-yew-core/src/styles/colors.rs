

#[derive(Clone, PartialEq)]
pub enum Theme
{
    Light,
    Dark,
}

impl Theme
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