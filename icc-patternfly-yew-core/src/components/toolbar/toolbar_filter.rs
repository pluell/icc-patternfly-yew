use yew::prelude::*;
use web_sys::Element;

use crate::components::{Chip, ChipGroup};
use super::*;


pub struct ToolbarFilter
{
    context: ToolbarContext,
    _context_listener: ContextHandle<ToolbarContext>,
    filter_item_node: NodeRef,
    num_chips: usize,
}

pub enum ToolbarFilterMsg
{
    Context(ToolbarContext),
    DeleteChipGroup,
    DeleteChip(String),
}

#[derive(Clone, PartialEq, Properties)]
pub struct ToolbarFilterProperties
{
    /** Flag indicating when toolbar toggle group is expanded for non-managed toolbar toggle groups. */
    #[prop_or_default]
    pub is_expanded: Option<bool>,
    /** An array of strings to be displayed as chips in the expandable content */
    #[prop_or_default]
    pub chips: Vec<String>,
    /** Callback passed by consumer used to close the entire chip group */
    #[prop_or_default]
    pub delete_chip_group: Callback<String>,
    /** Callback passed by consumer used to delete a chip from the chips[] */
    #[prop_or_default]
    pub delete_chip: Callback<(String, String)>,
    /** Customizable "Show Less" text string for the chip group */
    #[prop_or_default]
    pub chip_group_expanded_text: Option<String>,
    /** Customizeable template string for the chip group. Use variable "${remaining}" for the overflow chip count. */
    #[prop_or_default]
    pub chip_group_collapsed_text: Option<String>,
    /** Content to be rendered inside the data toolbar item associated with the chip group */
    pub children: Children,
    /** Unique category name to be used as a label for the chip group */
    pub category_name: String,
    /** Flag to show the toolbar item */
    #[prop_or(true)]
    pub show_toolbar_item: bool,
    /** Reference to a chip container created with a custom expandable content group, for non-managed multiple toolbar toggle groups. */
    #[prop_or_default]
    pub expandable_chip_container_ref: Option<NodeRef>,
}

impl Component for ToolbarFilter
{
    type Message = ToolbarFilterMsg;
    type Properties = ToolbarFilterProperties;

    fn create(ctx: &Context<Self>) -> Self
    {
        let (context, context_listener) = ctx
            .link()
            .context(ctx.link().callback(ToolbarFilterMsg::Context))
            .expect("No Message Context Provided");

        // Set the number of filters
        context.update_number_filters.emit((ctx.props().category_name.clone(), ctx.props().chips.len() as i32));

        Self {
            context,
            _context_listener: context_listener,
            filter_item_node: NodeRef::default(),
            num_chips: ctx.props().chips.len(),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool
    {
        // Update the number of filters if the chips array has changed
        if self.num_chips != ctx.props().chips.len()
        {
            self.num_chips = ctx.props().chips.len();
            
            self.context.update_number_filters.emit((ctx.props().category_name.clone(), ctx.props().chips.len() as i32));
        }

        true
    }

    /// Called everytime when messages are received
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            ToolbarFilterMsg::Context(context) => {
                self.context = context;
                true
            }
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
        let chip_group = if ctx.props().chips.len() > 0
        {
            html!{
                <ToolbarItem
                    filter_item_node={&self.filter_item_node}
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
                                    { chip.to_string() }
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
        };

        html!{
            <>
            {
                if ctx.props().show_toolbar_item
                {
                    html!{
                        <ToolbarItem>
                            { ctx.props().children.clone() }
                        </ToolbarItem>
                    }
                }
                else
                {
                    html!{}
                }
            }
            {
                if let Some(chip_group_content_node) = self.context.chip_group_content_ref.cast::<Element>()
                {
                    // The filter toolbar group should be the first node of the chip content group
                    if let Some(group_node) = chip_group_content_node.first_element_child()
                    {
                        create_portal(chip_group, group_node)
                    }
                    else
                    {
                        html!{}
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
}
