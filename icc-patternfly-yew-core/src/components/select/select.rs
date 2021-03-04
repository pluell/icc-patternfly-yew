
use yew::{
    html::{ChildrenWithProps},
    prelude::*,
};

use super::*;


pub struct Select<V: ToString + PartialEq + Clone + 'static>
{
    link: ComponentLink<Self>,
    props: SelectProperties<V>,
}

pub enum SelectMsg
{
    ToggleList(bool),
}

#[derive(Clone, PartialEq, Properties)]
pub struct SelectProperties<V: ToString + PartialEq + Clone + 'static>
{
    pub children: ChildrenWithProps<SelectOption<V>>,
    pub is_disabled: bool,
    pub is_open: bool,
    pub variant: SelectVariant,
    #[prop_or_default]
    pub placeholder_text: String,
    #[prop_or_default]
    pub selections: Vec<SelectOptionValue<V>>,
    pub ontoggle: Callback<bool>,
    pub onselect: Callback<SelectOptionValue<V>>,
}

impl<V: ToString + PartialEq + Clone + 'static> Component for Select<V>
{
    type Message = SelectMsg;
    type Properties = SelectProperties<V>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self {
            link,
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
    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            SelectMsg::ToggleList(is_open) => {
                self.props.ontoggle.emit(is_open);

                false
            },
        }
    }

    fn view(&self) -> Html
    {
        html!{
            <div class="pf-c-select">
                <SelectToggle
                    is_disabled=self.props.is_disabled
                    is_open=self.props.is_open
                    ontoggle=self.link.callback(|is_open| SelectMsg::ToggleList(is_open))
                >
                    {
                        match self.props.variant
                        {
                            SelectVariant::Single => {
                                html!{
                                    <div class="pf-c-select__toggle-wrapper">
                                        <span class="pf-c-select__toggle-text">{ self.get_display_string() }</span>
                                    </div>
                                }
                            },
                            SelectVariant::Checkbox => {
                                html!{
                                    <div class="pf-c-select__toggle-wrapper">
                                        <span class="pf-c-select__toggle-text">{&self.props.placeholder_text}</span>
                                        {
                                            if self.props.selections.len() > 0
                                            {
                                                html!{
                                                    <div class="pf-c-select__toggle-badge">
                                                        <span class="pf-c-badge pf-m-read">{self.props.selections.len()}</span>
                                                    </div>
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

                    }
                </SelectToggle>
                {
                    if self.props.is_open
                    {
                        html!{
                            <SelectMenu
                                variant=self.props.variant.clone()
                            >
                                { self.render_select_list() }
                            </SelectMenu>
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

impl<V: ToString + PartialEq + Clone + 'static> Select<V>
{
    fn render_select_list(&self) -> Html
    {
        if self.props.children.is_empty()
        {
            html!{}
        }
        else
        {
            html! {
                for self.props.children.iter()
                    .map(|mut select_opt| {
                        select_opt.props.is_selected = self.props.selections.contains(&select_opt.props.opt_val);

                        select_opt.props.variant = self.props.variant.clone();
                        
                        select_opt.props.onclick = self.props.onselect.clone();

                        select_opt
                    })
            }
        }
    }

    fn get_display_string(&self) -> String
    {
        if self.props.selections.len() == 1
        {
            // Find selected item
            for c in self.props.children.iter()
            {
                if self.props.selections.contains(&c.props.opt_val)
                {
                    // Display the value of the select option if it matches the selected key
                    match &c.props.opt_val
                    {
                        SelectOptionValue::String(value) => {
                            return value.to_string()
                        },
                        SelectOptionValue::Object(obj) => {
                            return obj.to_string()
                        },
                        SelectOptionValue::ObjRef(obj) => {
                            return obj.borrow().to_string()
                        }
                    }
                }
                
            }
        }

        // Use placeholder text if there is a placeholder
        self.props.placeholder_text.clone()
    }
}
