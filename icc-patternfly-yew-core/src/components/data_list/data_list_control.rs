use yew::{
    prelude::*,
};


pub struct DataListControl
{
    props: DataListControlProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct DataListControlProps
{
    /** Children of the data list control */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the DataList item control */
    #[prop_or_default]
    pub class_name: String,
}

impl Component for DataListControl
{
    type Message = ();
    type Properties = DataListControlProps;

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
            <div
                class=classes!(
                    "pf-c-data-list__item-control",
                    self.props.class_name.clone()
                )
                // {...props}
            >
                { for self.props.children.iter() }
            </div>
        }
    }
}
