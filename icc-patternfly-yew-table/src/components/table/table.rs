use yew::prelude::*;

use super::*;

pub struct Table
{
    has_selectable_rows: bool,
}

pub enum TableMsg
{
    RegisterSelectableRow,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TableProps
{
    /** Adds an accessible name for the Table */
    #[prop_or_default]
    pub aria_label: Option<String>,
    /** Content rendered inside the Table */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the Table  */
    #[prop_or_default]
    pub classes: Classes,
    /**
     * Style variant for the Table
     * compact: Reduces spacing and makes the table more compact
     */
    #[prop_or_default]
    pub variant: Option<TableVariant>,
    /** Render borders */
    #[prop_or_default]
    pub borders: bool,
    /** Specifies the grid breakpoints  */
    #[prop_or(TableGridBreakpoint::GridMd)]
    pub grid_break_point: TableGridBreakpoint,
    /** A valid WAI-ARIA role to be applied to the table element */
    #[prop_or_default]
    pub role: Option<String>,
    /** If set to true, the table header sticks to the top of its container */
    #[prop_or_default]
    pub is_sticky_header: bool,
    // /** @hide Forwarded ref */
    // #[prop_or_default]
    // pub innerRef?: React.RefObject<any>;
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
    /** Flag indicating this table's rows will not have the inset typically reserved for expanding/collapsing rows in a tree table. Intended for use on tree tables with no visible rows with children. */
    #[prop_or_default]
    pub has_no_inset: bool,
    /** Collection of column spans for nested headers. Deprecated: see https://github.com/patternfly/patternfly/issues/4584 */
    #[prop_or_default]
    pub nested_header_column_spans: Vec<i32>,
    /** Visible text to add alongside the hidden a11y caption for tables with selectable rows. */
    #[prop_or_default]
    pub selectable_row_caption_text: Option<String>,
    /** Value to overwrite the randomly generated data-ouia-component-id.*/
    #[prop_or_default]
    pub ouia_id: Option<String>,
    /** Set the value of data-ouia-safe. Only set to true when the component is in a static state, i.e. no animations are occurring. At all other times, this value must be false. */
    #[prop_or_default]
    pub ouia_safe: bool,
}

impl Component for Table
{
    type Message = TableMsg;
    type Properties = TableProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self{
            has_selectable_rows: false,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            TableMsg::RegisterSelectableRow => {
                if !self.has_selectable_rows
                {
                    self.has_selectable_rows = true;
                }
            },
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let table_caption = if let Some(selectable_row_caption_text) = &ctx.props().selectable_row_caption_text {
            html!{
                <caption>
                    {selectable_row_caption_text}
                    <div class="pf-v5-screen-reader">
                        {"This table has selectable rows. It can be navigated by row using tab, and each row can be selected using
                        space or enter."}
                    </div>
                </caption>
            }
        } else {
            html!{
                <caption class="pf-v5-screen-reader">
                    {"This table has selectable rows. It can be navigated by row using tab, and each row can be selected using space
                    or enter."}
                </caption>
            }
        };

        html!{
            <ContextProvider<TableContext> 
                context={TableContext{
                    register_selectable_row: ctx.link().callback(|_| TableMsg::RegisterSelectableRow)
                }}
            >
                <table
                    aria_label={ctx.props().aria_label.clone()}
                    role={ctx.props().role.clone()}
                    class={classes!(
                        ctx.props().classes.clone(),
                        "pf-v5-c-table",
                        if ctx.props().is_tree_table {ctx.props().grid_break_point.get_tree_grid_class()} else {ctx.props().grid_break_point.get_class()},
                        if let Some(variant) = &ctx.props().variant {variant.get_class()} else {""},
                        if !ctx.props().borders {"pf-m-no-border-rows"} else {""},
                        if ctx.props().is_sticky_header {"pf-m-sticky-header"} else {""},
                        if ctx.props().is_tree_table {"pf-m-tree-view"} else {""},
                        if ctx.props().is_striped {"pf-m-striped"} else {""},
                        if ctx.props().is_expandable {"pf-m-expandable"} else {""},
                        if ctx.props().has_no_inset {"pf-m-no-inset"} else {""},
                        if ctx.props().is_nested {"pf-m-nested"} else {""},
                    )}
                    // ref={tableRef}
                    // {...(isTreeTable && { role: 'treegrid' })}
                    // {...ouiaProps}
                    // {...props}
                >
                    {
                        if self.has_selectable_rows
                        {
                            table_caption
                        }
                        else
                        {
                            html!{}
                        }
                    }
                    {for ctx.props().children.iter()}
                </table>
            </ContextProvider<TableContext>>
        }
    }
}