use yew::{
    prelude::*,
    virtual_dom::VTag,
};

use crate::InsetModifer;


#[derive(Clone, PartialEq)]
pub enum DividerVariant
{
    Hr,
    Li,
    Div,
}

impl Into<VTag> for DividerVariant
{
    fn into(self) -> VTag
    {
        match self
        {
            Self::Hr => VTag::new("hr"),
            Self::Li => VTag::new("li"),
            Self::Div => VTag::new("div"),
        }
    }
}


pub struct Divider;

#[derive(Clone, PartialEq, Properties)]
pub struct DividerProps
{
    /** Additional classes added to the divider */
    #[prop_or_default]
    pub class_name: String,
    /** The component type to use */
    #[prop_or(DividerVariant::Hr)]
    pub component: DividerVariant,
    /** @deprecated Use `orientation` instead. Flag to indicate the divider is vertical (must be in a flex layout) */
    #[prop_or_default]
    pub is_vertical: bool,
    /** Insets at various breakpoints. */
    #[prop_or_default]
    pub inset: Vec<InsetModifer>,
    // /** Indicates how the divider will display at various breakpoints. Vertical divider must be in a flex layout. */
    // orientation?: {
    //     default?: 'vertical' | 'horizontal';
    //     sm?: 'vertical' | 'horizontal';
    //     md?: 'vertical' | 'horizontal';
    //     lg?: 'vertical' | 'horizontal';
    //     xl?: 'vertical' | 'horizontal';
    //     '2xl'?: 'vertical' | 'horizontal';
    // };
}

impl Component for Divider
{
    type Message = ();
    type Properties = DividerProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let mut component: VTag = ctx.props().component.clone().into();

        let classes = classes!(
            "pf-v5-c-divider",
            if ctx.props().is_vertical {"pf-m-vertical"} else {""},
            ctx.props().inset.iter().map(|inset_mod| inset_mod.get_class()).collect::<Vec<String>>(),
            ctx.props().class_name.clone()
        );

        component.add_attribute("class", classes.to_string());

        if ctx.props().component != DividerVariant::Hr
        {
            component.add_attribute("role", "separator");
        }

        // {...props}

        component.into()
    }
}
