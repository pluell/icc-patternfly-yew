use yew::{
    prelude::*,
};

use super::{DropdownDirection, DropdownMenu, DropdownPosition, DropdownProperties, DropdownToggleComponents};


pub struct DropdownWithContext
{
    props: DropdownProperties,
    menu_ref: NodeRef,
}

impl Component for DropdownWithContext
{
    type Message = ();
    type Properties = DropdownProperties;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self
    {
        Self {
            props,
            menu_ref: NodeRef::default(),
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
        let aria_haspopup = self.props.dropdown_items.len() > 0;

        html!{
            <div
                // {...props}
                class=classes!(
                    "pf-c-dropdown",
                    if self.props.direction == DropdownDirection::Up { "pf-m-top" } else { "" },
                    if self.props.position == DropdownPosition::Right { "pf-m-align-right" } else { "" },
                    if self.props.is_open { "pf-m-expanded" } else { "" },
                    self.props.class_name.clone(),
                )
                // {...getOUIAProps(ouiaComponentType, ouiaId, ouiaSafe)}
            >
            {
                match &self.props.toggle
                {
                    DropdownToggleComponents::DropdownToggle(toggle) => {
                        let mut new_toggle = toggle.clone();

                        new_toggle.props.menu_ref = self.menu_ref.clone();
                        new_toggle.props.is_open = self.props.is_open;
                        //     id,
                        new_toggle.props.is_plain = self.props.is_plain;
                        new_toggle.props.aria_haspopup = aria_haspopup.to_string();
                        //     onEnter: () => this.onEnter()

                        new_toggle
                    }
                }
            }
            {
                if self.props.is_open
                {
                    html!{
                        <DropdownMenu
                            ref=self.menu_ref.clone()
                            // component={component}
                            is_open=self.props.is_open
                            position=self.props.position.clone()
                            // aria-labelledby={contextId ? `${contextId}-toggle` : id}
                            is_grouped=self.props.is_grouped
                            // auto_focus={openedOnEnter && autoFocus}
                            onselect=self.props.onselect.clone()
                        >
                        { self.props.dropdown_items.clone() }
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
