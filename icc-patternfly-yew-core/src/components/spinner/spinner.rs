use yew::{
    prelude::*,
    virtual_dom::VTag,
};

use super::SpinnerSize;


const SPINNER_SIZE_CLASSES: &'static [&'static str] = &[
    "pf-m-sm",
    "pf-m-md",
    "pf-m-lg",
    "pf-m-xl",
];

pub struct Spinner;

#[derive(Clone, PartialEq, Properties)]
pub struct SpinnerProperties
{
    /** Additional classes added to the Spinner. */
    #[prop_or_default]
    pub classes: Classes,
    /** Size variant of progress. */
    #[prop_or(SpinnerSize::Xl)]
    pub size: SpinnerSize,
    /** Aria value text */
    #[prop_or_default]
    pub aria_valuetext: AttrValue,
    /** Whether to use an SVG (new) rather than a span (old) */
    #[prop_or_default]
    pub is_svg: bool,
    /** Diameter of spinner set as CSS variable */
    #[prop_or_default]
    pub diameter: AttrValue,
}

impl Component for Spinner
{
    type Message = ();
    type Properties = SpinnerProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        // Get the base component if using the new SVG or old span
        let mut component = if ctx.props().is_svg { VTag::new("svg") } else { VTag::new("span") };

        component.add_attribute("role", "progressbar".to_string());

        component.add_attribute("class", 
            classes!("pf-v5-c-spinner",
                SPINNER_SIZE_CLASSES[ctx.props().size.clone() as usize],
                ctx.props().classes.clone()
            ).to_string()
        );

        if ctx.props().aria_valuetext.len() > 0
        {
            component.add_attribute("aria-valuetext", ctx.props().aria_valuetext.to_string());
        }

        if ctx.props().diameter.len() > 0
        {
            component.add_attribute("style", format!("--pf-v5-c-spinner--diameter: {}", ctx.props().diameter));
        }

        if ctx.props().is_svg
        {
            component.add_attribute("viewBox", "0 0 100 100".to_string());

            component.add_child(html!{
                <circle class="pf-v5-c-spinner__path" cx="50" cy="50" r="45" fill="none" />
            });
        }
        else
        {
            component.add_child(html!{
                <>
                    <span class="pf-v5-c-spinner__clipper"></span>
                    <span class="pf-v5-c-spinner__lead-ball"></span>
                    <span class="pf-v5-c-spinner__tail-ball"></span>
                </>
            });
        }
        
        component.into()
    }
}
