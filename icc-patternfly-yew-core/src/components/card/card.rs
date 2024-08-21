use yew::{
    prelude::*,
    html::ChildrenRenderer,
    virtual_dom::VTag,
};

use super::CardChildVariant;


pub struct Card;

#[derive(Clone, PartialEq, Properties)]
pub struct CardProperties
{
    /** Content rendered inside the Card */
    #[prop_or_default]
    pub children: ChildrenRenderer<CardChildVariant>,
    /** ID of the Card. Also passed back in the CardHeader onExpand callback. */
    #[prop_or_default]
    pub id: String,
    /** Additional classes added to the Card */
    #[prop_or_default]
    pub class_name: String,
    /** Sets the base component to render. defaults to article */
    #[prop_or(String::from("article"))]
    pub component: String,
    /** Modifies the card to include hover styles on :hover */
    #[prop_or_default]
    pub is_hoverable: bool,
    /** Modifies the card to include compact styling. Should not be used with isLarge. */
    #[prop_or_default]
    pub is_compact: bool,
    /** Modifies the card to include selectable styling */
    #[prop_or_default]
    pub is_selectable: bool,
    /** Modifies the card to include selected styling */
    #[prop_or_default]
    pub is_selected: bool,
    /** Modifies the card to include flat styling */
    #[prop_or_default]
    pub is_flat: bool,
    /** Modifies the card to include rounded styling */
    #[prop_or_default]
    pub is_rounded: bool,
    /** Modifies the card to be large. Should not be used with isCompact. */
    #[prop_or_default]
    pub is_large: bool,
    /** Flag indicating if a card is expanded. Modifies the card to be expandable. */
    #[prop_or_default]
    pub is_expanded: bool,
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

        // Create the component base tag
        let mut component = VTag::new(ctx.props().component.clone());

        if ctx.props().id.len() > 0 { component.add_attribute("id", ctx.props().id.clone()); }

        // Build list of classes
        let mut classes = String::from("pf-v5-c-card");

        if ctx.props().is_hoverable { classes += " pf-m-hoverable" }
        if ctx.props().is_compact { classes += " pf-m-compact" }
        if ctx.props().is_selectable { classes += " pf-m-selectable" }
        if ctx.props().is_selected { classes += " pf-m-selected" }
        if ctx.props().is_expanded { classes += " pf-m-expanded" }
        if ctx.props().is_flat { classes += " pf-m-flat" }
        if ctx.props().is_rounded { classes += " pf-m-rounded" }
        if is_large { classes += " pf-m-display-lg" }
        
        // Add extra classes specified on the parent
        if ctx.props().class_name.len() > 0
        {
            classes += " ";
            classes += &ctx.props().class_name;
        }

        component.add_attribute("class", classes);

        // Set the tab index if the card is selectable
        if ctx.props().is_selectable { component.add_attribute("tabIndex", "0"); }

        //     {...props}
        //     {...ouiaProps}

        // Add children to the base tag and check if child properties
        // need to be updated from card context
        for child in ctx.props().children.iter()
        {
            match child
            {
                CardChildVariant::Header(mut child) => {
                    let mut props = (&*child.props).clone();
                    
                    props.card_id = ctx.props().id.clone();

                    child.props = std::rc::Rc::new(props);
                },
                CardChildVariant::ExpandableContent(mut child) => {
                    let mut props = (&*child.props).clone();

                    props.is_expanded = ctx.props().is_expanded;

                    child.props = std::rc::Rc::new(props);
                },
                _ => {}
            }
        }

        component.add_children(ctx.props().children.iter().map(|child| child.into()));

        component.into()
    }
}
