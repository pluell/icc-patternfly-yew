use yew::{
    prelude::*,
    virtual_dom::{VTag},
};

#[derive(Clone, PartialEq)]
pub enum TitleElements
{
    Div,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl From<TitleElements> for VTag
{
    fn from(element: TitleElements) -> VTag
    {
        match element
        {
            TitleElements::Div => VTag::new("div"),
            TitleElements::H1 => VTag::new("h1"),
            TitleElements::H2 => VTag::new("h2"),
            TitleElements::H3 => VTag::new("h3"),
            TitleElements::H4 => VTag::new("h4"),
            TitleElements::H5 => VTag::new("h5"),
            TitleElements::H6 => VTag::new("h6"),
        }
    }
}


pub struct FormSection
{
    props: FormSectionProperties,
}

#[derive(Clone, PartialEq, Properties)]
pub struct FormSectionProperties
{
    /** Content rendered inside the section */
    pub children: Children,
    /** Additional classes added to the FormSection. */
    #[prop_or_default]
    pub class_name: String,
    /** Title for the section */
    #[prop_or_default]
    pub title: Option<Html>,
    /** Element to wrap the section title*/
    #[prop_or(TitleElements::Div)]
    pub title_element: TitleElements,
}

impl Component for FormSection
{
    type Message = ();
    type Properties = FormSectionProperties;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self
    {
        Self {
            // link,
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
        html!{
            <section
                // {...props} 
                class=classes!("pf-c-form__section", self.props.class_name.to_string())
            >
                {
                    if let Some(title) = &self.props.title
                    {
                        let mut title_element: VTag = self.props.title_element.clone().into();

                        title_element.add_attribute(
                            "class",
                            classes!(
                                "pf-c-form__section-title",
                                self.props.class_name.to_string()
                            ).to_string()
                        );

                        title_element.add_child(title.clone());

                        title_element.into()
                    }
                    else
                    {
                        html!{}
                    }
                }
                { self.props.children.clone() }
            </section>
        }
    }
}
