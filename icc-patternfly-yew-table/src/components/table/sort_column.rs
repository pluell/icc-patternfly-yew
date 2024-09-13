use yew::prelude::*;

use icc_patternfly_yew_icons::{arrows_alt_v_icon, long_arrow_alt_down_icon, long_arrow_alt_up_icon};

use super::TableText;


#[derive(Clone, PartialEq)]
pub enum SortByDirection
{
    Asc,
    Desc,
}

pub struct SortColumn
{
    focused: bool,
}

#[derive(Clone, PartialEq, Properties)]
pub struct SortColumnProps
{
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub is_sorted_by: bool,
    #[prop_or_default]
    pub onsort: Option<Callback<()>>,
    #[prop_or_default]
    pub sort_direction: Option<SortByDirection>,
    #[prop_or_default]
    pub tooltip: Option<Html>,
    // #[prop_or_default]
    // pub tooltipProps?: Omit<TooltipProps, 'content'>;
    #[prop_or_default]
    pub tooltip_has_default_behavior: bool,
}

pub enum SortColumnMsg
{
    Focused(bool),
    OnSort,
}

impl Component for SortColumn
{
    type Message = SortColumnMsg;
    type Properties = SortColumnProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self {
            focused: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            SortColumnMsg::Focused(focused) => {
                self.focused = focused;
            },
            SortColumnMsg::OnSort => {
                if let Some(onsort) = &ctx.props().onsort
                {
                    onsort.emit(());
                }
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let sort_by_icon = if ctx.props().is_sorted_by {
            if let Some(sort_direction) = &ctx.props().sort_direction {
                if sort_direction == &SortByDirection::Asc {
                    long_arrow_alt_up_icon!{}
                } else {
                    long_arrow_alt_down_icon!{}
                }
            } else {
                long_arrow_alt_down_icon!{}
            }
        } else {
            arrows_alt_v_icon!{}
        };

        html!{
            <button
                // {...props}
                type="button"
                class={classes!(
                    ctx.props().classes.clone(), 
                    "pf-v5-c-table__button",
                )}
                onclick={ctx.link().callback(|_| SortColumnMsg::OnSort)}
                onfocus={ctx.link().callback(|_| SortColumnMsg::Focused(true))}
                onblur={ctx.link().callback(|_| SortColumnMsg::Focused(false))}
            >
                <div class={classes!(ctx.props().classes.clone(), "pf-v5-c-table__button-content")}>
                    <TableText
                        tooltip={ctx.props().tooltip.clone()}
                        // tooltipProps={tooltipProps}
                        tooltip_has_default_behavior={ctx.props().tooltip_has_default_behavior}
                        focused={self.focused}
                    >
                        { for ctx.props().children.iter() }
                    </TableText>
                    <span class="pf-v5-c-table__sort-indicator">
                        {sort_by_icon}
                    </span>
                </div>
            </button>
        }
    }
}