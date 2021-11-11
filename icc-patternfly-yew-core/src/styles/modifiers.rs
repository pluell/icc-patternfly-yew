

#[derive(Clone, PartialEq)]
pub enum WrapModifers
{
    NoWrap,
    Truncate,
    BreakWord,
}

impl WrapModifers
{
    pub fn get_class(&self) -> &'static str
    {
        match self
        {
            WrapModifers::NoWrap => "pf-m-nowrap",
            WrapModifers::Truncate => "pf-m-truncate",
            WrapModifers::BreakWord => "pf-m-break-word",
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Breakpoints
{
    Default,
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
}

#[derive(Clone, PartialEq)]
pub struct VisibilityModifer
{
    pub breakpoint: Breakpoints,
    pub is_visible: bool,
}

impl VisibilityModifer
{
    pub fn get_class(&self) -> String
    {
        let modifier = if self.is_visible {"visible"} else {"hidden"};

        let breakpoint = match self.breakpoint {
            Breakpoints::Default => "",
            Breakpoints::Sm => "-on-sm",
            Breakpoints::Md => "-on-md",
            Breakpoints::Lg => "-on-lg",
            Breakpoints::Xl => "-on-xl",
            Breakpoints::Xxl => "-on-2xl",
        };

        format!("pf-m-{}{}", modifier, breakpoint)
    }
}