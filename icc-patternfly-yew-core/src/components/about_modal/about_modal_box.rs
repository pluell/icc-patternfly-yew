use yew::{
    prelude::*,
};


pub struct AboutModalBox
{
    props: AboutModalBoxProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct AboutModalBoxProps
{
    /** content rendered inside the AboutModelBox.  */
    #[prop_or_default]
    pub children: Children,
    /** additional classes added to the AboutModalBox  */
    #[prop_or_default]
    pub class_name: String,

    // Aria props
    #[prop_or_default]
    pub aria_labelledby: Option<String>,
    #[prop_or_default]
    pub aria_describedby: Option<String>,
}

impl Component for AboutModalBox
{
    type Message = ();
    type Properties = AboutModalBoxProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self
    {
        Self {
            props
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
                role="dialog" aria-modal="true" 
                class=classes!("pf-c-about-modal-box", self.props.class_name.clone()) 
                // {...props}
                aria-labelledby=self.props.aria_labelledby.clone()
                aria-describedby=self.props.aria_describedby.clone()
            >
                {for self.props.children.iter()}
            </div>
        }
    }
}
