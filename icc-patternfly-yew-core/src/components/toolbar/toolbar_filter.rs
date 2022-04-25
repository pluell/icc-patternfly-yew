use yew::{
    prelude::*,
};

use crate::components::{Chip, ChipGroup};
use super::*;


pub struct ToolbarFilter
{
    filter_item_node: NodeRef,
    num_chips: usize,
}

pub enum ToolbarFilterMsg
{
    DeleteChipGroup,
    DeleteChip(String),
}

#[derive(Clone, PartialEq, Properties)]
pub struct ToolbarFilterProperties
{
    /** An array of strings to be displayed as chips in the expandable content */
    #[prop_or_default]
    pub chips: Vec<String>,
    /** Callback passed by consumer used to close the entire chip group */
    #[prop_or_default]
    pub delete_chip_group: Callback<String>,
    /** Callback passed by consumer used to delete a chip from the chips[] */
    #[prop_or_default]
    pub delete_chip: Callback<(String, String)>,
    /** Content to be rendered inside the data toolbar item associated with the chip group */
    pub children: Children,
    /** Unique category name to be used as a label for the chip group */
    pub category_name: String,
    /** Chip group content reference for passing to data toolbar children */
    #[prop_or_default]
    pub chip_group_content_ref: NodeRef,
    #[prop_or_default]
    pub update_number_filters: Callback<(String, i32)>,
}

impl Component for ToolbarFilter
{
    type Message = ToolbarFilterMsg;
    type Properties = ToolbarFilterProperties;

    fn create(ctx: &Context<Self>) -> Self
    {
        // Set the number of filters
        ctx.props().update_number_filters.emit((ctx.props().category_name.clone(), ctx.props().chips.len() as i32));

        Self {
            filter_item_node: NodeRef::default(),
            num_chips: ctx.props().chips.len(),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool
    {
        // Update the number of filters if the chips array has changed
        if self.num_chips != ctx.props().chips.len()
        {
            self.num_chips = ctx.props().chips.len();
            
            ctx.props().update_number_filters.emit((ctx.props().category_name.clone(), ctx.props().chips.len() as i32));
        }

        true
    }

    /// Called everytime when messages are received
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            ToolbarFilterMsg::DeleteChipGroup => {
                ctx.props().delete_chip_group.emit(ctx.props().category_name.clone());

                false
            },
            ToolbarFilterMsg::DeleteChip(chip) => {
                ctx.props().delete_chip.emit((ctx.props().category_name.clone(), chip));

                false
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <>
            <ToolbarItem>
                { ctx.props().children.clone() }
            </ToolbarItem>
            {
                if ctx.props().chips.len() > 0
                {
                    html!{
                        <ToolbarItem
                            ref={self.filter_item_node.clone()}
                            variant={ToolbarItemVariant::ChipGroup}
                        >
                            <ChipGroup
                                category_name={ctx.props().category_name.clone()}
                                is_closable={true}
                                onclick={ctx.link().callback(|_| ToolbarFilterMsg::DeleteChipGroup)}
                            >
                            {
                                for ctx.props().chips.iter().map(|chip| {
                                    let chip_msg = chip.clone();
                                    
                                    html!{
                                        <Chip
                                            onclick={ctx.link().callback(move |_| {
                                                ToolbarFilterMsg::DeleteChip(chip_msg.clone())
                                            })}
                                        >
                                            { &chip }
                                        </Chip>
                                    }
                                })
                            }
                            </ChipGroup>
                        </ToolbarItem>
                    }
                }
                else
                {
                    html!{}
                }
            }
            </>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool)
    {
        if let Some(chip_group_content_node) = ctx.props().chip_group_content_ref.get()
        {
            // The filter toolbar group should be the first node of the chip content group
            if let Some(group_node) = chip_group_content_node.first_child()
            {
                if let Some(filter_item_node) = self.filter_item_node.get()
                {
                    let mut found_node = false;

                    let group_node_children = group_node.child_nodes();

                    // Get the number of children of the toolbar group node
                    let num_groups = group_node_children.length();

                    // Search the chip group items for our node
                    for i in 0..num_groups
                    {
                        if let Some(chip_group_node) = group_node_children.get(i)
                        {
                            if filter_item_node == chip_group_node
                            {
                                if ctx.props().chips.len() == 0
                                {
                                    // Remove the filter item if there are no more filter chips to display
                                    group_node.remove_child(&filter_item_node)
                                        .expect("Unable to remove filter toolbar item from chip group");
                                }
                                else
                                {
                                    // Update the current chip group node with our new filter item
                                    group_node.replace_child(&chip_group_node, &filter_item_node)
                                        .expect("Unable to update filter toolbar item in chip group");
                                }

                                found_node = true;
                                break; 
                            }
                        }
                    }

                    // Add the filter item to the chip content chip group if the
                    // filter item is new and there are filter chips
                    if !found_node && ctx.props().chips.len() > 0
                    {
                        group_node.append_child(&filter_item_node)
                            .expect("Unable to add filter toolbar item to chip group");
                    }
                }
            }
        }
    }
}
