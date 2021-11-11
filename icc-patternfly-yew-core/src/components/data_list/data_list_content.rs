use yew::{
    prelude::*,
};


pub struct DataListContent
{
    props: DataListContentProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct DataListContentProps
{
    /** Content rendered inside the DataList item */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the DataList cell */
    #[prop_or_default]
    pub class_name: String,
    /** Identify the DataListContent item */
    #[prop_or_default]
    pub id: Option<String>,
    /** Id for the row */
    #[prop_or_default]
    pub rowid: String,
    /** Flag to show if the expanded content of the DataList item is visible */
    #[prop_or_default]
    pub is_hidden: bool,
    /** Flag to remove padding from the expandable content */
    #[prop_or_default]
    pub has_no_padding: bool,
    /** Adds accessible text to the DataList toggle */
    pub aria_label: String,
}

impl Component for DataListContent
{
    type Message = ();
    type Properties = DataListContentProps;

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
        html!{
            <section
                id=self.props.id.clone()
                class=classes!(
                    "pf-c-data-list__expandable-content",
                    self.props.class_name.clone()
                )
                hidden=self.props.is_hidden
                aria-label=self.props.aria_label.to_string()
                // {...props}
            >
                <div 
                    class=classes!(
                        "pf-c-data-list__expandable-content-body", 
                        if self.props.has_no_padding { "pf-m-no-padding" } else { "" }
                    )
                >
                    { for self.props.children.iter() }
                </div>
            </section>
        }
    }
}
