use yew::prelude::*;

pub struct ExpandableSection
{
    is_expanded: bool,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExpandableSectionProperties
{
    /** Content rendered inside the Expandable Component */
    pub children: Children,
    /** Additional classes added to the Expandable Component */
    #[prop_or_default]
    pub class_name: String,
    /** Flag to indicate if the content is expanded */
    #[prop_or_default]
    pub is_expanded: Option<bool>,
    /** Text that appears in the toggle */
    #[prop_or_default]
    pub toggle_text: String,
    /** Text that appears in the toggle when expanded (will override toggleText if both are specified; used for uncontrolled expandable with dynamic toggle text) */
    #[prop_or_default]
    pub toggle_text_expanded: String,
    /** Text that appears in the toggle when collapsed (will override toggleText if both are specified; used for uncontrolled expandable with dynamic toggle text) */
    #[prop_or_default]
    pub toggle_text_collapsed: String,
    /** Callback function to toggle the expandable content */
    #[prop_or_default]
    pub ontoggle: Callback<bool>,
    /** Forces active state */
    #[prop_or_default]
    pub is_active: bool,
}

pub enum ExpandableSectionMsg
{
    OnClick,
}

impl Component for ExpandableSection
{
    type Message = ExpandableSectionMsg;
    type Properties = ExpandableSectionProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self {
            is_expanded: false,
        }
    }

    /// Called everytime when messages are received
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            ExpandableSectionMsg::OnClick => {
                if let Some(is_expanded) = ctx.props().is_expanded
                {
                    ctx.props().ontoggle.emit(is_expanded);

                    false
                }
                else
                {
                    self.is_expanded = !self.is_expanded;

                    true
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let prop_or_state_is_expanded = match ctx.props().is_expanded {
            Some(is_expanded) => is_expanded,
            None => self.is_expanded,
        };

        html!{
            <div
                // {...props}
                class={classes!(
                    "pf-v5-c-expandable-section",
                    if prop_or_state_is_expanded { "pf-m-expanded" } else { "" },
                    if ctx.props().is_active { "pf-m-active" } else { "" },
                    ctx.props().class_name.to_string(),
                )}
            >
                <button
                    class="pf-v5-c-expandable-section__toggle"
                    type="button"
                    aria-expanded={prop_or_state_is_expanded.to_string()}
                    onclick={ctx.link().callback(|_| ExpandableSectionMsg::OnClick)}
                >
                    <span class="pf-v5-c-expandable-section__toggle-icon">
                        <i class="fas fa-angle-right"></i>
                    </span>
                    <span class="pf-v5-c-expandable-section__toggle-text">
                        { self.get_toggle_text(prop_or_state_is_expanded, ctx) }
                    </span>
                </button>
                <div class="pf-v5-c-expandable-section__content" hidden={!prop_or_state_is_expanded}>
                    { ctx.props().children.clone() }
                </div>
            </div>
        }
    }
}

impl ExpandableSection
{
    fn get_toggle_text(&self, is_expanded: bool, ctx: &Context<Self>) -> String
    {
        if is_expanded && !ctx.props().toggle_text_expanded.is_empty()
        {
            return ctx.props().toggle_text_expanded.to_string()
        }

        if !is_expanded && !ctx.props().toggle_text_collapsed.is_empty()
        {
            return ctx.props().toggle_text_collapsed.to_string()
        }

        ctx.props().toggle_text.to_string()
    }
}