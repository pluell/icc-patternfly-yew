use yew::prelude::*;

use crate::{VisibilityModifer};


pub struct DataListAction;

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

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                class={classes!(
                    "pf-v5-c-data-list__item-action",
                    ctx.props().visibility.iter().map(|vis_mod| vis_mod.get_class()).collect::<Vec<String>>(),
                    ctx.props().class_name.clone()
                )}
                // {...props}
            >
            {
                if ctx.props().is_plain_button_action
                {
                    html!{
                        <div class={"pf-v5-c-data-list__action"}>
                            { for ctx.props().children.iter() }
                        </div>
                    }
                }
                else
                {
                    html!{
                        for ctx.props().children.iter()
                    }
                }
            }
            </div>
        }
    }
}
