use yew::{
    prelude::*,
};

use crate::{VisibilityModifer};


pub struct DataListAction
{
    props: DataListActionProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct DataListActionProps
{
    /** Content rendered as DataList Action  (e.g <Button> or <Dropdown>) */
    pub children: Children,
    /** Additional classes added to the DataList Action */
    #[prop_or_default]
    pub class_name: String,
    /** Identify the DataList toggle number */
    pub id: String,
    /** Adds accessible text to the DataList Action */
    pub aria_labelledby: String,
    /** Adds accessible text to the DataList Action */
    pub aria_label: String,
    // /** What breakpoints to hide/show the data list action */
    #[prop_or_default]
    pub visibility: Vec<VisibilityModifer>,
    /** Flag to indicate that the action is a plain button (e.g. kebab dropdown toggle) so that styling is applied to align the button */
    #[prop_or_default]
    pub is_plain_button_action: bool,
}

impl Component for DataListAction
{
    type Message = ();
    type Properties = DataListActionProps;

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
                    "pf-c-data-list__item-action",
                    self.props.visibility.iter().map(|vis_mod| vis_mod.get_class()).collect::<Vec<String>>(),
                    self.props.class_name.clone()
                )
                // {...props}
            >
            {
                if self.props.is_plain_button_action
                {
                    html!{
                        <div class="pf-c-data-list__action">
                            { for self.props.children.iter() }
                        </div>
                    }
                }
                else
                {
                    html!{
                        for self.props.children.iter()
                    }
                }
            }
            </div>
        }
    }
}
