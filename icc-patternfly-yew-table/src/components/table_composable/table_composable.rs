use yew::{
    prelude::*,
};

use crate::{TableGridBreakpoint, TableVariant};


pub struct TableComposable{
    has_selectable_rows: bool,
}

#[derive(Clone, PartialEq, Properties)]
pub struct BaseCellProps
{
    /** Content rendered inside the cell */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the cell  */
    #[prop_or_default]
    pub class_name: String,
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
  

#[derive(Clone, PartialEq, Properties)]
pub struct TableComposableProps
{
    /** Adds an accessible name for the Table */
    #[prop_or_default]
    pub aria_label: Option<String>,
    /** Content rendered inside the Table */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the Table  */
    #[prop_or_default]
    pub class_name: String,
    /**
     * Style variant for the Table
     * compact: Reduces spacing and makes the table more compact
     */
    #[prop_or_default]
    pub variant: Option<TableVariant>,
    /** Render borders */
    #[prop_or(true)]
    pub borders: bool,
    /** Specifies the grid breakpoints  */
    #[prop_or(TableGridBreakpoint::GridMd)]
    pub grid_break_point: TableGridBreakpoint,
    /** A valid WAI-ARIA role to be applied to the table element */
    #[prop_or_default]
    pub role: Option<String>,
    // /** If set to true, the table header sticks to the top of its container */
    #[prop_or_default]
    pub is_sticky_header: bool,
    // /** Forwarded ref */
    // innerRef?: React.RefObject<any>;
    /** Flag indicating table is a tree table */
    #[prop_or_default]
    pub is_tree_table: bool,
    /** Flag indicating this table is nested within another table */
    #[prop_or_default]
    pub is_nested: bool,
    /** Flag indicating this table should be striped. This property works best for a single <tbody> table. Striping may also be done manually by applying this property to Tbody and Tr components. */
    #[prop_or_default]
    pub is_striped: bool,
    /** Flag indicating this table contains expandable rows to maintain proper striping */
    #[prop_or_default]
    pub is_expandable: bool,
    // /** Collection of column spans for nested headers. Deprecated: see https://github.com/patternfly/patternfly/issues/4584 */
    // nestedHeaderColumnSpans?: number[];
    /** Flag to apply a caption element with visually hidden instructions that improves a11y for tables with selectable rows. If this prop is set to true other caption elements should not be passed as children of this table, and you should instead use the selectableRowCaptionText prop. */
    #[prop_or_default]
    pub has_selectable_row_caption: bool,
    /** Visible text to add alongside the hidden a11y caption for tables with selectable rows. This prop must be used to add custom caption content to the table when the hasSelectableRowCaption prop is set to true. */
    #[prop_or_default]
    pub selectable_row_caption_text: Option<String>,
    // /** Value to overwrite the randomly generated data-ouia-component-id.*/
    // ouiaId?: number | string;
    /** Set the value of data-ouia-safe. Only set to true when the component is in a static state, i.e. no animations are occurring. At all other times, this value must be false. */
    #[prop_or_default]
    pub ouia_safe: bool,
}

impl Component for TableComposable
{
    type Message = ();
    type Properties = TableComposableProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self{
            has_selectable_rows: false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let table_caption = if let Some(selectable_row_caption_text) = &ctx.props().selectable_row_caption_text {
            html!{
                <caption>
                    {selectable_row_caption_text}
                    <div class="pf-screen-reader">
                        {"This table has selectable rows. It can be navigated by row using tab, and each row can be selected using
                        space or enter."}
                    </div>
                </caption>
            }
        } else {
            html!{
                <caption class="pf-screen-reader">
                    {"This table has selectable rows. It can be navigated by row using tab, and each row can be selected using space
                    or enter."}
                </caption>
            }
        };

        html!{
            // <TableComposableContext.Provider value={{ registerSelectableRow }}>
                <table
                    aria-label={ctx.props().aria_label.clone()}
                    role={ctx.props().role.clone()}
                    class={classes!(
                        &ctx.props().class_name,
                        "pf-c-table",
                        if ctx.props().is_tree_table {"treeGrid"} else {ctx.props().grid_break_point.get_class()},
                        if let Some(variant) = &ctx.props().variant {variant.get_class()} else {""},
                        if !ctx.props().borders {"pf-m-no-border-rows"} else {""},
                        if ctx.props().is_sticky_header {"pf-m-sticky-header"} else {""},
                        if ctx.props().is_tree_table {"pf-m-tree-view"} else {""},
                        if ctx.props().is_striped {"pf-m-striped"} else {""},
                        if ctx.props().is_expandable {"pf-m-expandable"} else {""},
                        if ctx.props().is_nested {"pf-m-nested"} else {""},
                    )}
                    // ref={tableRef}
                    // {...(isTreeTable && { role: 'treegrid' })}
                    // {...ouiaProps}
                    // {...props}
                >
                {
                    if ctx.props().has_selectable_row_caption && self.has_selectable_rows
                    {
                        table_caption
                    }
                    else
                    {
                        html!{}
                    }
                }
                { for ctx.props().children.iter() }
                </table>
            // </TableComposableContext.Provider>
        }
    }
}