use yew::{
    prelude::*,
};

use crate::components::{Button, ButtonVariant, ToolbarItem, ToolbarGroup};

use super::{ToolbarBreakpoint, get_toolbar_breakpoint};


pub struct ToolbarChipGroupContent
{
    link: ComponentLink<Self>,
    props: ToolbarChipGroupContentProperties,
}

pub enum ToolbarChipGroupContentMsg
{
    ClearChipGroups,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ToolbarChipGroupContentProperties
{
    /** Classes applied to root element of the data toolbar content row */
    #[prop_or_default]
    pub class_name: String,
    /** Flag indicating if a data toolbar toggle group's expandable content is expanded */
    #[prop_or_default]
    pub is_expanded: bool,
    // /** Chip group content reference for passing to data toolbar children */
    // chipGroupContentRef?: RefObject<any>;
    /** optional callback for clearing all filters in the toolbar */
    #[prop_or_default]
    pub clear_all_filters: Callback<()>,
    /** Flag indicating that the clear all filters button should be visible */
    #[prop_or_default]
    pub show_clear_filters_button: bool,
    /** Text to display in the clear all filters button */
    #[prop_or_default]
    pub clear_filters_button_text: String,
    /** Total number of filters currently being applied across all ToolbarFilter components */
    #[prop_or_default]
    pub number_of_filters: i32,
    /** The breakpoint at which the listed filters in chip groups are collapsed down to a summary */
    #[prop_or(ToolbarBreakpoint::None)]
    pub collapse_listed_filters_breakpoint: ToolbarBreakpoint,
}

impl Component for ToolbarChipGroupContent
{
    type Message = ToolbarChipGroupContentMsg;
    type Properties = ToolbarChipGroupContentProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self {
            link,
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
    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            ToolbarChipGroupContentMsg::ClearChipGroups => {
                self.props.clear_all_filters.emit(());

                false
            }
        }
    }

    fn view(&self) -> Html
    {
        let mut collapse_filters = false;
        if self.props.collapse_listed_filters_breakpoint == ToolbarBreakpoint::All
        {
            collapse_filters = true;
        }

        // Check if we need to collapse filters based on breakpoint
        if let Ok(inner_width_js) = web_sys::window().unwrap().inner_width()
        {
            if let Some(inner_width) = inner_width_js.as_f64()
            {
                if let Some(breakpoint_width) = get_toolbar_breakpoint(&self.props.collapse_listed_filters_breakpoint)
                {
                    collapse_filters = (inner_width as i32) < breakpoint_width;
                }
            }
        }

        let is_hidden = self.props.number_of_filters == 0 || self.props.is_expanded;

        html!{
            <div class=(
                    "pf-c-toolbar__content",
                    if is_hidden { "pf-m-hidden" } else { "" },
                    self.props.class_name.clone(),
                )
                hidden=is_hidden
            >
                <ToolbarGroup 
                    class_name={if collapse_filters {"pf-m-hidden"} else {""}}
                    hidden=collapse_filters
                />
                {
                    if collapse_filters && (self.props.number_of_filters > 0) && !self.props.is_expanded
                    {
                        html!{
                            <ToolbarGroup>
                                <ToolbarItem>{self.props.number_of_filters}{" filters applied"}</ToolbarItem>
                            </ToolbarGroup>
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
                {
                    if self.props.show_clear_filters_button && !self.props.is_expanded
                    {
                        html!{
                            <ToolbarItem>
                                <Button 
                                    variant=ButtonVariant::Link
                                    is_inline=true
                                    onclick=self.link.callback(|_| ToolbarChipGroupContentMsg::ClearChipGroups)
                                >
                                    { &self.props.clear_filters_button_text }
                                </Button>
                            </ToolbarItem>
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
            </div>
        }
    }
}
