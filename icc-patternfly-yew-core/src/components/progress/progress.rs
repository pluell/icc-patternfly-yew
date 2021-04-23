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


pub struct Progress
{
    props: ProgressProps,
}

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
        let id = if self.props.id.len() > 0 {
            self.props.id.clone()
        } else {
            utils_get_unique_id(None)
        };

        let variant_cls = if let Some(variant) = &self.props.variant {
            PROGRESS_VARIANT_CLASSES[variant.clone() as usize]
        } else {
            ""
        };

        let measure_location_cls = match self.props.measure_location {
            ProgressMeasureLocations::Inside | ProgressMeasureLocations::Outside => {
                PROGRESS_MEASURE_LOCATION_CLASSES[self.props.measure_location.clone() as usize]
            },
            _ => ""
        };

        let progress_size_cls = if self.props.measure_location == ProgressMeasureLocations::Inside { 
            PROGRESS_SIZE_CLASSES[ProgressSizes::Lg as usize]
        } else {
            PROGRESS_SIZE_CLASSES[self.props.size.clone() as usize]
        };

        // Build map of aria properties for the progress bar
        let mut progress_bar_aria_props = HashMap::new();

        progress_bar_aria_props.insert("aria-valuemin".to_string(), self.props.min.to_string());
        progress_bar_aria_props.insert("aria-valuenow".to_string(), self.props.value.to_string());
        progress_bar_aria_props.insert("aria-valuemax".to_string(), self.props.max.to_string());

        if self.props.title.is_some()
        {
            progress_bar_aria_props.insert(String::from("aria-labelledby"), format!("{}-description", id));
        }
        else if self.props.aria_labelledby.len() > 0
        {
            progress_bar_aria_props.insert(String::from("aria-labelledby"), self.props.aria_labelledby.clone());
        }

        if self.props.aria_label.len() > 0
        {
            progress_bar_aria_props.insert(String::from("aria-label"), self.props.aria_label.clone());
        }
      
        if self.props.value_text.len() > 0
        {
            progress_bar_aria_props.insert(String::from("aria-valuetext"), self.props.value_text.clone());
        }
      
        if self.props.title.is_none() && self.props.aria_labelledby.len() == 0 && self.props.aria_label.len() == 0
        {
            log::warn!(
              "One of aria-label or aria-labelledby properties should be passed when using the progress component without a title."
            );
        }

        // Make sure the value for the progress bar is never less than 0 and never greather than 100
        let scaled_value = std::cmp::min(100, std::cmp::max(0, (
                ((self.props.value - self.props.min) / (self.props.max - self.props.min)) * 100.0)as i32
            )
        );

        html!{
            <div
                // {...props}
                class=(
                    "pf-c-progress",
                    variant_cls,
                    measure_location_cls,
                    progress_size_cls,
                    if self.props.title.is_none() { "pf-m-singleline" } else { "" },
                    &self.props.class_name
                )
                id=&id
            >
                <ProgressContainer
                    parent_id=&id
                    value=scaled_value
                    title=self.props.title.clone()
                    label=&self.props.label
                    variant=self.props.variant.clone()
                    measure_location=self.props.measure_location.clone()
                    progress_bar_aria_props=progress_bar_aria_props
                    is_title_truncated=self.props.is_title_truncated
                    // tooltipPosition={tooltipPosition}
                />
            </div>
        }
    }
}
