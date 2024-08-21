use yew::prelude::*;


#[derive(Clone, PartialEq)]
pub enum EmptyStateVariant
{
    Xs,
    Small,
    Large,
    Xl,
    Full,
}

pub struct EmptyState;

#[derive(Clone, PartialEq, Properties)]
pub struct EmptyStateProps
{
    /** Additional classes added to the EmptyState */
    #[prop_or_default]
    pub class_name: String,
    /** Content rendered inside the EmptyState */
    #[prop_or_default]
    pub children: Children,
    /** Modifies EmptyState max-width */
    #[prop_or(EmptyStateVariant::Full)]
    pub variant: EmptyStateVariant,
    /** Cause component to consume the available height of its container */
    #[prop_or_default]
    pub is_full_height: bool,
}

impl Component for EmptyState
{
    type Message = ();
    type Properties = EmptyStateProps;

    fn create(_: &Context<Self>) -> Self
    {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let variant_cls = match ctx.props().variant {
            EmptyStateVariant::Xs => "pf-m-xs",
            EmptyStateVariant::Small => "pf-m-sm",
            EmptyStateVariant::Large => "pf-m-lg",
            EmptyStateVariant::Xl => "pf-m-xl",
            EmptyStateVariant::Full => "",
        };

        html!{
            <div
                class={classes!(
                    "pf-v5-c-empty-state",
                    variant_cls,
                    if ctx.props().is_full_height { "pf-m-full-height" } else { "" },
                    ctx.props().class_name.clone()
                )}
                // {...props}
            >
                <div class="pf-v5-c-empty-state__content">
                {
                    for ctx.props().children.iter()
                }
                </div>
            </div>
        }
    }
}
