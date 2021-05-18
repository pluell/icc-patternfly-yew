use std::collections::{HashMap};
use yew::{
    prelude::*,
    virtual_dom::{VTag},
};

pub struct ProgressBar
{
    props: ProgressBarProps,
}

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

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self
    {
        Self {
            props,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        if self.props != props
        {
            self.props = props;
            
            true
        }
        else
        {
            false
        }
    }

    /// Called everytime when messages are received
    fn update(&mut self, _: Self::Message) -> ShouldRender
    {
        false
    }

    fn view(&self) -> Html
    {
        let mut div_container = VTag::new("div");

        div_container.add_attribute("role", self.props.role.clone());

        let classes = format!("pf-c-progress__bar {}", &self.props.class_name);

        div_container.add_attribute("class", classes);

        // Add the aria props
        for (key, value) in self.props.progress_bar_aria_props.iter()
        {
            div_container.add_attribute(key, value.to_string());
        }

        div_container.add_child(html!{
            <div class="pf-c-progress__indicator" style=format!("width: {}%", self.props.value)>
                <span class="pf-c-progress__measure">
                {
                    for self.props.children.iter()
                }
                </span>
            </div>
        });

        div_container.into()
    }
}
