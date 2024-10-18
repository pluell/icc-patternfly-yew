use yew::prelude::*;

use super::CardContext;


pub struct Card;

#[derive(Clone, PartialEq, Properties)]
pub struct CardProperties
{
    /** Content rendered inside the Card */
    #[prop_or_default]
    pub children: Html, //ChildrenRenderer<CardChildVariant>,
    /** ID of the Card. Also passed back in the CardHeader onExpand callback. */
    #[prop_or_default]
    pub id: AttrValue,
    /** Additional classes added to the Card */
    #[prop_or_default]
    pub classes: Classes,
    /** Sets the base component to render. defaults to div */
    #[prop_or(String::from("div"))]
    pub component: String,
    /** Modifies the card to include compact styling. Should not be used with isLarge. */
    #[prop_or_default]
    pub is_compact: bool,
    /** Flag indicating that the card is selectable. */
    #[prop_or_default]
    pub is_selectable: bool,
    /** Flag indicating that the card is clickable and contains some action that triggers on click. */
    #[prop_or_default]
    pub is_clickable: bool,
    /** Flag indicating whether a card that is either selectable only or both clickable and selectable is
     * currently selected and has selected styling.
     */
    #[prop_or_default]
    pub is_selected: bool,
    /** Flag indicating whether a card that is either only clickable or that is both clickable and selectable
     * is currently clicked and has clicked styling.
     */
    #[prop_or_default]
    pub is_clicked: bool,
    /** Flag indicating that a clickable or selectable card is disabled. */
    #[prop_or_default]
    pub is_disabled: bool,
    /** Modifies the card to include flat styling */
    #[prop_or_default]
    pub is_flat: bool,
    /** Modifies the card to include rounded styling */
    #[prop_or_default]
    pub is_rounded: bool,
    /** Modifies the card to be large. Should not be used with isCompact. */
    #[prop_or_default]
    pub is_large: bool,
    /** Cause component to consume the available height of its container */
    #[prop_or_default]
    pub is_full_height: bool,
    /** Modifies the card to include plain styling; this removes border and background */
    #[prop_or_default]
    pub is_plain: bool,
    /** Flag indicating if a card is expanded. Modifies the card to be expandable. */
    #[prop_or_default]
    pub is_expanded: bool,
    /** Value to overwrite the randomly generated data-ouia-component-id.*/
    #[prop_or_default]
    pub ouia_id: Option<AttrValue>,
    /** Set the value of data-ouia-safe. Only set to true when the component is in a static state, i.e. no animations are occurring. At all other times, this value must be false. */
    #[prop_or(true)]
    pub ouia_safe: bool,
}

impl Component for Card
{
    type Message = ();
    type Properties = CardProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        // Check if we have a valid is_large setting
        let is_large = if ctx.props().is_compact && ctx.props().is_large {
            log::warn!("Card: Cannot use is_compact with is_large. Defaulting to is_compact");

            false
        } else {
            ctx.props().is_large
        };

        html!{
            <ContextProvider<CardContext> context={CardContext{
                    card_id: ctx.props().id.clone(),
                    // register_title_id: Callback<String>,
                    is_expanded: ctx.props().is_expanded,
                    is_clickable: ctx.props().is_clickable,
                    is_selectable: ctx.props().is_selectable,
                    is_selected: ctx.props().is_selected,
                    is_clicked: ctx.props().is_clicked,
                    is_disabled: ctx.props().is_disabled,
                }}
            >
                <@{ctx.props().component.to_string()}
                    id={ctx.props().id.clone()}
                    class={classes!(
                        "pf-v5-c-card",
                        if ctx.props().is_compact {"pf-m-compact"} else {""},
                        if ctx.props().is_expanded {"pf-m-expanded"} else {""},
                        if ctx.props().is_flat {"pf-m-flat"} else {""},
                        if ctx.props().is_rounded {"pf-m-rounded"} else {""},
                        if is_large {"pf-m-display-lg"} else {""},
                        if ctx.props().is_full_height {"pf-m-full-height"} else {""},
                        if ctx.props().is_plain {"pf-m-plain"} else {""},
                        self.get_selectable_modifiers(ctx), // getSelectableModifiers(),
                        if ctx.props().is_disabled {"pf-m-disabled"} else {""},
                        ctx.props().classes.clone()
                    )}
                    // {...props}
                    // {...ouiaProps}
                    ouia-id={ctx.props().ouia_id.clone()}
                    ouid-safe={ctx.props().ouia_safe.to_string()}
                >
                    {ctx.props().children.clone()}
                </@>
            </ContextProvider<CardContext>>
        }
    }
}

impl Card
{
    fn get_selectable_modifiers(&self, ctx: &Context<Self>) -> Classes
    {
        // if (isSelectable && isClickable) 
        if ctx.props().is_selectable && ctx.props().is_clickable
        {
            return classes!(
              "pf-m-selectable",
              "pf-m-clickable",
              if ctx.props().is_selected || ctx.props().is_clicked {"pf-m-current"} else {""}
            );
        }
        
        if ctx.props().is_selectable
        {
            // return css(styles.modifiers.selectable, isSelected && styles.modifiers.selected);
            return classes!(
                "pf-m-selectable",
                if ctx.props().is_selected {"pf-m-current"} else {""}
              );
        }
        
        if ctx.props().is_clickable
        {
            // return css(styles.modifiers.clickable, isClicked && styles.modifiers.current);
            return classes!(
                "pf-m-clickable",
                if ctx.props().is_clicked {"pf-m-current"} else {""}
              );
        }

        classes!()
    }
}