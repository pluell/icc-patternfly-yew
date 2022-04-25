use yew::{
    prelude::*,
};

use crate::styles::{WrapModifers};


pub struct DataListCell;

#[derive(Clone, PartialEq, Properties)]
pub struct DataListCellProps
{
    /** Content rendered inside the DataList cell */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the DataList cell */
    #[prop_or_default]
    pub class_name: String,
    /** Width (from 1-5) to the DataList cell */
    #[prop_or(1)]
    pub width: i32,
    /** Enables the body Content to fill the height of the card */
    #[prop_or_default]
    pub is_filled: bool,
    /**  Aligns the cell content to the right of its parent. */
    #[prop_or_default]
    pub align_right: bool,
    /** Set to true if the cell content is an Icon */
    #[prop_or_default]
    pub is_icon: bool,
    /** Determines which wrapping modifier to apply to the DataListCell */
    #[prop_or_default]
    pub wrap_modifier: Option<WrapModifers>,

    /** Additional classes added to the DataList item Content Wrapper.  Children should be one ore more <DataListCell> nodes */
    /** Array of <DataListCell> nodes that are rendered one after the other. */
    #[prop_or_default]
    pub data_list_cells: ChildrenWithProps<DataListCell>,
    /** Id for the row */
    #[prop_or_default]
    pub row_id: Option<String>,
}

impl Component for DataListCell
{
    type Message = ();
    type Properties = DataListCellProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div
                class={classes!(
                    "pf-c-data-list__cell",
                    if ctx.props().width > 1 && ctx.props().width < 6 {format!{"pf-m-flex-{}", ctx.props().width}} else {String::new()},
                    if !ctx.props().is_filled { "pf-m-no-fill" } else { "" },
                    if ctx.props().align_right { "pf-m-align-right" } else { "" },
                    if ctx.props().is_icon { "pf-m-icon" } else { "" },
                    ctx.props().class_name.clone(),
                    if let Some(wrap_modifier) = &ctx.props().wrap_modifier { wrap_modifier.get_class() } else { "" },
                )}
                // {...props}
            >
                { for ctx.props().children.iter() }
            </div>
        }
    }
}
