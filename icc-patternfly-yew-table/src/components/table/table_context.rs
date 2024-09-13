use yew::Callback;


#[derive(Clone, PartialEq)]
pub struct TableContext
{
    pub register_selectable_row: Callback<()>,
}