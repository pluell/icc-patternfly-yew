use yew::{
    prelude::*,
};
use std::cell::RefCell;
use std::rc::Rc;

use super::*;


pub struct SelectOption<V: ToString + PartialEq + Clone + 'static>
{
    link: ComponentLink<Self>,
    props: SelectOptionProperties<V>,
}

pub enum SelectOptionMsg
{
    OnClick,
}

#[derive(Clone, PartialEq)]
pub enum SelectOptionValue<V: PartialEq + ToString>
{
    String(String),
    Object(V),
    ObjRef(Rc<RefCell<V>>),
}

#[derive(Clone, PartialEq, Properties)]
pub struct SelectOptionProperties<V: ToString + PartialEq + Clone + 'static>
{
    #[prop_or_default]
    pub onclick: Callback<SelectOptionValue<V>>,
    #[prop_or_default]
    pub is_selected: bool,
    #[prop_or(SelectVariant::Single)]
    pub variant: SelectVariant,
    pub opt_val: SelectOptionValue<V>,
    #[prop_or_default]
    pub description: String,
}

impl<V: ToString + PartialEq + Clone + 'static> Component for SelectOption<V>
{
    type Message = SelectOptionMsg;
    type Properties = SelectOptionProperties<V>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self {
            link,
            props,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        self.props = props;

        true
    }

    /// Called everytime when messages are received
    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            SelectOptionMsg::OnClick => {
                self.props.onclick.emit(self.props.opt_val.clone());
                
                false
            },
        }
    }

    fn view(&self) -> Html
    {
        let cls_description = |has_desc| {
            match has_desc
            {
                true => "pf-m-description",
                false => ""
            }
        };

        let has_desc = !self.props.description.is_empty();

        match self.props.variant
        {
            SelectVariant::Single => {
                html!{
                    <li>
                        <button 
                            type="button"
                            class=("pf-c-select__menu-item", cls_description(has_desc))
                            aria-selected="false"
                            onclick=self.link.callback(|_| SelectOptionMsg::OnClick)
                        >
                        {
                            if has_desc
                            {
                                html!{
                                    <>
                                    <span class="pf-c-select__menu-item-main">
                                        {&self.get_display_string()}
                                        <span class="pf-c-select__menu-item-icon" hidden=!self.props.is_selected>
                                            <i class="fas fa-check" aria-hidden="true"></i>
                                        </span>
                                    </span>
                                    <span class="pf-c-select__menu-item-description">{&self.props.description}</span>
                                    </>
                                }
                            }
                            else
                            {
                                html!{
                                    <>
                                    {&self.get_display_string()}
                                    <span class="pf-c-select__menu-item-icon" hidden=!self.props.is_selected>
                                        <i class="fas fa-check" aria-hidden="true"></i>
                                    </span>
                                    </>
                                }
                            }
                        }
                        </button>
                    </li>
                }

            },
            SelectVariant::Checkbox => {
                html!{
                    <label class="pf-c-check pf-c-select__menu-item pf-m-description">
                        <input 
                            class="pf-c-check__input" 
                            type="checkbox" 
                            checked=self.props.is_selected 
                            onclick=self.link.callback(|_| SelectOptionMsg::OnClick)
                        />
                        <span class="pf-c-check__label">{&self.get_display_string()}</span>
                        {
                            if has_desc
                            {
                                html!{
                                    <div class="pf-c-check__description">{&self.props.description}</div>
                                }
                            }
                            else
                            {
                                html!{}
                            }
                        }
                    </label>
                }
            },
        }
    }
}

impl<V: ToString + PartialEq + Clone + 'static> SelectOption<V>
{
    fn get_display_string(&self) -> String
    {
        match &self.props.opt_val
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