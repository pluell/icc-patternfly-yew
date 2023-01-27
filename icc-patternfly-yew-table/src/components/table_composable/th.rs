use yew::{
    prelude::*,
};


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
    // /** Renders a checkbox select so that all row checkboxes can be selected/deselected */
    // #[prop_or_default]
    // select?: ThSelectType;
    // /** Renders a chevron so that all row chevrons can be expanded/collapsed */
    // #[prop_or_default]
    // expand?: ThExpandType;
    // /** Formats the header so that its column will be sortable */
    // #[prop_or_default]
    // sort?: ThSortType;
    /**
     * Tooltip to show on the header cell.
     * Note: If the header cell is truncated and has simple string content, it will already attempt to display the header text.
     * If you want to show a tooltip that differs from the header text, you can set it here.
     * To disable it completely you can set it to null.
     */
    #[prop_or_default]
    pub tooltip: Option<Html>,
    // /** Callback on mouse enter */
    // #[prop_or_default]
    // onMouseEnter?: (event: any) => void;
    // /** Adds tooltip/popover info button */
    // #[prop_or_default]
    // pub info?: ThInfoType;
    /** Adds scope to the column to associate header cells with data cells*/
    #[prop_or_default]
    pub scope: Option<String>,
    /** Indicates the header column should be sticky */
    #[prop_or_default]
    pub is_sticky_column: bool,
    /** Adds a border to the right side of the cell */
    #[prop_or_default]
    pub has_right_border: bool,
    /** Minimum width for a sticky column */
    #[prop_or_default]
    pub sticky_min_width: Option<i32>,
    /** Left offset of a sticky column. This will typically be equal to the combined value set by stickyMinWidth of any sticky columns that precede the current sticky column. */
    #[prop_or_default]
    pub sticky_left_offset: Option<i32>,
    /** Indicates the <th> is part of a subheader of a nested header */
    #[prop_or_default]
    pub is_subheader: bool,

    // BaseCellProps
    /** Content rendered inside the cell */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the cell  */
    #[prop_or_default]
    pub class_name: String,
    /** Element to render */
    #[prop_or(String::from("th"))]
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
                style_builder = format!("--pf-c-table__sticky-column--MinWidth: {}", sticky_min_width);
            }

            if style_builder.len() > 0 { style_builder.push(';')};

            if let Some(sticky_left_offset) = &ctx.props().sticky_left_offset
            {
                style_builder.push_str(&format!("--pf-c-table__sticky-column--Left: {}", sticky_left_offset));
            }

            Some(style_builder)
        } else {
            None
        };

        // TODO: Convert th to MergedComponent
        html!{
            <th
                data-label={ctx.props().data_label.clone()}
                // onMouseEnter={tooltip !== null ? onMouseEnter : onMouseEnterProp}
                scope={
                    if ctx.props().component == "th" && ctx.props().children.len() > 0 {
                        ctx.props().scope.clone()
                    } else {
                        None
                    }
                }
                // ref={innerRef}
                class={classes!(
                    &ctx.props().class_name,
                    // if ctx.props().textCenter && styles.modifiers.center,
                    if ctx.props().is_subheader {"pf-c-table__subhead"} else {""},
                    if ctx.props().is_sticky_column {"pf-c-table__sticky-column"} else {""},
                    if ctx.props().has_right_border {"pf-m-border-right"} else {""},
                    // modifier && styles.modifiers[modifier as 'breakWord' | 'fitContent' | 'nowrap' | 'truncate' | 'wrap'],
                    // mergedClassName
                )}
                // {...mergedProps}
                // {...props}
                style={style}
            >
                // TODO: Update this to use the transformed children list
                // {transformedChildren}
                { for ctx.props().children.iter() }
            </th>
        }
    }
}