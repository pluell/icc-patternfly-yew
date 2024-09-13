use yew::prelude::*;


#[derive(Clone, PartialEq)]
pub enum TableTextVariant
{
    Span,
    Div,
    Nav,
}

pub struct TableText;

#[derive(Clone, PartialEq, Properties)]
pub struct TableTextProps
{
    /** Content rendered within the table text */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the table text */
    #[prop_or_default]
    pub classes: Classes,
    /** Determines which element to render as a table text */
    #[prop_or(TableTextVariant::Span)]
    pub variant: TableTextVariant,
    // /** Determines which wrapping modifier to apply to the table text */
    // wrapModifier?: WrapModifier | 'wrap' | 'nowrap' | 'truncate' | 'breakWord' | 'fitContent';
    /** text to display on the tooltip */
    #[prop_or_default]
    pub tooltip: Option<Html>,
    /** other props to pass to the tooltip */
    // tooltipProps?: Omit<TooltipProps, 'content'>;
    /** callback used to create the tooltip if text is truncated */
    #[prop_or_default]
    pub on_mouse_enter: Callback<()>,
    /** Determines if the TableText is focused by parent component */
    #[prop_or_default]
    pub focused: bool,
    /** Determines if tooltip should have normal visbility behavior. If false, the tooltip will only be shown when children is not entirely visible */
    #[prop_or_default]
    pub tooltip_has_default_behavior: bool,
}

impl Component for TableText
{
    type Message = ();
    type Properties = TableTextProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let component = match ctx.props().variant {
            TableTextVariant::Span => "span",
            TableTextVariant::Div => "div",
            TableTextVariant::Nav => "nav",
        };

        html!{
            <@{component}
                // ref={textRef as React.Ref<HTMLDivElement>}
                // onMouseEnter={!tooltipHasDefaultBehavior ? onMouseEnter : undefined}
                class={classes!(
                    ctx.props().classes.clone(),
                    // wrapModifier && styles.modifiers[wrapModifier],
                    "pf-v5-c-table__text"
                )}
                // {...props}
            >
                { for ctx.props().children.iter() }
            </@>
        }
    }
}