use std::collections::{HashMap};
use yew::{
    prelude::*,
};

use super::*;


pub struct ProgressContainer
{
    props: ProgressContainerProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ProgressContainerProps
{
    /** Properties needed for aria support */
    #[prop_or_default]
    pub progress_bar_aria_props: HashMap<&'static str, String>,
    /** Progress component DOM ID. */
    pub parent_id: String,
    /** Progress title. */
    #[prop_or_default]
    pub title: Option<String>,
    /** Label to indicate what progress is showing. */
    #[prop_or_default]
    pub label: Option<Html>,
    /** Type of progress status. */
    #[prop_or_default]
    pub variant: Option<ProgressVariants>,
    /** Location of progress value. */
    #[prop_or(ProgressMeasureLocations::Top)]
    pub measure_location: ProgressMeasureLocations,
    /** Actual progress value. */
    pub value: i32,
    /** Whether title should be truncated */
    #[prop_or_default]
    pub is_title_truncated: bool,
    // /** Position of the tooltip which is displayed if title is truncated */
    // tooltipPosition?: 'auto' | 'top' | 'bottom' | 'left' | 'right';
}

impl Component for ProgressContainer
{
    type Message = ();
    type Properties = ProgressContainerProps;

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
        html!{
            <>
                // TODO: Add tooltip
                // {tooltip ? (
                //     <Tooltip position={tooltipPosition} content={tooltip} isVisible>
                //         {Title}
                //     </Tooltip>
                // ) : (
                // Title   
                // )}
                <div
                    class=classes!(
                        "pf-c-progress__description", 
                        if self.props.is_title_truncated { "pf-m-truncate" } else { "" },
                    )
                    id=format!("{}-description", self.props.parent_id)
                    aria-hidden="true"
                    // onMouseEnter={isTitleTruncated ? onMouseEnter : null}
                >
                {
                    if let Some(title) = &self.props.title
                    {
                        &title
                    }
                    else
                    {
                        ""
                    }
                }
                </div>
                <div class="pf-c-progress__status" aria-hidden="true">
                {
                    if self.props.measure_location == ProgressMeasureLocations::Top
                        || self.props.measure_location == ProgressMeasureLocations::Outside
                    {
                        html!{
                            <span class="pf-c-progress__measure">
                            {
                                if let Some(label) = &self.props.label
                                {
                                    label.clone()
                                }
                                else
                                {
                                    html!{format!("{}%", self.props.value)}
                                }
                            }
                            </span>
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
                {
                    if let Some(variant) = &self.props.variant
                    {
                        match variant
                        {
                            ProgressVariants::Danger => {
                                html!{
                                    <span class="pf-c-progress__status-icon">
                                        <i class="fas fa-fw fa-times-circle" aria-hidden="true"></i>
                                    </span>
                                }
                            },
                            ProgressVariants::Warning => {
                                html!{
                                    <span class="pf-c-progress__status-icon">
                                        <i class="fas fa-fw fa-exclamation-triangle" aria-hidden="true"></i>
                                    </span>
                                }
                            },
                            ProgressVariants::Success => {
                                html!{
                                    <span class="pf-c-progress__status-icon">
                                        <i class="fas fa-fw fa-check-circle" aria-hidden="true"></i>
                                    </span>
                                }
                            },
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
                </div>
                <ProgressBar 
                    role="progressbar" 
                    progress_bar_aria_props=self.props.progress_bar_aria_props.clone() 
                    value=self.props.value
                >
                {
                    if self.props.measure_location == ProgressMeasureLocations::Inside
                    {
                        format!("{}%", self.props.value)
                    }
                    else
                    {
                        String::new()
                    }
                }
                </ProgressBar>
            </>
        }
    }
}
