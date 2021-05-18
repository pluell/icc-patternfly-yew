use yew::{
    prelude::*,
    virtual_dom::{VTag},
};


#[derive(Clone, PartialEq)]
pub enum TextListVariants
{
    Ul,
    Ol,
    Dl,
}

impl Into<VTag> for TextListVariants
{
    fn into(self) -> VTag
    {
        match self
        {
            Self::Ul => VTag::new("ul"),
            Self::Ol => VTag::new("ol"),
            Self::Dl => VTag::new("dl"),
        }
    }
}


pub struct TextList
{
    props: TextListProperties,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TextListProperties
{
    /** Content rendered within the TextList */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the TextList */
    #[prop_or_default]
    pub class_name: String,
    /** The text list component */
    #[prop_or(TextListVariants::Ul)]
    pub component: TextListVariants,
}

impl Component for TextList
{
    type Message = ();
    type Properties = TextListProperties;

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
        let mut component: VTag = self.props.component.clone().into();

        if self.props.class_name.len() > 0
        {
            component.add_attribute("class", self.props.class_name.clone());
        }

        component.add_attribute("data-pf-content", true.to_string());

        component.add_children(self.props.children.iter());

        component.into()
    }
}
