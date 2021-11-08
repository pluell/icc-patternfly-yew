use yew::{
    prelude::*,
};

pub struct ActionGroup
{
    props: ActionGroupProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ActionGroupProps
{
    /** Anything that can be rendered as ActionGroup content. */
    pub children: Children,
    /** Additional classes added to the ActionGroup. */
    #[prop_or_default]
    pub class_name: String,
}

impl Component for ActionGroup
{
    type Message = ();
    type Properties = ActionGroupProps;

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
            <div 
                // {...props} 
                class=classes!("pf-c-form__group", "pf-m-action", self.props.class_name.to_string())
            >
                <div class=classes!("styles.pf-c-form__group-control")>
                    <div class=classes!("pf-c-form__actions")>
                        { self.props.children.clone() }
                    </div>
                </div>
            </div>
        }
    }
}
