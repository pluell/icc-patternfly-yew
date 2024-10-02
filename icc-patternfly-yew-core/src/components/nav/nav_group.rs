use yew::prelude::*;

use crate::utils_get_unique_id;


pub struct NavGroup
{
    id: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct NavGroupProps
{
    /** Title shown for the group */
    #[prop_or_default]
    pub title: Option<AttrValue>,
    /** Anything that can be rendered inside of the group */
    #[prop_or_default]
    pub children: Html,
    /** Additional classes added to the container */
    #[prop_or_default]
    pub classes: Classes,
    /** Identifier to use for the section aria label */
    #[prop_or_default]
    pub id: Option<AttrValue>,
}

impl Component for NavGroup
{
    type Message = ();
    type Properties = NavGroupProps;

    fn create(ctx: &Context<Self>) -> Self
    {
        // Generate a unique id if one is not provided
        let id = if let Some(id) = &ctx.props().id {
            id.to_string()
        } else {
            utils_get_unique_id(None)
        };

        Self {
            id,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let labelled_by = if ctx.props().title.is_some() {
            Some(self.id.clone())
        } else {
            None
        };

        html!{
            <section 
                class={classes!("pf-v5-c-nav__section", ctx.props().classes.clone())} 
                aria-labelledby={labelled_by}
                // {...props}
            >
                {
                    if let Some(title) = &ctx.props().title {
                        html!{
                            <h2 class="pf-v5-c-nav__section-title" id={self.id.clone()}>
                                {title.clone()}
                            </h2>
                        }
                    } else {
                        html!{}
                    }
                }
                <ul class={classes!("pf-v5-c-nav__list", ctx.props().classes.clone())} role="list">
                    {ctx.props().children.clone()}
                </ul>
            </section>
        }
    }
}
