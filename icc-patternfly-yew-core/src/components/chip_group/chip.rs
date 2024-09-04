use yew::prelude::*;

use icc_patternfly_yew_icons::times_icon;

use crate::components::{Button, ButtonVariant};


pub struct Chip;

#[derive(Clone, PartialEq, Properties)]
pub struct ChipProperties
{
    /** Badge to add to the chip. The badge will be rendered after the chip text. */
    #[prop_or_default]
    pub badge: Option<Html>,
    /** Content rendered inside the chip text */
    pub children: Children,
    /** Aria Label for close button */
    #[prop_or_default]
    pub close_btn_aria_label: Option<String>,
    /** Additional classes added to the chip item */
    #[prop_or_default]
    pub classes: Classes,
    /** Flag indicating if the chip is an overflow chip */
    #[prop_or_default]
    pub is_overflow_chip: bool,
    /** Flag indicating if chip is read only */
    #[prop_or_default]
    pub is_read_only: bool,
    /** Function that is called when clicking on the chip close button */
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    /** Component that will be used for chip. It is recommended that <button> or <li>  are used when the chip is an overflow chip. */
    #[prop_or(String::from("div"))]
    pub component: String,
    // /** Position of the tooltip which is displayed if text is longer */
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

    /** Css property expressed in percentage or any css unit that overrides the default value of the max-width of the chip's text */
    #[prop_or_default]
    pub text_max_width: Option<String>,
    /** Value to overwrite the randomly generated data-ouia-component-id.*/
    #[prop_or_default]
    pub ouia_id: Option<String>,
}

impl Component for Chip
{
    type Message = ();
    type Properties = ChipProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        if ctx.props().is_overflow_chip {
            self.view_overflow_chip(ctx)
        } else {
            self.view_chip(ctx)
        }
    }
}

impl Chip
{
    fn view_chip(&self, ctx: &Context<Self>) -> Html
    {
        let style = if let Some(text_max_width) = &ctx.props().text_max_width {
            Some(format!("max-width: {}", text_max_width))
        } else {
            None
        };

        html!{
            <@{ctx.props().component.clone()}
                {style}
                class={classes!("pf-v5-c-chip", ctx.props().classes.clone())}
                // {...(this.state.isTooltipVisible && { tabIndex: 0 })}
                // {...getOUIAProps(Chip.displayName, ouiaId !== undefined ? ouiaId : this.state.ouiaStateId)}
                // {...props}
            >
                <span class={"pf-v5-c-chip__content"}>
                    <span class={"pf-v5-c-chip__text"}>
                        {ctx.props().children.clone()}
                    </span>
                    {
                        if let Some(badge) = &ctx.props().badge
                        {
                            badge.clone()
                        }
                        else{
                            html!{}
                        }
                    }
                </span>
                {
                    if !ctx.props().is_read_only
                    {
                        html!{
                            <span class="pf-v5-c-chip__actions">
                                <Button
                                    variant={ButtonVariant::Plain}
                                    onclick={ctx.props().onclick.clone()}
                                    aria_label={ctx.props().close_btn_aria_label.clone()}
                                    // id={`remove_${id}`}
                                    // aria-labelledby={`remove_${id} ${id}`}
                                    // ouiaId={ouiaId || closeBtnAriaLabel}
                                >
                                    {times_icon!{}}
                                </Button>
                            </span>
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
            </@>
        }
    }

    fn view_overflow_chip(&self, ctx: &Context<Self>) -> Html
    {
        let style = if let Some(text_max_width) = &ctx.props().text_max_width {
            Some(format!("max-width: {}", text_max_width))
        } else {
            None
        };

        let type_prop = if ctx.props().component == "button" {    
            Some(String::from("button"))
        } else {
            None
        };

        html!{
            <@{ctx.props().component.clone()}
                onclick={ctx.props().onclick.clone()}
                {style}
                class={classes!("pf-v5-c-chip", "pf-m-overflow", ctx.props().classes.clone())}
                type={type_prop}
                // {...getOUIAProps('OverflowChip', ouiaId !== undefined ? ouiaId : this.state.ouiaStateId)}
                // {...props}
            >
                <span class={"pf-v5-c-chip__content"}>
                    <span class={"pf-v5-c-chip__text"}>
                        {ctx.props().children.clone()}
                    </span>
                    {
                        if let Some(badge) = &ctx.props().badge {
                            badge.clone()
                        } else {
                            html!{}
                        }
                    }
                </span>
            </@>
        }
    }
}
