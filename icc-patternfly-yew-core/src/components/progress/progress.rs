use std::collections::{HashMap};
use yew::{
    prelude::*,
};

use crate::{utils_get_unique_id};

use super::*;


#[derive(Clone, PartialEq)]
pub enum ProgressSizes
{
    Sm,
    Md,
    Lg,
}

const PROGRESS_SIZE_CLASSES: &'static [&'static str] = &[
    "pf-m-sm",
    "",
    "pf-m-lg",
];


pub struct Progress;

#[derive(Clone, PartialEq, Properties)]
pub struct ProgressProps
{
    /** Classname for progress component. */
    #[prop_or_default]
    pub class_name: String,
    /** Size variant of progress. */
    #[prop_or(ProgressSizes::Md)]
    pub size: ProgressSizes,
    /** Where the measure percent will be located. */
    #[prop_or(ProgressMeasureLocations::Top)]
    pub measure_location: ProgressMeasureLocations,
    /** Status variant of progress. */
    #[prop_or_default]
    pub variant: Option<ProgressVariants>,
    /** Title above progress. */
    #[prop_or_default]
    pub title: Option<String>,
    /** Text description of current progress value to display instead of percentage. */
    #[prop_or_default]
    pub label: Option<Html>,
    /** Actual value of progress. */
    #[prop_or_default]
    pub value: f64,
    /** DOM id for progress component. */
    #[prop_or_default]
    pub id: String,
    /** Minimal value of progress. */
    #[prop_or(0.0)]
    pub min: f64,
    /** Maximum value of progress. */
    #[prop_or(100.0)]
    pub max: f64,
    /** Accessible text description of current progress value, for when value is not a percentage. Use with label. */
    #[prop_or_default]
    pub value_text: String,
    /** Indicate whether to truncate the title */
    #[prop_or_default]
    pub is_title_truncated: bool,
    // /** Position of the tooltip which is displayed if title is truncated */
    // tooltipPosition?: 'auto' | 'top' | 'bottom' | 'left' | 'right';
    /** Adds accessible text to the ProgressBar. Required when title not used and there is not any label associated with the progress bar */
    #[prop_or_default]
    pub aria_label: String,
    /** Associates the ProgressBar with it's label for accessibility purposes. Required when title not used */
    #[prop_or_default]
    pub aria_labelledby: String,
}

impl Component for Progress
{
    type Message = ();
    type Properties = ProgressProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let id = if ctx.props().id.len() > 0 {
            ctx.props().id.clone()
        } else {
            utils_get_unique_id(None)
        };

        let variant_cls = if let Some(variant) = &ctx.props().variant {
            PROGRESS_VARIANT_CLASSES[variant.clone() as usize]
        } else {
            ""
        };

        let measure_location_cls = match ctx.props().measure_location {
            ProgressMeasureLocations::Inside | ProgressMeasureLocations::Outside => {
                PROGRESS_MEASURE_LOCATION_CLASSES[ctx.props().measure_location.clone() as usize]
            },
            _ => ""
        };

        let progress_size_cls = if ctx.props().measure_location == ProgressMeasureLocations::Inside { 
            PROGRESS_SIZE_CLASSES[ProgressSizes::Lg as usize]
        } else {
            PROGRESS_SIZE_CLASSES[ctx.props().size.clone() as usize]
        };

        // Build map of aria properties for the progress bar
        let mut progress_bar_aria_props = HashMap::new();

        progress_bar_aria_props.insert("aria-valuemin", ctx.props().min.to_string());
        progress_bar_aria_props.insert("aria-valuenow", ctx.props().value.to_string());
        progress_bar_aria_props.insert("aria-valuemax", ctx.props().max.to_string());

        if ctx.props().title.is_some()
        {
            progress_bar_aria_props.insert("aria-labelledby", format!("{}-description", id));
        }
        else if ctx.props().aria_labelledby.len() > 0
        {
            progress_bar_aria_props.insert("aria-labelledby", ctx.props().aria_labelledby.clone());
        }

        if ctx.props().aria_label.len() > 0
        {
            progress_bar_aria_props.insert("aria-label", ctx.props().aria_label.clone());
        }
      
        if ctx.props().value_text.len() > 0
        {
            progress_bar_aria_props.insert("aria-valuetext", ctx.props().value_text.clone());
        }
      
        if ctx.props().title.is_none() && ctx.props().aria_labelledby.len() == 0 && ctx.props().aria_label.len() == 0
        {
            log::warn!(
              "One of aria-label or aria-labelledby properties should be passed when using the progress component without a title."
            );
        }

        // Make sure the value for the progress bar is never less than 0 and never greather than 100
        let scaled_value = std::cmp::min(100, std::cmp::max(0, (
                ((ctx.props().value - ctx.props().min) / (ctx.props().max - ctx.props().min)) * 100.0)as i32
            )
        );

        html!{
            <div
                // {...props}
                class={classes!(
                    "pf-c-progress",
                    variant_cls,
                    measure_location_cls,
                    progress_size_cls,
                    if ctx.props().title.is_none() { "pf-m-singleline" } else { "" },
                    &ctx.props().class_name
                )}
                id={id.clone()}
            >
                <ProgressContainer
                    parent_id={id.clone()}
                    value={scaled_value}
                    title={ctx.props().title.clone()}
                    label={ctx.props().label.clone()}
                    variant={ctx.props().variant.clone()}
                    measure_location={ctx.props().measure_location.clone()}
                    progress_bar_aria_props={progress_bar_aria_props}
                    is_title_truncated={ctx.props().is_title_truncated}
                    // tooltipPosition={tooltipPosition}
                />
            </div>
        }
    }
}
