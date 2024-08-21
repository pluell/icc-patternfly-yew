use std::collections::HashMap;
use yew::{
    prelude::*,
    virtual_dom::VTag,
};

pub struct ProgressBar;

#[derive(Clone, PartialEq, Properties)]
pub struct ProgressBarProps
{
    /** What should be rendered inside progress bar. */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes for Progress bar. */
    #[prop_or_default]
    pub class_name: String,
    /** Actual progress value. */
    pub value: i32,
    /** Minimal value of progress. */
    #[prop_or_default]
    pub progress_bar_aria_props: HashMap<&'static str, String>,
    
    /** Extra properties */
    #[prop_or_default]
    pub role: String,
}

impl Component for ProgressBar
{
    type Message = ();
    type Properties = ProgressBarProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let mut div_container = VTag::new("div");

        div_container.add_attribute("role", ctx.props().role.clone());

        let classes = format!("pf-v5-c-progress__bar {}", &ctx.props().class_name);

        div_container.add_attribute("class", classes);

        // Add the aria props
        for (key, value) in ctx.props().progress_bar_aria_props.iter()
        {
            div_container.add_attribute(key, value.to_string());
        }

        div_container.add_child(html!{
            <div class="pf-v5-c-progress__indicator" style={format!("width: {}%", ctx.props().value)}>
                <span class="pf-v5-c-progress__measure">
                {
                    for ctx.props().children.iter()
                }
                </span>
            </div>
        });

        div_container.into()
    }
}
