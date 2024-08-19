//      use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{
    Badge, Button, ButtonVariant, InputGroup, InputGroupItem, TextInputGroup, TextInputGroupMain,
    TextInputGroupUtilities,
};
use icc_patternfly_yew_icons::{angle_down_icon, angle_up_icon, search_icon, times_icon};

#[derive(Clone, PartialEq)]
pub struct SearchInputExpandable {
    /** Flag to indicate if the search input is expanded. */
    pub is_expanded: bool,
    /** Callback function to toggle the expandable search input. */
    pub on_toggle_expand: Callback<bool>, //(event: React.SyntheticEvent<HTMLButtonElement>, isExpanded: boolean) => void
    /** An accessible label for the expandable search input toggle. */
    pub toggle_aria_label: String,
}

pub enum SearchInputMsg {
    OnClickToggle,
    OnClickPrevious,
    OnClickNext,
    OnClickClear,
}

pub struct SearchInput;

#[derive(Clone, PartialEq, Properties)]
pub struct SearchInputProperties {
    // /** Delimiter in the query string for pairing attributes with search values.
    //  * Required whenever attributes are passed as props.
    //  */
    // advancedSearchDelimiter?: string;
    // /** The container to append the menu to.
    //  * If your menu is being cut off you can append it to an element higher up the DOM tree.
    //  * Some examples:
    //  * appendTo={() => document.body}
    //  * appendTo={document.getElementById('target')}
    //  */
    // appendTo?: HTMLElement | (() => HTMLElement) | 'inline';
    /** An accessible label for the search input. */
    #[prop_or_default]
    pub aria_label: Option<String>,
    /** Flag to indicate utilities should be displayed. By default if this prop is undefined or false, utilities will only be displayed when the search input has a value. */
    #[prop_or_default]
    pub are_utilities_displayed: bool,
    // /** Array of attribute values used for dynamically generated advanced search. */
    // attributes?: string[] | SearchInputSearchAttribute[];
    // /** Additional classes added to the search input. */
    #[prop_or_default]
    pub classes: Classes,
    /** Object that makes the search input expandable/collapsible. */
    #[prop_or_default]
    pub expandable_input: Option<SearchInputExpandable>,
    // /* Additional elements added after the attributes in the form.
    // * The new form elements can be wrapped in a form group component for automatic formatting. */
    // formAdditionalItems?: React.ReactNode;
    // /** Attribute label for strings unassociated with one of the provided listed attributes. */
    // hasWordsAttrLabel?: React.ReactNode;
    /** A suggestion for autocompleting. */
    #[prop_or_default]
    pub hint: Option<String>,
    /** Id for the search input */
    #[prop_or_default]
    pub search_input_id: Option<String>,
    // /** @hide A reference object to attach to the input box. */
    // innerRef?: React.RefObject<any>;
    /** A flag for controlling the open state of a custom advanced search implementation. */
    #[prop_or_default]
    pub is_advanced_search_open: bool,
    /** Flag indicating if search input is disabled. */
    #[prop_or_default]
    pub is_disabled: bool,
    /** Flag indicating if the next navigation button is disabled. */
    #[prop_or_default]
    pub is_next_navigation_button_disabled: bool,
    // /** Flag indicating if the previous navigation button is disabled. */
    #[prop_or_default]
    pub is_previous_navigation_button_disabled: bool,
    /** Accessible label for the button to navigate to next result. */
    #[prop_or_default]
    pub next_navigation_button_aria_label: Option<String>,
    /** A callback for when the input value changes. */
    #[prop_or_default]
    pub on_change: Option<Callback<String>>, // (event: React.FormEvent<HTMLInputElement>, value: string) => void;
    /** A callback for when the user clicks the clear button. */
    #[prop_or_default]
    pub on_clear: Option<Callback<()>>, // (event: React.SyntheticEvent<HTMLButtonElement>) => void;
    /** A callback for when the user clicks to navigate to next result. */
    #[prop_or_default]
    pub on_next_click: Option<Callback<()>>, // (event: React.SyntheticEvent<HTMLButtonElement>) => void;
    /** A callback for when the user clicks to navigate to previous result. */
    #[prop_or_default]
    pub on_previous_click: Option<Callback<()>>, // (event: React.SyntheticEvent<HTMLButtonElement>) => void;
    // /** A callback for when the search button is clicked. */
    // onSearch?: (
    //     event: React.SyntheticEvent<HTMLButtonElement>,
    //     value: string,
    //     attrValueMap: { [key: string]: string }
    // ) => void;
    // /** A callback for when the open advanced search button is clicked. */
    // onToggleAdvancedSearch?: (event: React.SyntheticEvent<HTMLButtonElement>, isOpen?: boolean) => void;
    // /** Accessible label for the button which opens the advanced search form menu. */
    // openMenuButtonAriaLabel?: string;
    /** Placeholder text of the search input. */
    #[prop_or_default]
    pub placeholder: Option<String>,
    /** Accessible label for the button to navigate to previous result. */
    #[prop_or_default]
    pub previous_navigation_button_aria_label: Option<String>,
    // /** z-index of the advanced search form when appendTo is not inline. */
    // zIndex?: number;
    /** Label for the button which resets the advanced search form and clears the search input. */
    #[prop_or_default]
    pub reset_button_label: Option<String>,
    /** The number of search results returned. Either a total number of results,
     * or a string representing the current result over the total number of results. i.e. "1 / 5". */
    #[prop_or_default]
    pub results_count: Option<String>,
    /** Label for the button which calls the onSearch event handler. */
    #[prop_or_default]
    pub submit_search_button_label: Option<String>,
    /** Value of the search input. */
    #[prop_or_default]
    pub value: Option<String>,
    /** Name attribute for the search input */
    #[prop_or_default]
    pub name: Option<String>,   
}

