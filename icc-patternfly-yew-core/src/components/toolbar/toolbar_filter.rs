use yew::{
    prelude::*,
};

use crate::components::{Chip, ChipGroup};
use super::*;


pub struct ToolbarFilter
{
    link: ComponentLink<Self>,
    props: ToolbarFilterProperties,
    filter_item_node: NodeRef,
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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        props.update_number_filters.emit((props.category_name.clone(), props.chips.len() as i32));

        Self {
            link,
            props,
            filter_item_node: NodeRef::default(),
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        if self.props != props
        {
            // Update the number of filters if the chips array has changed
            if props.chips != self.props.chips
            {
                props.update_number_filters.emit((props.category_name.clone(), props.chips.len() as i32));
            }

            self.props = props;
            
            true
        }
        else
        {
            false
        }
    }

    /// Called everytime when messages are received
    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            ToolbarFilterMsg::DeleteChipGroup => {
                self.props.delete_chip_group.emit(self.props.category_name.clone());

                false
            },
            ToolbarFilterMsg::DeleteChip(chip) => {
                self.props.delete_chip.emit((self.props.category_name.clone(), chip));

                false
            },
        }
    }

    fn view(&self) -> Html
    {
        html!{
            <>
            <ToolbarItem>
                { self.props.children.clone() }
            </ToolbarItem>
            {
                if self.props.chips.len() > 0
                {
                    html!{
                        <ToolbarItem
                            ref=self.filter_item_node.clone()
                            variant=ToolbarItemVariant::ChipGroup
                        >
                            <ChipGroup
                                category_name=&self.props.category_name
                                is_closable=true
                                onclick=self.link.callback(|_| ToolbarFilterMsg::DeleteChipGroup)
                            >
                            {
                                for self.props.chips.iter().map(|chip| {
                                    let chip_msg = chip.clone();
                                    
                                    html!{
                                        <Chip
                                            onclick=self.link.callback(move |_| {
                                                ToolbarFilterMsg::DeleteChip(chip_msg.clone())
                                            })
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

    fn rendered(&mut self, _first_render: bool)
    {
        if let Some(chip_group_content_node) = self.props.chip_group_content_ref.get()
        {
            if let Some(group_node) = chip_group_content_node.first_child()
            {
                if let Some(filter_item_node) = self.filter_item_node.get()
                {
                    if self.props.chips.len() == 0
                    {
                        // Remove the filter item if 
                        group_node.remove_child(&filter_item_node)
                            .expect("Unable to remove filter toolbar item from chip group");
                    }
                    else
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
                                    // Update the current chip group node with our new filter item
                                    group_node.replace_child(&chip_group_node, &filter_item_node)
                                        .expect("Unable to update filter toolbar item in chip group");

                                    found_node = true;
                                    break; 
                                }
                            }
                        }

                        if !found_node
                        {
                            // Add the filter item to the chip content chip group
                            group_node.append_child(&filter_item_node)
                                .expect("Unable to add filter toolbar item to chip group");
                        }
                    }
                }
            }
        }
    }
}
