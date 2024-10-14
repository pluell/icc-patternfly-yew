use yew::prelude::*;

#[derive(Clone, PartialEq)] 
pub enum NotificationDrawerVariant
{
    Success,
    Danger,
    Warning,
    Info,
    Custom,
}

impl NotificationDrawerVariant
{
    pub fn view_icon(&self) -> Html
    {
        match self
        {
            Self::Success => {
                icc_patternfly_yew_icons::check_circle_icon!{}
            },
            Self::Danger => {
                icc_patternfly_yew_icons::exclamation_circle_icon!{}
            },
            Self::Warning => {
                icc_patternfly_yew_icons::exclamation_triangle_icon!{}
            },
            Self::Info => {
                icc_patternfly_yew_icons::info_circle_icon!{}
            },
            Self::Custom => {
                icc_patternfly_yew_icons::bell_icon!{}
            },
        }
    }

    pub fn class(&self) -> &'static str
    {
        match self
        {
            Self::Success => {
                "pf-m-success"
            },
            Self::Danger => {
                "pf-m-danger"
            },
            Self::Warning => {
                "pf-m-warning"
            },
            Self::Info => {
                "pf-m-info"
            },
            Self::Custom => {
                "pf-m-custom"
            },
        }
    }
}