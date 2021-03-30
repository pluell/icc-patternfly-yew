use yew::{
    prelude::*,
};


pub struct ModalBoxFooter
{
    props: ModalBoxFooterProperties,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ModalBoxFooterProperties
{
    /** Content rendered inside the Footer */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the Footer */
    #[prop_or_default]
    pub class_name: String,
}

impl Component for ModalBoxFooter
{
    type Message = ();
    type Properties = ModalBoxFooterProperties;

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
            <footer
                //{...props} 
                class=(
                    "pf-c-modal-box__footer",
                    self.props.class_name.clone(),
                )
            >
                { for self.props.children.iter() }
            </footer>
        }
    }
}
