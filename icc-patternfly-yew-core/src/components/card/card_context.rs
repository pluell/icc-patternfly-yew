use yew::AttrValue;


#[derive(Clone, PartialEq)]
pub struct CardContext
{
    pub card_id: AttrValue,
    // pub register_title_id: Callback<String>,
    pub is_expanded: bool,
    pub is_clickable: bool,
    pub is_selectable: bool,
    pub is_selected: bool,
    pub is_clicked: bool,
    pub is_disabled: bool,
  }