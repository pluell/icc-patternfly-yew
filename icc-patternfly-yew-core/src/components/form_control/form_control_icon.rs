use yew::prelude::*;


#[derive(Clone, PartialEq)]
pub enum FromControlIconStatus {
    Success,
    Error,
    Warning
}

impl FromControlIconStatus {
    fn get_class(&self) -> &'static str {
        match self {
            FromControlIconStatus::Success => "pf-m-success",
            FromControlIconStatus::Error => "pf-m-error",
            FromControlIconStatus::Warning => "pf-m-warning",
        }
    }
}

pub struct FormControlIcon;

#[derive(Clone, PartialEq, Properties)]
pub struct FormControlIconProps {
    /** Additional class names added to the text input icon wrapper. */
    #[prop_or_default]
    pub classes: Classes,
    /** A custom icon to render instead of a status icon. */
    #[prop_or_default]
    pub custom_icon: Option<Html>,
    /** The status icon to render. */
    #[prop_or_default]
    pub status: Option<FromControlIconStatus>,
}

impl Component for FormControlIcon {
    type Message = ();
    type Properties = FormControlIconProps;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <span
                class={classes!(
                    "pf-v5-c-form-control__icon",
                    if let Some(status) = &ctx.props().status {status.get_class()} else {""},
                    ctx.props().classes.clone()
                )} 
                // {...props}
            >
            {
                if let Some(custom_icon) = &ctx.props().custom_icon {
                    custom_icon.clone()
                } else if let Some(status) = &ctx.props().status {
                    match status {
                        FromControlIconStatus::Success => icc_patternfly_yew_icons::check_circle_icon!(),
                        FromControlIconStatus::Error => icc_patternfly_yew_icons::exclamation_circle_icon!(),
                        FromControlIconStatus::Warning => icc_patternfly_yew_icons::exclamation_triangle_icon!(),
                    }
                } else {
                    html!{}
                }
            }
            </span>
        }
    }
}
