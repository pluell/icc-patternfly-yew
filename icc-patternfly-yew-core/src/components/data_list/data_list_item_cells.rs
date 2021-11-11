use yew::{
    prelude::*,
    virtual_dom::{VChild},
};

use super::{DataListCell};



pub struct DataListItemCells
{
    props: DataListItemCellsProps,
}

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

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self
    {
        Self {
            props,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        if self.props != props
        {
            self.props = props;
            
            true
        }
        else
        {
            false
        }
    }

    /// Called everytime when messages are received
    fn update(&mut self, _: Self::Message) -> ShouldRender
    {
        false
    }

    fn view(&self) -> Html
    {
        html!{
            <div 
                class=classes!(
                    "pf-c-data-list__item-content",
                    self.props.class_name.clone(),
                )
                // {...props}
            >
                { self.props.data_list_cells.clone() }
            </div>
        }
    }
}
