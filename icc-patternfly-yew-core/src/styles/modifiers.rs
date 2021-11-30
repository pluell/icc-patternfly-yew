

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

impl Breakpoints
{
    pub fn get_class_appendix(&self) -> &'static str
    {
        match self
        {
            Breakpoints::Default => "",
            Breakpoints::Sm => "-on-sm",
            Breakpoints::Md => "-on-md",
            Breakpoints::Lg => "-on-lg",
            Breakpoints::Xl => "-on-xl",
            Breakpoints::Xxl => "-on-2xl",
        }
    }
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

        let breakpoint = self.breakpoint.get_class_appendix();

        format!("pf-m-{}{}", modifier, breakpoint)
    }
}

#[derive(Clone, PartialEq)]
pub enum Insets
{
    None,
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
    Xxxl,
}

impl Insets
{
    pub fn get_class_appendix(&self) -> &'static str
    {
        match self
        {
            Self::None => "-inset-none",
            Self::Xs => "-inset-xs",
            Self::Sm => "-inset-sm",
            Self::Md => "-inset-md",
            Self::Lg => "-inset-lg",
            Self::Xl => "-inset-xl",
            Self::Xxl => "-inset-2xl",
            Self::Xxxl => "-inset-3xl",
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct InsetModifer
{
    pub breakpoint: Breakpoints,
    pub inset: Insets,
}

impl InsetModifer
{
    pub fn get_class(&self) -> String
    {
        let inset = self.inset.get_class_appendix();

        let breakpoint = self.breakpoint.get_class_appendix();

        format!("pf-m{}{}", inset, breakpoint)
    }
}