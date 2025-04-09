use yew::prelude::*;

use crate::TitleHeadingLevels;


pub struct EmptyStateHeader;

#[derive(Clone, PartialEq, Properties)]
pub struct EmptyStateHeaderProps
{
    /** Content rendered inside the empty state header, either in addition to or instead of the titleText prop */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the empty state header */
    #[prop_or_default]
    pub classes: Classes,
    /** Additional classes added to the title inside empty state header */
    #[prop_or_default]
    pub title_classes: Option<Classes>,
    /** Text of the title inside empty state header, will be wrapped in headingLevel */
    #[prop_or_default]
    pub title_text: Option<AttrValue>,
    /** Empty state icon element to be rendered */
    #[prop_or_default]
    pub icon: Option<Html>, //React.ReactElement<EmptyStateIconProps>;
    /** The heading level to use, default is h1 */
    #[prop_or(TitleHeadingLevels::H1)]
    pub heading_level: TitleHeadingLevels,
}

impl Component for EmptyStateHeader
{
    type Message = ();
    type Properties = EmptyStateHeaderProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html!{
            <div 
                class={classes!(
                    "pf-v5-c-empty-state__header",
                    ctx.props().classes.clone()
                )}
                // {...props}
            >
            {ctx.props().icon.clone()}
            {
                html!{
                    if ctx.props().title_text.is_some() || ctx.props().children.len() > 0 {
                        <div class="pf-v5-c-empty-state__title">
                            {
                                html!{
                                    if let Some(title_text) = &ctx.props().title_text {
                                        <@{ctx.props().heading_level.to_string()} 
                                            class={classes!("pf-v5-c-empty-state__title-text", ctx.props().title_classes.clone())}
                                        >
                                            {title_text}
                                        </@>
                                    }
                                }
                            
                            }
                            {ctx.props().children.clone()}
                        </div>
                    }
                }
            }
            </div>
        }
    }
}
