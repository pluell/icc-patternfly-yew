

#[derive(Clone, PartialEq)]
pub enum TableGridBreakpoint
{
    None,
    Grid,
    GridMd,
    GridLg,
    GridXl,
    Grid2xl,
}

impl TableGridBreakpoint
{
    pub fn get_class(&self) -> &'static str
    {
        match self
        {
            TableGridBreakpoint::None => "",
            TableGridBreakpoint::Grid => "pf-m-grid",
            TableGridBreakpoint::GridMd => "pf-m-grid-md",
            TableGridBreakpoint::GridLg => "pf-m-grid-lg",
            TableGridBreakpoint::GridXl => "pf-m-grid-xl",
            TableGridBreakpoint::Grid2xl => "pf-m-grid-2xl",
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum TableVariant
{
    Compact
}

impl TableVariant
{
    pub fn get_class(&self) -> &'static str
    {
        match self
        {
            TableVariant::Compact => "pf-m-compact",
        }
    }
}
