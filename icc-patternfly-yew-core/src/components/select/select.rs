
use yew::{
    html::{ChildrenWithProps},
    prelude::*,
};

use super::*;


pub struct Select<V: ToString + PartialEq + Clone + 'static>
{
    link: ComponentLink<Self>,
    props: SelectProperties<V>,
    menu_ref: NodeRef,
}

pub enum SelectMsg
{
    ToggleList(bool),
}

#[derive(Clone, PartialEq, Properties)]
pub struct SelectProperties<V: ToString + PartialEq + Clone + 'static>
{
    /** Content rendered inside the Select. Must be React.ReactElement<SelectGroupProps>[] */
    #[prop_or_default]
    pub children: ChildrenWithProps<SelectOption<V>>,
    /** Classes applied to the root of the Select */
    #[prop_or_default]
    pub class_name: String,
    // /** Flag specifying which direction the Select menu expands */
    // pub direction: 'up' | 'down';
    /** Flag to indicate if select is open */
    #[prop_or_default]
    pub is_open: bool,
    /** Flag to indicate if select options are grouped */
    #[prop_or_default]
    pub is_grouped: bool,
    /** Display the toggle with no border or background */
    #[prop_or_default]
    pub is_plain: bool,
    /** Flag to indicate if select is disabled */
    #[prop_or_default]
    pub is_disabled: bool,
    /** Flag to indicate if the typeahead select allows new items */
    #[prop_or_default]
    pub is_creatable: bool,
    /** Text displayed in typeahead select to prompt the user to create an item */
    #[prop_or_default]
    pub create_text: String,
    /** Title text of Select */
    #[prop_or_default]
    pub placeholder_text: Html,
    /** Text to display in typeahead select when no results are found */
    #[prop_or_default]
    pub no_results_found_text: String,
    /** Array of selected items for multi select variants. */
    #[prop_or_default]
    pub selections: Vec<SelectOptionValue<V>>,
    /** Flag indicating if selection badge should be hidden for checkbox variant,default false */
    #[prop_or_default]
    pub is_checkbox_selection_badge_hidden: bool,
    /** Id for select toggle element */
    #[prop_or_default]
    pub toggle_id: String,
    /** Adds accessible text to Select */
    #[prop_or_default]
    pub aria_label: String,
    /** Id of label for the Select aria-labelledby */
    #[prop_or_default]
    pub aria_labelledby: String,
    /** Label for input field of type ahead select variants */
    #[prop_or_default]
    pub type_ahead_aria_label: String,
    /** Label for clear selection button of type ahead select variants */
    #[prop_or_default]
    pub clear_selections_aria_label: String,
    /** Label for toggle of type ahead select variants */
    #[prop_or_default]
    pub toggle_aria_label: String,
    /** Label for remove chip button of multiple type ahead select variant */
    #[prop_or_default]
    pub remove_selection_aria_label: String,
    /** ID list of favorited select items */
    #[prop_or_default]
    pub favorites: Vec<String>,
    /** Label for the favorites group */
    #[prop_or_default]
    pub favorites_label: String,
    // /** Enables favorites. Callback called when a select options's favorite button is clicked */
    // #[prop_or_default]
    // pub onfavorite: (itemId: string, isFavorite: boolean) => void;
    /** Callback for selection behavior */
    #[prop_or_default]
    // onSelect: (
    //     event: React.MouseEvent | React.ChangeEvent,
    //     value: string | SelectOptionObject,
    //     isPlaceholder: boolean
    // ) => void;
    pub onselect: Callback<SelectOptionValue<V>>,
    /** Callback for toggle button behavior */
    #[prop_or_default]
    pub ontoggle: Callback<bool>,
    // /** Callback for typeahead clear button */
    // #[prop_or_default]
    // pub onclear: (event: React.MouseEvent) => void;
    // /** Optional callback for custom filtering */
    // #[prop_or_default]
    // pub onfilter: (e: React.ChangeEvent<HTMLInputElement>) => React.ReactElement[];
    // /** Optional callback for newly created options */
    // #[prop_or_default]
    // pub oncreateoption: (newOptionValue: string) => void;
    // /** Optional event handler called each time the value in the typeahead input changes. */
    // #[prop_or_default]
    // pub ontypeaheadinputchanged: (value: string) => void;
    /** Variant of rendered Select */
    #[prop_or(SelectVariant::Single)]
    pub variant: SelectVariant,
    /** Width of the select container as a number of px or string percentage */
    #[prop_or_default]
    pub width: i32,
    /** Max height of the select container as a number of px or string percentage */
    #[prop_or_default]
    pub max_height: i32,
    /** Icon element to render inside the select toggle */
    #[prop_or_default]
    pub toggle_icon: Option<Html>,
    /** Custom content to render in the select menu.  If this prop is defined, the variant prop will be ignored and the select will render with a single select toggle */
    #[prop_or_default]
    pub custom_content: Option<Html>,
    /** Flag indicating if select should have an inline text input for filtering */
    #[prop_or_default]
    pub has_inline_filter: bool,
    /** Placeholder text for inline filter */
    #[prop_or_default]
    pub inline_filter_placeholder_text: String,
    /** Custom text for select badge */
    #[prop_or_default]
    pub custom_badge_text: String,
    /** Prefix for the id of the input in the checkbox select variant*/
    #[prop_or_default]
    pub input_id_prefix: String,
    // /** Optional props to pass to the chip group in the typeaheadmulti variant */
    // #[prop_or_default]
    // pub chipGroupProps: Omit<ChipGroupProps, 'children' | 'ref'>;
    /** Optional props to render custom chip group in the typeaheadmulti variant */
    #[prop_or_default]
    pub chip_group_component: Option<Html>,
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
                    id=self.props.toggle_id.clone()
                    menu_ref=self.menu_ref.clone()
                    is_open=self.props.is_open
                    is_plain=self.props.is_plain
                    is_disabled=self.props.is_disabled
                    ontoggle=self.link.callback(|is_open| SelectMsg::ToggleList(is_open))
                    variant=self.props.variant.clone()
                    aria_labelledby=self.props.aria_labelledby.clone()
                    aria_label=self.props.aria_label.clone()
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
                                    <span class="pf-c-select__toggle-text">{ self.props.placeholder_text.clone() }</span>
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
                        _ => {
                            // TODO: Implement remaining select variants
                            html!{}
                        }
                    }

                }
                </SelectToggle>
                {
                    if self.props.is_open
                    {
                        html!{
                            <SelectMenu
                                ref=self.menu_ref.clone()
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

    fn get_display_string(&self) -> Html
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
                            return html!{value};
                        },
                        SelectOptionValue::Object(obj) => {
                            return html!{obj.to_string()};
                        },
                        SelectOptionValue::ObjRef(obj) => {
                            return html!{obj.borrow().to_string()};
                        }
                    }
                }
                
            }
        }

        // Use placeholder text if there is a placeholder
        self.props.placeholder_text.clone()
    }
}
