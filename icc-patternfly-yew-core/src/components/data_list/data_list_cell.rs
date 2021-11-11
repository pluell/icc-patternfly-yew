use yew::{
    prelude::*,
};

use crate::styles::{WrapModifers};


pub struct DataListCell
{
    props: DataListCellProps,
}

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
                    "pf-c-data-list__cell",
                    if self.props.width > 1 && self.props.width < 6 {format!{"pf-m-flex-{}", self.props.width}} else {String::new()},
                    if !self.props.is_filled { "pf-m-no-fill" } else { "" },
                    if self.props.align_right { "pf-m-align-right" } else { "" },
                    if self.props.is_icon { "pf-m-icon" } else { "" },
                    self.props.class_name.clone(),
                    if let Some(wrap_modifier) = &self.props.wrap_modifier { wrap_modifier.get_class() } else { "" },
                )
                // {...props}
            >
                { for self.props.children.iter() }
            </div>
        }
    }
}
