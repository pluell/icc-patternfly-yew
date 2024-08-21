use yew::{
    prelude::*,
    virtual_dom::VChild,
};

use super::DataListCell;



pub struct DataListItemCells;

#[derive(Clone, PartialEq, Properties)]
pub struct DataListItemCellsProps
{
    /** Additional classes added to the DataList item Content Wrapper.  Children should be one ore more <DataListCell> nodes */
    #[prop_or_default]
    pub class_name: String,
    /** Array of <DataListCell> nodes that are rendered one after the other. */
    #[prop_or_default]
    pub data_list_cells: Vec<VChild<DataListCell>>,
    /** Id for the row */
    #[prop_or_default]
    pub row_id: Option<String>,
}

impl Component for DataListItemCells
{
    type Message = ();
    type Properties = DataListItemCellsProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                class={classes!(
                    "pf-v5-c-data-list__item-content",
                    ctx.props().class_name.clone(),
                )}
                // {...props}
            >
                { ctx.props().data_list_cells.clone() }
            </div>
        }
    }
}
