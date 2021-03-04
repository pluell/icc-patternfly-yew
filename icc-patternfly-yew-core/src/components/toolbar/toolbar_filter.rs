use yew::{
    prelude::*,
};

use crate::components::{Chip, ChipGroup};
use super::*;


pub struct ToolbarFilter
{
    link: ComponentLink<Self>,
    props: ToolbarFilterProperties,
}

pub enum ToolbarFilterMsg
{
    DeleteChipGroup,
    DeleteChip(String),
}

#[derive(Clone, PartialEq, Properties)]
pub struct ToolbarFilterProperties
{
    #[prop_or_default]
    pub chips: Vec<String>,
    pub category_name: String,
    #[prop_or_default]
    pub delete_chip_group: Callback<String>,
    #[prop_or_default]
    pub delete_chip: Callback<(String, String)>,
    pub children: Children,
}

impl Component for ToolbarFilter
{
    type Message = ToolbarFilterMsg;
    type Properties = ToolbarFilterProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self {
            link,
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
            // TODO: This needs to be added to the ToolbarChipGroupContent node somehow
            {
                if self.props.chips.len() > 0
                {
                    html!{
                        <ToolbarItem
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
}
