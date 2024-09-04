use std::collections::HashMap;

use yew::prelude::*;

use super::*;


pub struct Toolbar
{
    chip_group_content_ref: NodeRef,
    is_managed_toggle_expanded: bool,
    filter_info: HashMap<String, i32>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ToolbarProperties
{
    /** Optional callback for clearing all filters in the toolbar */
    #[prop_or_default]
    pub clear_all_filters: Callback<()>,
    /** Text to display in the clear all filters button */
    #[prop_or_default]
    pub clear_filters_button_text: String,
    // /** Custom content appended to the filter generated chip group. To maintain spacing and styling, each node should be wrapped in a ToolbarItem or ToolbarGroup. This property will remove the default "Clear all filters" button. */
    // customChipGroupContent?: React.ReactNode;
    // /** The breakpoint at which the listed fitlers in chip groups are collapsed down to a summary */
    #[prop_or_default]
    pub collapse_listed_filters_breakpoint: Option<ToolbarBreakpoint>,
    /** Flag indicating if a data toolbar toggle group's expandable content is expanded */
    #[prop_or_default]
    pub is_expanded: bool,
    /** A callback for setting the isExpanded flag */
    #[prop_or_default]
    pub toggle_is_expanded: Option<Callback<()>>,
    /** Classes applied to root element of the data toolbar */
    #[prop_or_default]
    pub classes: Classes,
    /** Content to be rendered as rows in the data toolbar */
    pub children: ChildrenWithProps<ToolbarContent>,
    /** Id of the data toolbar */
    #[prop_or_default]
    pub id: String,
    /** Flag indicating the toolbar height should expand to the full height of the container */
    #[prop_or_default]
    pub is_full_height: bool,
    /** Flag indicating the toolbar is static */
    #[prop_or_default]
    pub is_static: bool,
    /** Flag indicating the toolbar should use the Page insets */
    #[prop_or_default]
    pub use_page_insets: bool,
    /** Flag indicating the toolbar should stick to the top of its container */
    #[prop_or_default]
    pub is_sticky: bool,
    // /** Insets at various breakpoints. */
    // inset?: {
    //     default?: 'insetNone' | 'insetSm' | 'insetMd' | 'insetLg' | 'insetXl' | 'inset2xl';
    //     sm?: 'insetNone' | 'insetSm' | 'insetMd' | 'insetLg' | 'insetXl' | 'inset2xl';
    //     md?: 'insetNone' | 'insetSm' | 'insetMd' | 'insetLg' | 'insetXl' | 'inset2xl';
    //     lg?: 'insetNone' | 'insetSm' | 'insetMd' | 'insetLg' | 'insetXl' | 'inset2xl';
    //     xl?: 'insetNone' | 'insetSm' | 'insetMd' | 'insetLg' | 'insetXl' | 'inset2xl';
    //     '2xl'?: 'insetNone' | 'insetSm' | 'insetMd' | 'insetLg' | 'insetXl' | 'inset2xl';
    // };
    /** Text to display in the total number of applied filters ToolbarFilter */
    #[prop_or_default]
    pub number_of_filters_text: Option<fn(i32) -> String>, // (numberOfFilters: number) => string;
    // /** Value to overwrite the randomly generated data-ouia-component-id.*/
    // ouiaId?: number | string;
    // /** Set the value of data-ouia-safe. Only set to true when the component is in a static state, i.e. no animations are occurring. At all other times, this value must be false. */
    // ouiaSafe?: boolean;
}

pub enum ToolbarMsg
{
    UpdateNumFilters((String, i32)),
}

impl Component for Toolbar
{
    type Message = ToolbarMsg;
    type Properties = ToolbarProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self {
            chip_group_content_ref: NodeRef::default(),
            is_managed_toggle_expanded: false,
            filter_info: HashMap::new(),
        }
    }

    /// Called everytime when messages are received
    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            ToolbarMsg::UpdateNumFilters((group_name, num_chips)) => {
                self.filter_info.insert(group_name, num_chips);
            },
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        // Calculate number of filters
        let number_of_filters = self.get_num_filters();

        let is_expanded = if self.is_toggle_managed(ctx) { self.is_managed_toggle_expanded } else { ctx.props().is_expanded };

        let update_number_filters = ctx.link().callback(|(group_name, num_chips)| 
            ToolbarMsg::UpdateNumFilters((group_name, num_chips))
        );

        html!{
            <div
                id={ctx.props().id.clone()}
                class={classes!(
                    "pf-v5-c-toolbar",
                    if ctx.props().is_full_height {"pf-m-full-height"} else {""},
                    if ctx.props().is_static {"pf-m-static"} else {""},
                    if ctx.props().use_page_insets {"pf-m-page-insets"} else {""},
                    if ctx.props().is_sticky {"pf-m-sticky"} else {""},
                    // formatBreakpointMods(inset, styles, '', getBreakpoint(width)),
                    ctx.props().classes.clone(),
                )}
            >
                <ContextProvider<ToolbarContext> context={ToolbarContext{
                    is_expanded,
                    chip_group_content_ref: self.chip_group_content_ref.clone(),
                    update_number_filters,
                    number_of_filters,
                }}>
                    {for ctx.props().children.iter()}
                    <ToolbarChipGroupContent 
                        {is_expanded}
                        chip_group_content_ref={self.chip_group_content_ref.clone()}
                        clear_all_filters={ctx.props().clear_all_filters.clone()}
                        show_clear_filters_button={number_of_filters > 0}
                        clear_filters_button_text={ctx.props().clear_filters_button_text.clone()}
                        {number_of_filters}
                        number_of_filters_text={ctx.props().number_of_filters_text.clone()}
                        collapse_listed_filters_breakpoint={ctx.props().collapse_listed_filters_breakpoint.clone()}
                        // customChipGroupContent={customChipGroupContent}
                    />
                </ContextProvider<ToolbarContext>>
            </div>
        }
    }
}

impl Toolbar
{
    fn is_toggle_managed(&self, ctx: &Context<Self>) -> bool
    {
        !(ctx.props().is_expanded || ctx.props().toggle_is_expanded.is_some())
    }

    fn get_num_filters(&self) -> i32
    {
        let mut num_filters = 0;

        for (_, num_chips) in self.filter_info.iter()
        {
            num_filters += num_chips;
        }

        num_filters
    }
}
