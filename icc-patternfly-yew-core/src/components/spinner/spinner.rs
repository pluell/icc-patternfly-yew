use yew::{
    prelude::*,
    virtual_dom::{VTag},
};

use super::{SpinnerSize};


const SPINNER_SIZE_CLASSES: &'static [&'static str] = &[
    "pf-m-sm",
    "pf-m-md",
    "pf-m-lg",
    "pf-m-xl",
];

pub struct Spinner
{
    props: SpinnerProperties,
}

#[derive(Clone, PartialEq, Properties)]
pub struct SpinnerProperties
{
    /** Additional classes added to the Spinner. */
    #[prop_or_default]
    pub class_name: String,
    /** Size variant of progress. */
    #[prop_or(SpinnerSize::Xl)]
    pub size: SpinnerSize,
    /** Aria value text */
    #[prop_or_default]
    pub aria_valuetext: String,
    /** Whether to use an SVG (new) rather than a span (old) */
    #[prop_or_default]
    pub is_svg: bool,
    /** Diameter of spinner set as CSS variable */
    #[prop_or_default]
    pub diameter: String,
}

impl Component for Spinner
{
    type Message = ();
    type Properties = SpinnerProperties;

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
        // Get the base component if using the new SVG or old span
        let mut component = if self.props.is_svg { VTag::new("svg") } else { VTag::new("span") };

        component.add_attribute("role", "progressbar".to_string());

        component.add_attribute("class", 
            format!("pf-c-spinner {} {}",
                SPINNER_SIZE_CLASSES[self.props.size.clone() as usize],
                &self.props.class_name
        ));

        if self.props.aria_valuetext.len() > 0
        {
            component.add_attribute("aria-valuetext", self.props.aria_valuetext.to_string());
        }

        if self.props.diameter.len() > 0
        {
            component.add_attribute("style", format!("--pf-c-spinner--diameter: {}", self.props.diameter));
        }

        if self.props.is_svg
        {
            component.add_attribute("viewBox", "0 0 100 100".to_string());

            component.add_child(html!{
                <circle class="pf-c-spinner__path" cx="50" cy="50" r="45" fill="none" />
            });
        }
        else
        {
            component.add_child(html!{
                <>
                    <span class="pf-c-spinner__clipper"></span>
                    <span class="pf-c-spinner__lead-ball"></span>
                    <span class="pf-c-spinner__tail-ball"></span>
                </>
            });
        }
        
        component.into()
    }
}
