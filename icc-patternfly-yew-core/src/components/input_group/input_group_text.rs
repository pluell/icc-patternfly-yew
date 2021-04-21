use yew::{
    prelude::*,
    virtual_dom::{VTag},
};


#[derive(Clone, PartialEq)]
pub enum InputGroupTextVariant
{
    Default,
    Plain,
}

pub struct InputGroupText
{
    props: InputGroupTextProperties,
}

#[derive(Clone, PartialEq, Properties)]
pub struct InputGroupTextProperties
{
    /** Additional classes added to the input group text. */
    #[prop_or_default]
    pub class_name: String,
    /** Content rendered inside the input group text. */
    #[prop_or_default]
    pub children: Children,
    /** Component that wraps the input group text. */
    #[prop_or(String::from("span"))]
    pub component: String,
    /** Input group text variant */
    #[prop_or(InputGroupTextVariant::Default)]
    pub variant: InputGroupTextVariant,
}

impl Component for InputGroupText
{
    type Message = ();
    type Properties = InputGroupTextProperties;

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
        true
    }

    fn view(&self) -> Html
    {
        let mut component = VTag::new(self.props.component.clone());

        // Build list of classes
        let mut classes = String::from("pf-c-input-group__text");

        if self.props.variant == InputGroupTextVariant::Plain
        {
            classes += " pf-m-plain";
        }
        
        // Add extra classes specified on the parent
        if self.props.class_name.len() > 0
        {
            classes += " ";
            classes += &self.props.class_name;
        }

        component.add_attribute("class", &classes);

        //     {...props}

        component.add_children(self.props.children.iter());

        component.into()
    }
}
