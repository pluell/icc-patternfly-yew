use yew::{
    prelude::*,
};


pub struct ModalBoxDescription
{
    props: ModalBoxDescriptionProperties,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ModalBoxDescriptionProperties
{
    /** Content rendered inside the description */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the description */
    #[prop_or_default]
    pub class_name: String,
    /** ID of the description */
    #[prop_or_default]
    pub id: String,
}

impl Component for ModalBoxDescription
{
    type Message = ();
    type Properties = ModalBoxDescriptionProperties;

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
                // {...props} 
                id=self.props.id
                class=(
                    "pf-c-modal-box__description",
                    self.props.class_name.clone()
                )
            >
                { for self.props.children.iter() }
            </div>
        }
    }
}
