use yew::prelude::*;

use crate::components::{Button, ButtonVariant, ToolbarItem, ToolbarGroup};

use super::{ToolbarBreakpoint, get_toolbar_breakpoint};


pub struct ToolbarChipGroupContent;

pub enum ToolbarChipGroupContentMsg
{
    ClearChipGroups,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ToolbarChipGroupContentProperties
{
    /** Classes applied to root element of the data toolbar content row */
    #[prop_or_default]
    pub classes: Classes,
    /** Flag indicating if a data toolbar toggle group's expandable content is expanded */
    #[prop_or_default]
    pub is_expanded: bool,
    /** Chip group content reference for passing to data toolbar children */
    pub chip_group_content_ref: NodeRef,
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
    /** Text to display in the total number of applied filters ToolbarFilter */
    #[prop_or_default]
    pub number_of_filters_text: Option<fn(i32) -> String>,
    /** The breakpoint at which the listed filters in chip groups are collapsed down to a summary */
    #[prop_or(Some(ToolbarBreakpoint::Lg))]
    pub collapse_listed_filters_breakpoint: Option<ToolbarBreakpoint>,
    // /** Custom additional content appended to the generated chips. To maintain spacing and styling, each node should be a ToolbarItem or ToolbarGroup. This property will remove the built in "Clear all filters" button. */
    // customChipGroupContent?: React.ReactNode;
}

impl Component for ToolbarChipGroupContent
{
    type Message = ToolbarChipGroupContentMsg;
    type Properties = ToolbarChipGroupContentProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    /// Called everytime when messages are received
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            ToolbarChipGroupContentMsg::ClearChipGroups => {
                ctx.props().clear_all_filters.emit(());

                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let mut collapse_filters = false;

        // Check if we need to collapse filters based on breakpoint
        if let Some(collapse_listed_filters_breakpoint) = &ctx.props().collapse_listed_filters_breakpoint
        {
            if collapse_listed_filters_breakpoint == &ToolbarBreakpoint::All
            {
                collapse_filters = true;
            }
            else
            {
                if let Ok(inner_width_js) = web_sys::window().unwrap().inner_width()
                {
                    if let Some(inner_width) = inner_width_js.as_f64()
                    {
                        if let Some(breakpoint_width) = get_toolbar_breakpoint(collapse_listed_filters_breakpoint)
                        {
                            collapse_filters = (inner_width as i32) < breakpoint_width;
                        }
                    }
                }
            }
        }

        let is_hidden = ctx.props().number_of_filters == 0 || ctx.props().is_expanded;

        let number_of_filters_text = ctx.props().number_of_filters_text.unwrap_or(|number_of_filters| format!("{} filters applied", number_of_filters));

        html!{
            <div class={classes!(
                    "pf-v5-c-toolbar__content",
                    if !is_hidden && !collapse_filters {"pf-m-chip-container"} else {""},   
                    if is_hidden { "pf-m-hidden" } else { "" },
                    ctx.props().classes.clone(),
                )}
                ref={&ctx.props().chip_group_content_ref}
                hidden={is_hidden}
            >
                <ToolbarGroup 
                    classes={classes!(if collapse_filters {"pf-m-hidden"} else {""})}
                    hidden={collapse_filters}
                />
                {
                    if collapse_filters && (ctx.props().number_of_filters > 0) && !ctx.props().is_expanded
                    {
                        html!{
                            <ToolbarGroup>
                                <ToolbarItem>{number_of_filters_text(ctx.props().number_of_filters)}</ToolbarItem>
                            </ToolbarGroup>
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
                {
                    if ctx.props().show_clear_filters_button && !ctx.props().is_expanded
                    {
                        html!{
                            <ToolbarItem>
                                <Button 
                                    variant={ButtonVariant::Link}
                                    is_inline={true}
                                    onclick={ctx.link().callback(|_| ToolbarChipGroupContentMsg::ClearChipGroups)}
                                >
                                {
                                    if ctx.props().clear_filters_button_text.len() > 0
                                    {
                                        &ctx.props().clear_filters_button_text
                                    }
                                    else
                                    {
                                        "Clear all filters"
                                    }
                                }
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
