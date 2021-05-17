pub use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum AlertVariant
{
    Success,
    Danger,
    Warning,
    Info,
    Default,
}

impl std::fmt::Display for AlertVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self
        {
            AlertVariant::Success => {
                write!(f, "Success")
            },
            AlertVariant::Danger => {
                write!(f, "Danger")
            },
            AlertVariant::Warning => {
                write!(f, "Warning")
            },
            AlertVariant::Info => {
                write!(f, "Info")
            },
            AlertVariant::Default => {
                write!(f, "Default")
            },
        }
    }
}

impl AlertVariant
{
    pub(crate) fn view(&self) -> Html
    {
        match self
        {
            AlertVariant::Success => {
                icc_patternfly_yew_icons::check_circle_icon!{}
            },
            AlertVariant::Danger => {
                icc_patternfly_yew_icons::exclamation_circle_icon!{}
            },
            AlertVariant::Warning => {
                icc_patternfly_yew_icons::exclamation_triangle_icon!{}
            },
            AlertVariant::Info => {
                icc_patternfly_yew_icons::info_circle_icon!{}
            },
            AlertVariant::Default => {
                icc_patternfly_yew_icons::bell_icon!{}
            },
        }
    }

    pub(crate) fn class(&self) -> &str
    {
        match self
        {
            AlertVariant::Success => {
                "pf-m-success"
            },
            AlertVariant::Danger => {
                "pf-m-danger"
            },
            AlertVariant::Warning => {
                "pf-m-warning"
            },
            AlertVariant::Info => {
                "pf-m-info"
            },
            AlertVariant::Default => {
                ""
            },
        }
    }
}