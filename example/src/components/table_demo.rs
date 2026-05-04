use yew::prelude::*;

use icc_patternfly_yew_core::{
    PageSection, PageSectionVariants, SearchInput, Select, SelectOption,
    SelectOptionValue, SelectVariant, Text, TextContent, TextVariants,
    Toolbar, ToolbarContent, ToolbarFilter, ToolbarGroup, ToolbarGroupVariant, ToolbarItem,
    ToolbarItemVariant,
};
use icc_patternfly_yew_table::{
    CellWidth, Table, TableVariant, Tbody, Td, Th, Thead, Tr,
};

use super::form_demo::FormDemoStatus;

struct TableDemoRow {
    name: String,
    status: FormDemoStatus,
    value: i32,
    another_field: String,
}

fn initialize_demo_rows() -> Vec<TableDemoRow> {
    vec![
        TableDemoRow {
            name: String::from("The first one"),
            status: FormDemoStatus::StatusOne,
            value: 10,
            another_field: String::from(""),
        },
        TableDemoRow {
            name: String::from("Next one"),
            status: FormDemoStatus::StatusOne,
            value: 6531,
            another_field: String::from("This one has extra data"),
        },
        TableDemoRow {
            name: String::from("Demo Row C"),
            status: FormDemoStatus::AnotherStatus,
            value: 6542,
            another_field: String::from("Another field C"),
        },
        TableDemoRow {
            name: String::from("Data row D"),
            status: FormDemoStatus::LastStatus,
            value: 0,
            another_field: String::from(""),
        },
        TableDemoRow {
            name: String::from("Data row E"),
            status: FormDemoStatus::SecondStatus,
            value: 6,
            another_field: String::from(""),
        },
        TableDemoRow {
            name: String::from("Data row F"),
            status: FormDemoStatus::Disabled,
            value: 753,
            another_field: String::from(""),
        },
        TableDemoRow {
            name: String::from("Data row G"),
            status: FormDemoStatus::LastStatus,
            value: 10123,
            another_field: String::from(""),
        },
    ]
}

pub struct TableDemo {
    search_filter: Option<String>,
    status_filter: Vec<FormDemoStatus>,
    data: Vec<TableDemoRow>,
    status_filter_is_open: bool,
}

pub enum Msg {
    OnChangeDescriptionFilter(String),
    ClearDescriptionFilter,
    ClearAllFilters,
    ToggleFilterStatus(bool),
    SelectFilterStatus(FormDemoStatus),
    RemoveFilterStatus(String),
    ClearStatusFilter,
    NoOp,
}

impl Component for TableDemo {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            search_filter: None,
            status_filter: Vec::new(),
            data: initialize_demo_rows(),
            status_filter_is_open: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::OnChangeDescriptionFilter(text_filter) => {
                if text_filter.len() == 0 {
                    self.search_filter = None;
                } else {
                    self.search_filter = Some(text_filter);
                }
            }
            Self::Message::ClearDescriptionFilter => {
                self.search_filter = None;
            }
            Self::Message::ClearAllFilters => {
                self.search_filter = None;
                self.status_filter.clear();
            }
            Self::Message::ToggleFilterStatus(is_open) => {
                self.status_filter_is_open = is_open;
            }
            Self::Message::SelectFilterStatus(status) => {
                if let Some(index) = self
                    .status_filter
                    .iter()
                    .position(|status_filter| status_filter == &status)
                {
                    self.status_filter.remove(index);
                } else {
                    self.status_filter.push(status);
                }
            }
            Self::Message::RemoveFilterStatus(status) => {
                if let Some(index) = self
                    .status_filter
                    .iter()
                    .position(|status_filter| status_filter.to_string() == status)
                {
                    self.status_filter.remove(index);
                }
            }
            Self::Message::ClearStatusFilter => {
                self.status_filter.clear();
            }
            Self::Message::NoOp => {
                return false;
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <PageSection variant={PageSectionVariants::Light}>
                <TextContent>
                    <Text component={TextVariants::H1}>{"Table Demo"}</Text>
                </TextContent>
            </PageSection>
            <PageSection>
                <Toolbar clear_all_filters={ctx.link().callback(|_| Msg::ClearAllFilters)}>
                    <ToolbarContent>
                        <ToolbarItem variant={ToolbarItemVariant::SearchFilter}>
                            <SearchInput
                                placeholder="Description"
                                value={self.search_filter.clone()}
                                on_change={ctx.link().callback(|value| Msg::OnChangeDescriptionFilter(value))}
                                on_clear={ctx.link().callback(|_| Msg::ClearDescriptionFilter)}
                            />
                        </ToolbarItem>
                        <ToolbarGroup variant={ToolbarGroupVariant::FilterGroup}>
                            <ToolbarFilter
                                chips={self.status_filter.iter().map(|status| status.to_string()).collect::<Vec<String>>()}
                                category_name="Status"
                                delete_chip={ctx.link().callback(|(_, chip)| Msg::RemoveFilterStatus(chip))}
                                delete_chip_group={ctx.link().callback(|_| Msg::ClearStatusFilter)}
                            >
                                <Select<FormDemoStatus>
                                    is_open={self.status_filter_is_open}
                                    variant={SelectVariant::Checkbox}
                                    selections={self.status_filter.iter().map(|filter| SelectOptionValue::Object(filter.clone())).collect::<Vec<SelectOptionValue<FormDemoStatus>>>()}
                                    placeholder_text="Status"
                                    ontoggle={ctx.link().callback(|is_open| Msg::ToggleFilterStatus(is_open))}
                                    onselect={ctx.link().callback(|value|
                                        match value
                                        {
                                            SelectOptionValue::Object(status) => Msg::SelectFilterStatus(status),
                                            _ => Msg::NoOp,
                                        }

                                    )}
                                >
                                    <SelectOption<FormDemoStatus> opt_val={SelectOptionValue::Object(FormDemoStatus::StatusOne)} />
                                    <SelectOption<FormDemoStatus> opt_val={SelectOptionValue::Object(FormDemoStatus::SecondStatus)} />
                                    <SelectOption<FormDemoStatus> opt_val={SelectOptionValue::Object(FormDemoStatus::AnotherStatus)} />
                                    <SelectOption<FormDemoStatus> opt_val={SelectOptionValue::Object(FormDemoStatus::LastStatus)} />
                                    <SelectOption<FormDemoStatus> opt_val={SelectOptionValue::Object(FormDemoStatus::Disabled)} />
                                </Select<FormDemoStatus>>
                            </ToolbarFilter>
                        </ToolbarGroup>
                    </ToolbarContent>
                </Toolbar>
                <Table variant={TableVariant::Compact}>
                    <Thead>
                        <Tr>
                            <Th width={CellWidth::Width20}>{"Name"}</Th>
                            <Th width={CellWidth::Width10}>{"Status"}</Th>
                            <Th>{"Value"}</Th>
                            <Th>{"Another field"}</Th>
                        </Tr>
                    </Thead>
                    <Tbody>
                        for row in self.data.iter().filter(|row| self.status_filter.is_empty() || self.status_filter.contains(&row.status)) {
                            <Tr>
                                <Td>{&row.name}</Td>
                                <Td >{row.status.to_string()}</Td>
                                <Td>{row.value}</Td>
                                <Td>{&row.another_field}</Td>
                            </Tr>
                        }
                    </Tbody>
                </Table>
            </PageSection>
            </>
        }
    }
}
