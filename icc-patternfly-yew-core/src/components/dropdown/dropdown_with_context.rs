use yew::{
    prelude::*,
};

use super::{DropdownDirection, DropdownMenu, DropdownPosition, DropdownProperties, DropdownToggleComponents};


pub struct DropdownWithContext
{
    menu_ref: NodeRef,
}

impl Component for DropdownWithContext
{
    type Message = ();
    type Properties = DropdownProperties;

    fn create(_: &Context<Self>) -> Self
    {
        Self {
            menu_ref: NodeRef::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let aria_haspopup = ctx.props().dropdown_items.len() > 0;

        html!{
            <div
                // {...props}
                class={classes!(
                    "pf-c-dropdown",
                    if ctx.props().direction == DropdownDirection::Up { "pf-m-top" } else { "" },
                    if ctx.props().position == DropdownPosition::Right { "pf-m-align-right" } else { "" },
                    if ctx.props().is_open { "pf-m-expanded" } else { "" },
                    ctx.props().class_name.clone(),
                )}
                // {...getOUIAProps(ouiaComponentType, ouiaId, ouiaSafe)}
            >
            {
                match &ctx.props().toggle
                {
                    DropdownToggleComponents::DropdownToggle(toggle) => {
                        let mut new_toggle = toggle.clone();
                        let mut props = (&*new_toggle.props).clone();

                        props.menu_ref = self.menu_ref.clone();
                        props.is_open = ctx.props().is_open;
                        //     id,
                        props.is_plain = ctx.props().is_plain;
                        props.aria_haspopup = aria_haspopup.to_string();
                        //     onEnter: () => this.onEnter()

                        new_toggle.props = std::rc::Rc::new(props);

                        html!{new_toggle}
                    },
                    DropdownToggleComponents::KebabToggle(toggle) => {
                        let mut new_toggle = toggle.clone();
                        let mut props = (&*new_toggle.props).clone();

                        props.menu_ref = self.menu_ref.clone();
                        props.is_open = ctx.props().is_open;
                        //     id,
                        props.is_plain = ctx.props().is_plain;
                        props.aria_haspopup = aria_haspopup.to_string();
                        //     onEnter: () => this.onEnter()

                        new_toggle.props = std::rc::Rc::new(props);

                        html!{new_toggle}
                    },
                }
            }
            {
                if ctx.props().is_open
                {
                    html!{
                        <DropdownMenu
                            ref={self.menu_ref.clone()}
                            // component={component}
                            is_open={ctx.props().is_open}
                            position={ctx.props().position.clone()}
                            // aria-labelledby={contextId ? `${contextId}-toggle` : id}
                            is_grouped={ctx.props().is_grouped}
                            // auto_focus={openedOnEnter && autoFocus}
                            onselect={ctx.props().onselect.clone()}
                        >
                        { ctx.props().dropdown_items.clone() }
                        </DropdownMenu>
                    }
                }
                else
                {
                    html!{}
                }
            }
            </div>
        }
    }
}
