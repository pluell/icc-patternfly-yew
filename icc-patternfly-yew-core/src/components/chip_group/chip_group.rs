use yew::prelude::*;

use crate::components::{Button, ButtonVariant, Chip};


pub struct ChipGroup
{
    is_open: bool,
}

pub enum ChipGroupMsg
{
    OnClickOverflowChip,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ChipGroupProperties
{
    /** Content rendered inside the chip group. Should be <Chip> elements. */
    pub children: Children,
    /** Additional classes added to the chip item */
    #[prop_or_default]
    pub classes: Classes,
    /** Flag for having the chip group default to expanded */
    #[prop_or_default]
    pub default_is_open: bool,
    /** Customizable "Show Less" text string */
    #[prop_or(Some(String::from("Show Less")))]
    pub expanded_text: Option<String>,
    /** Customizeable template string. Use variable "${remaining}" for the overflow chip count. */
    #[prop_or_default]
    pub collapsed_text: Option<String>,
    /** Category name text for the chip group category.  If this prop is supplied the chip group with have a label and category styling applied */
    #[prop_or_default]
    pub category_name: String,
    /** Aria label for chip group that does not have a category name */
    #[prop_or_default]
    pub aria_label: Option<String>,
    /** Set number of chips to show before overflow */
    #[prop_or(3)]
    pub num_chips: i32,
    /** Flag if chip group can be closed*/
    #[prop_or_default]
    pub is_closable: bool,
    /** Aria label for close button */
    #[prop_or_default]
    pub close_btn_aria_label: Option<String>,
    /** Function that is called when clicking on the chip group close button */
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    /** Function that is called when clicking on the overflow (expand/collapse) chip button */
    #[prop_or_default]
    pub on_overflow_chip_click: Callback<()>, //(event: React.MouseEvent) => void;
    // /** Position of the tooltip which is displayed if the category name text is longer */
    // #[prop_or_default]
    // tooltipPosition?:
    //     | TooltipPosition
    //     | 'auto'
    //     | 'top'
    //     | 'bottom'
    //     | 'left'
    //     | 'right'
    //     | 'top-start'
    //     | 'top-end'
    //     | 'bottom-start'
    //     | 'bottom-end'
    //     | 'left-start'
    //     | 'left-end'
    //     | 'right-start'
    //     | 'right-end';
    /** Value to overwrite the randomly generated data-ouia-component-id.*/
    #[prop_or_default]
    pub ouia_id: Option<String>,
}

impl Component for ChipGroup
{
    type Message = ChipGroupMsg;
    type Properties = ChipGroupProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self {
            is_open: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg : Self::Message) -> bool
    {
        match msg
        {
            ChipGroupMsg::OnClickOverflowChip => {
                self.is_open = !self.is_open;

                ctx.props().on_overflow_chip_click.emit(());

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div
                class={classes!(
                    "pf-v5-c-chip-group",
                    ctx.props().classes.clone(),
                    if ctx.props().category_name.is_empty() {""} else {"pf-m-category"},
                )}
                role="group"
                // {...(categoryName && { 'aria-labelledby': id })}
                // {...(!categoryName && { 'aria-label': ariaLabel })}
                // {...getOUIAProps(ChipGroup.displayName, ouiaId)}
            >
                <div class={"pf-v5-c-chip-group__main"}>
                    { &ctx.props().category_name }
                    <ul 
                        class={"pf-v5-c-chip-group__list"}
                        // {...(categoryName && { 'aria-labelledby': id })}
                        // {...(!categoryName && { 'aria-label': ariaLabel })}
                        role={"list"}
                    >
                    {
                        for ctx.props().children.iter().enumerate().filter(|(index, _)| self.is_open || index < &(ctx.props().num_chips as usize)).map(|(_, child)|
                            html!{
                                <li class={"pf-v5-c-chip-group__list-item"}>
                                {
                                    child.clone()
                                }
                                </li>
                            }
                        )
                    }
                    {
                        if ctx.props().children.len() as i32 > ctx.props().num_chips
                        {
                            html!{
                                <li class={"pf-v5-c-chip-group__list-item"}>
                                    <Chip
                                        is_overflow_chip=true
                                        onclick={ctx.link().callback(move |_| ChipGroupMsg::OnClickOverflowChip)}
                                        component="button"
                                    >
                                    {
                                        if self.is_open {
                                            ctx.props().expanded_text.clone().unwrap_or(String::new())
                                        } else {
                                            let remaining = ctx.props().children.len() as i32 - ctx.props().num_chips;
                                            ctx.props().collapsed_text.clone().unwrap_or(format!("{} more", remaining))
                                        }
                                    }
                                    </Chip>
                                </li>
                            }
                        }
                        else
                        {
                            html!{}
                        }
                    }
                    </ul>
                </div>
                {
                    if ctx.props().is_closable
                    {
                        html!{
                            <div class={"pf-v5-c-chip-group__close"}>
                                <Button
                                    variant={ButtonVariant::Plain}
                                    onclick={ctx.props().onclick.clone()}
                                    aria_label={ctx.props().close_btn_aria_label.clone()}
                                    // id={`remove_group_${id}`}
                                    // aria-labelledby={`remove_group_${id} ${id}`}
                                    // ouiaId={ouiaId || closeBtnAriaLabel}
                                >
                                    <i class={"fas fa-times"}></i>
                                </Button>
                            </div>
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
            </div>
        }
    }
}
