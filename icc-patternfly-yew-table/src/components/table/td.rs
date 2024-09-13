use yew::prelude::*;


pub struct Td;

#[derive(Clone, PartialEq, Properties)]
pub struct TdProps
// extends BaseCellProps, Omit<React.HTMLProps<HTMLTableDataCellElement>, 'onSelect' | 'width'>
{
    /**
     * The column header the cell corresponds to.
     * This attribute replaces table header in mobile viewport. It is rendered by ::before pseudo element.
     */
    #[prop_or_default]
    data_label: Option<String>,
    // /** Renders a checkbox or radio select */
    // #[prop_or_default]
    // select?: TdSelectType;
    // /** Turns the cell into an actions cell. Recommended to use an ActionsColumn component as a child of the Td rather than this prop. */
    // #[prop_or_default]
    // actions?: TdActionsType;
    // /** Turns the cell into an expansion toggle and determines if the corresponding expansion row is open */
    // #[prop_or_default]
    // expand?: TdExpandType;
    // /** Turns the cell into a compound expansion toggle */
    // #[prop_or_default]
    // compoundExpand?: TdCompoundExpandType;
    // /** Turns the cell into a favorites cell with a star button */
    // #[prop_or_default]
    // favorites?: TdFavoritesType;
    // /** Turns the cell into the first cell in a tree table row */
    // #[prop_or_default]
    // treeRow?: TdTreeRowType;
    // /** Turns the cell into the first cell in a draggable row*/
    // #[prop_or_default]
    // draggableRow?: TdDraggableType;
    /** True to remove padding */
    #[prop_or_default]
    pub no_padding: bool,
    /** Applies pf-v5-c-table__action to td */
    #[prop_or_default]
    pub is_action_cell: bool,
    /**
     * Tooltip to show on the body cell.
     * Note: If the body cell is truncated and has simple string content, it will already attempt to display the cell text.
     * If you want to show a tooltip that differs from the cell text, you can set it here.
     * To disable it completely you can set it to null.
     */
    #[prop_or_default]
    pub tooltip: Option<Html>,
    // /** Callback on mouse enter */
    // #[prop_or_default]
    // onMouseEnter?: (event: any) => void;
    /** Indicates the column should be sticky */
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

    // BaseCellProps
    /** Content rendered inside the cell */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the cell  */
    #[prop_or_default]
    pub classes: Classes    ,
    /** Element to render */
    #[prop_or_default]
    pub component: String,
    /** Modifies cell to center its contents. */
    #[prop_or_default]
    pub text_center: bool,
    // /** Style modifier to apply */
    // modifier?: 'breakWord' | 'fitContent' | 'nowrap' | 'truncate' | 'wrap';
    // /** Width percentage modifier */
    // width?: 10 | 15 | 20 | 25 | 30 | 35 | 40 | 45 | 50 | 60 | 70 | 80 | 90 | 100;
    // /** Visibility breakpoint modifiers */
    // visibility?: (keyof IVisibility)[];
    // /** Forwarded ref */
    // innerRef?: React.Ref<any>;
}

impl Component for Td
{
    type Message = ();
    type Properties = TdProps;

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

        // TODO: Convert td to MergedComponent
        html!{
            <td
                // tabIndex={(select || !truncated) && modifier !== 'truncate' ? -1 : 0}
                // {...(!treeTableTitleCell && { 'data-label': dataLabel })}
                // onFocus={tooltip !== null ? onMouseEnter : onMouseEnterProp}
                // onBlur={() => setShowTooltip(false)}
                // onMouseEnter={tooltip !== null ? onMouseEnter : onMouseEnterProp}
                class={classes!(
                    "pf-v5-c-table__td",
                    ctx.props().classes.clone(),
                    if ctx.props().is_action_cell {"pf-v5-c-table__action"} else {""},
                    if ctx.props().text_center {"pf-m-center"} else {""},
                    if ctx.props().no_padding {"pf-m-no-padding"} else {""},
                    if ctx.props().is_sticky_column {"pf-v5-c-table__sticky-cell"} else {""},
                    if ctx.props().has_right_border {"pf-m-border-right"} else {""},
                    if ctx.props().has_left_border {"pf-m-border-left"} else {""},
                    // styles.modifiers[modifier as 'breakWord' | 'fitContent' | 'nowrap' | 'truncate' | 'wrap' | undefined],
                    // draggableParams && styles.tableDraggable,
                    // mergedClassName
                )}
                // ref={innerRef}
                // {...mergedProps}
                // {...props}
                {style}
            >
                // TODO: update to use mergedChildren when needed
                // {mergedChildren || children}
                { for ctx.props().children.iter() }
            </td>
        }
    }
}