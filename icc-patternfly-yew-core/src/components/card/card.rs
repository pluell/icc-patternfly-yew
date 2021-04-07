use yew::{
    prelude::*,
    html::{ChildrenRenderer},
    virtual_dom::{VTag},
};

use super::{CardChildVariant, CardVariants};



pub struct Card
{
    props: CardProperties,
}

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

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self
    {
        Self {
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
    fn update(&mut self, _: Self::Message) -> ShouldRender
    {
        false
    }

    fn view(&self) -> Html
    {
        // Check if we have a valid is_large setting
        let is_large = if self.props.is_compact && self.props.is_large {
            log::warn!("Card: Cannot use is_compact with is_large. Defaulting to is_compact");

            false
        } else {
            self.props.is_large
        };

        // Create the component base tag
        let mut component = VTag::new(self.props.component.clone());

        if self.props.id.len() > 0 { component.add_attribute("id", &self.props.id); }

        // Build list of classes
        let mut classes = String::from("pf-c-card");

        if self.props.is_hoverable { classes += " pf-m-hoverable" }
        if self.props.is_compact { classes += " pf-m-compact" }
        if self.props.is_selectable { classes += " pf-m-selectable" }
        if self.props.is_selected { classes += " pf-m-selected" }
        if self.props.is_expanded { classes += " pf-m-expanded" }
        if self.props.is_flat { classes += " pf-m-flat" }
        if self.props.is_rounded { classes += " pf-m-rounded" }
        if is_large { classes += " pf-m-display-lg" }
        
        // Add extra classes specified on the parent
        if self.props.class_name.len() > 0
        {
            classes += " ";
            classes += &self.props.class_name;
        }

        component.add_attribute("class", &classes);

        // Set the tab index if the card is selectable
        if self.props.is_selectable { component.add_attribute("tabIndex", &0); }

        //     {...props}
        //     {...ouiaProps}

        // Add children to the base tag and check if child properties
        // need to be updated from card context
        for mut child in self.props.children.iter()
        {
            match child.props
            {
                CardVariants::Header(ref mut props) => {
                    props.card_id = self.props.id.clone();
                },
                CardVariants::ExpandableContent(ref mut props) => {
                    props.is_expanded = self.props.is_expanded;
                },
                _ => {}
            }
            
            component.add_child(child.into());
        }

        component.into()
    }
}
