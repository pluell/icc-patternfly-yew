use yew::{
    prelude::*,
};

use crate::styles::{WrapModifers};


#[derive(Clone, PartialEq)]
pub enum DataListGridBreakpoints
{
    None,
    Always,
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
}

impl DataListGridBreakpoints
{
    fn get_class(&self) -> &'static str
    {
        match self
        {
            DataListGridBreakpoints::None => "pf-m-grid-none",
            DataListGridBreakpoints::Always => "pf-m-grid",
            DataListGridBreakpoints::Sm => "pf-m-grid-sm",
            DataListGridBreakpoints::Md => "pf-m-grid-md",
            DataListGridBreakpoints::Lg => "pf-m-grid-lg",
            DataListGridBreakpoints::Xl => "pf-m-grid-xl",
            DataListGridBreakpoints::Xxl => "pf-m-grid-2xl",
        }
    }
}

pub struct DataList;

#[derive(Clone, PartialEq, Properties)]
pub struct DataListProps
{
    /** Content rendered inside the DataList list */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the DataList list */
    #[prop_or_default]
    pub class_name: String,
    /** Adds accessible text to the DataList list */
    #[prop_or_default]
    pub aria_label: String,
    /** Optional callback to make DataList selectable, fired when DataListItem selected */
    #[prop_or_default]
    pub onselectdatalistitem: Option<Callback<String>>,
    /** Id of DataList item currently selected */
    #[prop_or_default]
    pub selected_data_list_item_id: Option<String>,
    /** Flag indicating if DataList should have compact styling */
    #[prop_or_default]
    pub is_compact: bool,
    // /** Specifies the grid breakpoints  */
    #[prop_or(DataListGridBreakpoints::Md)]
    pub grid_breakpoint: DataListGridBreakpoints,
    // /** Determines which wrapping modifier to apply to the DataList */
    #[prop_or_default]
    pub wrap_modifier: Option<WrapModifers>,
}

impl Component for DataList
{
    type Message = ();
    type Properties = DataListProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <ul
                class={classes!(
                    "pf-c-data-list",
                    if ctx.props().is_compact { "pf-m-compact" } else { "" },
                    ctx.props().grid_breakpoint.get_class(),
                    if let Some(wrap_modifier) = &ctx.props().wrap_modifier { wrap_modifier.get_class() } else { "" },
                    // dragging && styles.modifiers.dragOver, 
                    ctx.props().class_name.clone()
                )}
                // style={props.style}
                // {...props}
                // {...dragProps}
                // ref={this.ref}
            >
                { for ctx.props().children.iter() }
            </ul>
        }
    }
}
