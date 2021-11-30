use yew::{
    prelude::*,
    virtual_dom::{VTag},
};


pub struct MastheadBrand
{
    props: MastheadBrandProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct MastheadBrandProps
{
    /** Content rendered inside of the masthead brand. */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the masthead brand. */
    #[prop_or_default]
    pub class_name: String,
    /** Component type of the masthead brand. */
    #[prop_or(String::from("a"))]
    pub component: String,
}

impl Component for MastheadBrand
{
    type Message = ();
    type Properties = MastheadBrandProps;

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
        let mut classes = String::from("pf-c-masthead__brand");
        
        // Add extra classes specified on the parent
        if self.props.class_name.len() > 0
        {
            classes += " ";
            classes += &self.props.class_name;
        }

        component.add_attribute("class", classes);

        component.add_attribute("tabIndex", "0");

        //     {...props}

        component.add_children(self.props.children.iter());

        component.into()
    }
}
