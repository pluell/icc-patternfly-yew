mod progress;
mod progress_bar;
mod progress_container;

pub use progress::*;
pub use progress_bar::*;
pub use progress_container::*;


#[derive(Clone, PartialEq)]
pub enum ProgressMeasureLocations
{
    Outside,
    Inside,
    Top,
    None,
}

const PROGRESS_MEASURE_LOCATION_CLASSES: &'static [&'static str] = &[
    "pf-m-outside",
    "pf-m-inside",
    "",
    "",
];

#[derive(Clone, PartialEq)]
pub enum ProgressVariants
{
    Danger,
    Success,
    Warning
}

const PROGRESS_VARIANT_CLASSES: &'static [&'static str] = &[
    "pf-m-danger",
    "pf-m-success",
    "pf-m-warning",
];