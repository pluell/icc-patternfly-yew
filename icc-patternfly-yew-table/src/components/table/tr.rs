    use yew::prelude::*;


pub struct Tr;

#[derive(Clone, PartialEq, Properties)]
pub struct TrProps
{
    /** Content rendered inside the <tr> row */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the <tr> row  */
    #[prop_or_default]
    pub classes: Classes,
    // /** Forwarded ref */
    // innerRef?: React.Ref<any>;
    /** Flag indicating the Tr is hidden */
    #[prop_or_default]
    pub is_hidden: bool,
    /** Only applicable to Tr within the Tbody: Makes the row expandable and determines if it's expanded or not.
     * To prevent column widths from responding automatically when expandable rows are toggled, the width prop must also be passed into either the th or td component
     */
    #[prop_or_default]
    pub is_expanded: Option<bool>,
    /** Only applicable to Tr within the Tbody: Whether the row is editable */
    #[prop_or_default]
    pub is_editable: bool,
    /** Flag which adds hover styles for the clickable table row */
    #[prop_or_default]
    pub is_clickable: bool,
    /** Flag indicating the row is selected - adds selected styling */
    #[prop_or_default]
    pub is_row_selected: bool,
    /** Flag indicating the row is striped */
    #[prop_or_default]
    pub is_striped: bool,
    /** Flag indicating the row will act as a border. This is typically used for a table with a nested and sticky header. */
    #[prop_or_default]
    pub is_border_row: bool,
    /** Flag indicating the row is controlling the expansion of another row. */
    #[prop_or_default]
    pub is_control_row: bool,
    // /** An event handler for the row */
    // onRowClick?: (event?: React.KeyboardEvent | React.MouseEvent) => void;
    /** Flag indicating that the row is selectable */
    #[prop_or_default]
    pub is_selectable: bool,
    /** Flag indicating the spacing offset of the first cell should be reset */
    #[prop_or_default]
    pub reset_offset: bool,
    /** Value to overwrite the randomly generated data-ouia-component-id.*/
    #[prop_or_default]
    pub ouia_id: Option<String>,
    /** Set the value of data-ouia-safe. Only set to true when the component is in a static state, i.e. no animations are occurring. At all other times, this value must be false. */
    #[prop_or_default]
    pub ouia_safe: bool,

    // Aria props
    #[prop_or_default]
    pub aria_label: Option<String>,
}

impl Component for Tr
{
    type Message = ();
    type Properties = TrProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let row_is_hidden = ctx.props().is_hidden || (ctx.props().is_expanded.is_some() && !ctx.props().is_expanded.unwrap());

        // const { registerSelectableRow } = React.useContext(TableComposableContext);

        let aria_label = if let Some(passed_aria_label) = &ctx.props().aria_label {
                Some(passed_aria_label.to_string())
            } else if ctx.props().is_selectable && !row_is_hidden {
                // registerSelectableRow();

                if ctx.props().is_row_selected {
                    Some(String::from("Row selected"))
                } else {
                    Some(String::new())
                }
            }
            else {
                None
            };

        html!{
            <tr
                class={classes!(
                    "pf-v5-c-table__tr",
                    ctx.props().classes.clone(),
                    if ctx.props().is_expanded.is_some() {"pf-m-expandable"} else {""},
                    if let Some(is_expanded) = ctx.props().is_expanded {if is_expanded {"pf-m-expanded"} else {""}} else {""},
                    if ctx.props().is_editable {"pf-m-inline-editable"} else {""},
                    if ctx.props().is_clickable {"pf-m-clickable"} else {""},
                    if ctx.props().is_row_selected {"pf-m-selected"} else {""},
                    if ctx.props().is_striped {"pf-m-striped"} else {""},
                    if ctx.props().is_border_row {"pf-m-border-row"} else {""},
                    if ctx.props().is_control_row {"pf-v5-c-table__control-row"} else {""},
                    if ctx.props().reset_offset {"pf-m-first-cell-offset-reset"} else {""},
                )}
                hidden={row_is_hidden}
                tab_index={if ctx.props().is_clickable {Some("0")} else {None}}
                aria_label={aria_label}
                // ref={innerRef}
                // {...(onRowClick && { onClick: onRowClick, onKeyDown })}
                // {...ouiaProps}
                // {...props}
            >
                { for ctx.props().children.iter() }
            </tr>
        }
    }
}