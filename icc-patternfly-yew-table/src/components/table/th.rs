use yew::prelude::*;

pub use super::base::{ThExpandType, ThInfoType, ThSelectType, ThSortType};
use super::{CellWidth, DecoratorReturnType, IColumn, IExtra, IExtraColumnData, TableModifier};
use super::utils::sortable;

pub struct Th;

#[derive(Clone, PartialEq, Properties)]
pub struct ThProps
{
    /**
     * The column header the cell corresponds to. Applicable when this component is used as a direct child to <Tr>.
     * This attribute replaces table header in mobile viewport. It is rendered by ::before pseudo element.
     */
    #[prop_or_default]
    pub data_label: Option<String>,
    /** Renders a checkbox select so that all row checkboxes can be selected/deselected */
    #[prop_or_default]
    pub select: Option<ThSelectType>,
    /** Renders a chevron so that all row chevrons can be expanded/collapsed */
    #[prop_or_default]
    pub expand: Option<ThExpandType>,
    /** Formats the header so that its column will be sortable */
    #[prop_or_default]
    pub sort: Option<ThSortType>,
    /**
     * Tooltip to show on the header cell.
     * Note: If the header cell is truncated and has simple string content, it will already attempt to display the header text.
     * If you want to show a tooltip that differs from the header text, you can set it here.
     * To disable it completely you can set it to null.
     */
    #[prop_or_default]
    pub tooltip: Option<Html>,
    // /** other props to pass to the tooltip */
    // tooltipProps?: Omit<TooltipProps, 'content'>;
    // /** Callback on mouse enter */
    // onMouseEnter?: (event: any) => void;
    /** Adds tooltip/popover info button */
    #[prop_or_default]
    pub info: Option<ThInfoType>,
    /** Adds scope to the column to associate header cells with data cells*/
    #[prop_or_default]
    pub scope: Option<String>,
    /** Indicates the header column should be sticky */
    #[prop_or_default]
    pub is_sticky_column: bool,
    /** Adds a border to the right side of the cell */
    #[prop_or_default]
    pub has_right_border: bool,
    /** Adds a border to the left side of the cell */
    #[prop_or_default]
    pub has_left_border: bool,
    /** Minimum width for a sticky column */
    #[prop_or_default]
    pub sticky_min_width: Option<i32>,
    /** Left offset of a sticky column. This will typically be equal to the combined value set by stickyMinWidth of any sticky columns that precede the current sticky column. */
    #[prop_or_default]
    pub sticky_left_offset: Option<i32>,
    /** Right offset of a sticky column. This will typically be equal to the combined value set by stickyMinWidth of any sticky columns that come after the current sticky column. */
    #[prop_or_default]
    pub sticky_right_offset: Option<i32>,
    /** Indicates the <th> is part of a subheader of a nested header */
    #[prop_or_default]
    pub is_subheader: bool,
    /** Visually hidden text accessible only via assistive technologies. This must be passed in if the
     * th is intended to be visually empty, and must be conveyed as a column header text.
     */
    #[prop_or_default]
    pub screen_reader_text: Option<String>,
    /** Provides an accessible name to the th. This should only be passed in when the th contains only non-text
     * content, such as a "select all" checkbox or "expand all" toggle.
     */
    #[prop_or_default]
    pub aria_label: Option<String>,

    // BaseCellProps
    /** Content rendered inside the cell */
    #[prop_or_default]
    pub children: Html,
    /** Additional classes added to the cell  */
    #[prop_or_default]
    pub classes: Classes,
    /** Element to render */
    #[prop_or(String::from("th"))]
    pub component: String,
    /** Modifies cell to center its contents. */
    #[prop_or_default]
    pub text_center: bool,
    /** Style modifier to apply */
    #[prop_or_default]
    pub modifier: Option<TableModifier>,
    /** Width percentage modifier */
    #[prop_or_default]
    pub width: Option<CellWidth>,
    // /** Visibility breakpoint modifiers */
    // visibility?: (keyof IVisibility)[];
    // /** Forwarded ref */
    // innerRef?: React.Ref<any>;
}

impl Component for Th
{
    type Message = ();
    type Properties = ThProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        // Compute sticky column styles
        let style = if ctx.props().is_sticky_column {
            let mut style_builder = String::new();

            if let Some(sticky_min_width) = &ctx.props().sticky_min_width
            {
                style_builder = format!("--pf-v5-c-table__sticky-column--MinWidth: {}", sticky_min_width);
            }

            if style_builder.len() > 0 { style_builder.push(';')};

            if let Some(sticky_left_offset) = &ctx.props().sticky_left_offset
            {
                style_builder.push_str(&format!("--pf-v5-c-table__sticky-column--Left: {}", sticky_left_offset));
            }

            Some(style_builder)
        } else {
            None
        };

        let sort_params = self.get_sort_params(ctx);

        let sort_classes = if let Some(sort_params) = &sort_params {
            sort_params.classes.clone()
        } else {
            None
        };

        // TODO: Convert th to MergedComponent
        html!{
            <th
                // tabIndex={sort || select || !truncated ? -1 : 0}
                // onFocus={tooltip !== null ? onMouseEnter : onMouseEnterProp}
                // onBlur={() => setShowTooltip(false)}
                data-label={ctx.props().data_label.clone()}
                // onMouseEnter={tooltip !== null ? onMouseEnter : onMouseEnterProp}
                scope={
                    if ctx.props().component == "th" {
                        ctx.props().scope.clone()
                    } else {
                        None
                    }
                }
                // ref={innerRef}
                aria_label={ctx.props().aria_label.clone()}
                class={classes!(
                    "pf-v5-c-table__th",
                    ctx.props().classes.clone(),
                    if ctx.props().text_center {"pf-m-center"} else {""},   
                    if ctx.props().is_subheader {"pf-v5-c-table__subhead"} else {""},
                    if ctx.props().is_sticky_column {"pf-v5-c-table__sticky-cell"} else {""},
                    if ctx.props().has_right_border {"pf-m-border-right"} else {""},
                    if ctx.props().has_left_border {"pf-m-border-left"} else {""},
                    if let Some(modifier) = &ctx.props().modifier {modifier.get_class()} else {""},
                    // mergedClassName
                    if let Some(width) = &ctx.props().width {width.get_class()} else {""},
                    sort_classes,
                )}
                // {...mergedProps}
                // {...props}
                {style}
            >
            {
                if let Some(sort_params) = &sort_params {
                    sort_params.children.clone()
                } else {
                    Some(ctx.props().children.clone())
                }
            }
            </th>
        }
    }
}

impl Th
{
    fn  get_sort_params(&self, ctx: &Context<Self>) -> Option<DecoratorReturnType>
    {
        if let Some(sort) = &ctx.props().sort
        {
            Some(sortable(&ctx.props().children, IExtra{
                extra_column_data: IExtraColumnData {
                    column_index: Some(sort.column_index),
                    column: Some(IColumn{
                        sort_by: Some(sort.sort_by.clone()),
                        onsort: sort.onsort.clone(),
                    }),
                    property: None,
                },
                ..Default::default()
            }))
        }
        else
        {
            None
        }
    }
}