impl Component for SearchInput {
    type Message = SearchInputMsg;
    type Properties = SearchInputProperties;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    /// Called everytime when messages are received
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SearchInputMsg::OnClickToggle => {
                if let Some(expandable_input) = &ctx.props().expandable_input {
                    expandable_input
                        .on_toggle_expand
                        .emit(expandable_input.is_expanded);
                    true
                } else {
                    false
                }
            }
            SearchInputMsg::OnClickNext => {
                if let Some(on_next_click) = &ctx.props().on_next_click {
                    on_next_click.emit(());
                    true
                } else {
                    false
                }
            }
            SearchInputMsg::OnClickPrevious => {
                if let Some(on_previous_click) = &ctx.props().on_previous_click {
                    on_previous_click.emit(());
                    true
                } else {
                    false
                }
            }
            SearchInputMsg::OnClickClear => {
                if let Some(on_clear) = &ctx.props().on_clear {
                    on_clear.emit(());
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {   
        if let Some(expandable_input) = &ctx.props().expandable_input {
            html! {
                <InputGroup
                    //{...searchInputProps}
                >
                    <InputGroupItem is_fill={true}>{self.view_text_input_group(ctx)}</InputGroupItem>
                    <InputGroupItem is_plain={true}>{self.expandable_toggle(ctx, expandable_input)}</InputGroupItem>
                </InputGroup>
            }
        } else {
            self.view_text_input_group(ctx)
        }
    }
}

impl SearchInput {
    fn view_text_input_group(&self, ctx: &Context<Self>) -> Html {
        let render_utilities = ctx.props().value.is_some()
            && (ctx.props().results_count.is_some()
                || (ctx.props().on_next_click.is_some()
                    && ctx.props().on_previous_click.is_some())
                || (ctx.props().on_clear.is_some() && ctx.props().expandable_input.is_none()));

        html! {
            <TextInputGroup
                is_disabled={ctx.props().is_disabled}
                //{...searchInputProps}
            >
                <TextInputGroupMain
                    hint={ctx.props().hint.clone()}
                    icon={search_icon!{}}
                    // innerRef={searchInputInputRef}
                    value={ctx.props().value.clone()}
                    placeholder={ctx.props().placeholder.clone()}
                    aria_label={ctx.props().aria_label.clone()}
                    // onKeyDown={onEnter}
                    onchange={ctx.props().on_change.clone()}
                    name={ctx.props().name.clone()}
                    input_id={ctx.props().search_input_id.clone()}
                />
                {
                    if render_utilities || ctx.props().are_utilities_displayed
                    {
                        html!{
                            <TextInputGroupUtilities>
                            {
                                if let Some(results_count) = &ctx.props().results_count
                                {
                                    html!{
                                        <Badge is_read={true}>{results_count.to_string()}</Badge>
                                    }
                                }
                                else {
                                    html!{}
                                }
                            }
                            {
                                if ctx.props().on_next_click.is_some() && ctx.props().on_previous_click.is_some()
                                {
                                    html!{
                                        <div class={classes!("pf-v5-c-text-input-group__group")}>
                                            <Button
                                                variant={ButtonVariant::Plain}
                                                aria_label={ctx.props().previous_navigation_button_aria_label.clone()}
                                                is_disabled={ctx.props().is_disabled || ctx.props().is_previous_navigation_button_disabled}
                                                onclick={ctx.link().callback(|_| SearchInputMsg::OnClickPrevious)}
                                            >
                                                {angle_up_icon!{}}
                                            </Button>
                                            <Button
                                                variant={ButtonVariant::Plain}
                                                aria_label={ctx.props().next_navigation_button_aria_label.clone()}
                                                is_disabled={ctx.props().is_disabled || ctx.props().is_next_navigation_button_disabled}
                                                onclick={ctx.link().callback(|_| SearchInputMsg::OnClickNext)}
                                            >
                                                {angle_down_icon!{}}
                                            </Button>
                                        </div>
                                    }
                                }
                                else {
                                    html!{}
                                }
                            }
                            {
                                if ctx.props().on_clear.is_some() && ctx.props().expandable_input.is_none()
                                {
                                    html!{
                                        <Button
                                            variant={ButtonVariant::Plain}
                                            is_disabled={ctx.props().is_disabled}
                                            aria_label={ctx.props().reset_button_label.clone()}
                                            onclick={ctx.link().callback(|_| SearchInputMsg::OnClickClear)}
                                        >
                                            {times_icon!{}}
                                        </Button>
                                    }
                                }
                                else
                                {
                                    html!{}
                                }
                            }
                            </TextInputGroupUtilities>
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }
                </TextInputGroup>
        }
    }

    fn expandable_toggle(
        &self,
        ctx: &Context<Self>,
        expandable_input: &SearchInputExpandable,
    ) -> Html {
        html! {
            <Button
                variant={ButtonVariant::Plain}
                aria_label={expandable_input.toggle_aria_label.clone()}
                aria_expanded={expandable_input.is_expanded.to_string()}
                icon={if expandable_input.is_expanded {times_icon!{}} else {search_icon!{}}}
                onclick={ctx.link().callback(|_| SearchInputMsg::OnClickToggle)}
            //   ref={searchInputExpandableToggleRef}
            />
        }
    }
}
