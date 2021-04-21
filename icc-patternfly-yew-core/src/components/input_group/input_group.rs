use yew::{
    prelude::*,
};


pub struct InputGroup
{
    props: InputGroupProperties,
}

#[derive(Clone, PartialEq, Properties)]
pub struct InputGroupProperties
{
    /** Additional classes added to the input group. */
    #[prop_or_default]
    pub class_name: String,
    /** Content rendered inside the input group. */
    #[prop_or_default]
    pub children: Children,
}

impl Component for InputGroup
{
    type Message = ();
    type Properties = InputGroupProperties;

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
                class=(
                    "pf-c-input-group", 
                    &self.props.class_name,
                )
                // {...props}
            >
            {
                // {idItem
                // ? React.Children.map(children, (child: any) =>
                //     formCtrls.includes(child.type.displayName)
                //         ? React.cloneElement(child, { 'aria-describedby': idItem.props.id })
                //         : child
                //     )
                // : children}
                for self.props.children.iter()
            }
            </div>
        }
    }
}
