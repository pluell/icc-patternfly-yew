use yew::{
    prelude::*,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::marker::PhantomData;

use super::*;


pub struct SelectOption<V: ToString + PartialEq + Clone + 'static>
{
    _phantom: PhantomData<V>,
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

    fn create(_: &Context<Self>) -> Self
    {
        Self{
            _phantom: PhantomData,
        }
    }

    /// Called everytime when messages are received
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        match msg
        {
            SelectOptionMsg::OnClick => {
                ctx.props().onclick.emit(ctx.props().opt_val.clone());
                
                false
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let cls_description = |has_desc| {
            match has_desc
            {
                true => "pf-m-description",
                false => ""
            }
        };

        let has_desc = !ctx.props().description.is_empty();

        match ctx.props().variant
        {
            SelectVariant::Single => {
                html!{
                    <li>
                        <button 
                            type="button"
                            class={classes!("pf-c-select__menu-item", cls_description(has_desc))}
                            aria-selected="false"
                            onclick={ctx.link().callback(|_| SelectOptionMsg::OnClick)}
                        >
                        {
                            if has_desc
                            {
                                html!{
                                    <>
                                    <span class="pf-c-select__menu-item-main">
                                        {&self.get_display_string(ctx)}
                                        <span class="pf-c-select__menu-item-icon" hidden={!ctx.props().is_selected}>
                                            <i class="fas fa-check" aria-hidden="true"></i>
                                        </span>
                                    </span>
                                    <span class="pf-c-select__menu-item-description">{&ctx.props().description}</span>
                                    </>
                                }
                            }
                            else
                            {
                                html!{
                                    <>
                                    {&self.get_display_string(ctx)}
                                    <span class="pf-c-select__menu-item-icon" hidden={!ctx.props().is_selected}>
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
                            checked={ctx.props().is_selected} 
                            onclick={ctx.link().callback(|_| SelectOptionMsg::OnClick)}
                        />
                        <span class="pf-c-check__label">{&self.get_display_string(ctx)}</span>
                        {
                            if has_desc
                            {
                                html!{
                                    <div class="pf-c-check__description">{&ctx.props().description}</div>
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
            _ => {
                // TODO: Implement remaining select variants
                html!{}
            }
        }
    }
}

impl<V: ToString + PartialEq + Clone + 'static> SelectOption<V>
{
    fn get_display_string(&self, ctx: &Context<Self>) -> String
    {
        match &ctx.props().opt_val
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