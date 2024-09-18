

#[derive(Clone, PartialEq)]
pub enum CellWidth
{
    Width10,
    Width15,
    Width20,
    Width25,
    Width30,
    Width35,
    Width40,
    Width45,
    Width50,
    Width60,
    Width70,
    Width80,
    Width90,
    Width100
}

impl CellWidth
{
    pub fn get_class(&self) -> &'static str
    {
        match self
        {
            Self::Width10 => "pf-m-width-10",
            Self::Width15 => "pf-m-width-15",
            Self::Width20 => "pf-m-width-20",
            Self::Width25 => "pf-m-width-25",
            Self::Width30 => "pf-m-width-30",
            Self::Width35 => "pf-m-width-35",
            Self::Width40 => "pf-m-width-40",
            Self::Width45 => "pf-m-width-45",
            Self::Width50 => "pf-m-width-50",
            Self::Width60 => "pf-m-width-60",
            Self::Width70 => "pf-m-width-70",
            Self::Width80 => "pf-m-width-80",
            Self::Width90 => "pf-m-width-90",
            Self::Width100 => "pf-m-width-100"
        }
    }
}
