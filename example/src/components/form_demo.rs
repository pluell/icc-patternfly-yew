use yew::prelude::*;

use icc_patternfly_yew_core::{
    Alert, AlertVariant, Button, ButtonVariant, Card, CardBody, CardTitle, Checkbox, Form,
    FormAlert, FormFieldGroup, FormFieldGroupHeader, FormFiledGroupHeaderTitleTextObject,
    FormGroup, FormHelperText, FormHelperTextTypes, FormSelect, FormSelectOption, HelperText,
    HelperTextItem, Modal, ModalVariants, PageSection, PageSectionVariants, Radio, Tab,
    TabTextTitle, Tabs, Text, TextContent, TextInput, TextInputType, TextVariants, Title,
    TitleHeadingLevels, ValidatedOptions,
};

#[derive(strum_macros::Display, strum_macros::EnumString, Clone, PartialEq)]
pub enum FormDemoStatus {
    StatusOne,
    SecondStatus,
    AnotherStatus,
    LastStatus,
    Disabled,
}

pub struct FormDemo {
    cur_tab: String,
    name: String,
    phone: String,
    status: FormDemoStatus,
    modal_is_open: bool,
}

pub enum Msg {
    OnSelectTab(String),
    OnChangeName(String),
    OnChangePhone(String),
    OnSelectStatus(String),
    OnClickAddParameter,
    CloseDialog,
}

impl Component for FormDemo {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            cur_tab: String::from("ContactInfo"),
            name: String::new(),
            phone: String::new(),
            status: FormDemoStatus::StatusOne,
            modal_is_open: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::OnSelectTab(tab_name) => {
                self.cur_tab = tab_name;
            }
            Self::Message::OnChangeName(name) => {
                self.name = name;
            }
            Self::Message::OnChangePhone(phone) => {
                self.phone = phone;
            }
            Self::Message::OnSelectStatus(status) => {
                if let Ok(status) = FormDemoStatus::try_from(status.as_str()) {
                    self.status = status;
                }
            }
            Self::Message::OnClickAddParameter => {
                self.modal_is_open = true;
            }
            Self::Message::CloseDialog => {
                self.modal_is_open = false;
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <PageSection variant={PageSectionVariants::Light}>
                <TextContent>
                    <Text component={TextVariants::H1}>{"Form Demo"}</Text>
                </TextContent>
            </PageSection>
            <PageSection>
                <div class={"pf-v5-u-w-50-on-lg"}>
                    <Tabs
                        is_box={true}
                        active_key={self.cur_tab.clone()}
                        onselect={ctx.link().callback(Msg::OnSelectTab)}
                    >
                        <Tab
                            event_key="ContactInfo"
                            title={html!{<TabTextTitle>{"Contact Info"}</TabTextTitle>}}
                        >
                            <Card>
                                <CardTitle><Title heading_level={TitleHeadingLevels::H2}>{"Contact Info"}</Title></CardTitle>
                                <CardBody>
                                    <Form>
                                        <FormGroup
                                            label="Full name"
                                            field_id="form-demo-full-name"
                                            is_required={true}
                                        >
                                            <TextInput
                                                is_required={true}
                                                id="form-demo-full-name"
                                                value={self.name.clone()}
                                                onchange={ctx.link().callback(Msg::OnChangeName)}
                                            />
                                            <FormHelperText>
                                                <HelperText>
                                                    <HelperTextItem>{"Include your middle name if you have one."}</HelperTextItem>
                                                </HelperText>
                                            </FormHelperText>
                                        </FormGroup>
                                        <FormGroup label="Phone number" is_required={true} field_id="demo-form-phone">
                                            <TextInput
                                                is_required={true}
                                                input_type={TextInputType::Tel}
                                                id="demo-form-phone"
                                                placeholder="555-555-5555"
                                                value={self.phone.clone()}
                                                onchange={ctx.link().callback(Msg::OnChangePhone)}
                                            />
                                        </FormGroup>
                                        <FormGroup is_inline={true} field_id="basic-form-checkbox-group" label="How can we contact you?">
                                            <Checkbox label="Email" aria_label="Email" id="inlinecheck01" />
                                            <Checkbox label="Phone" aria_label="Phone" id="inlinecheck02" />
                                            <Checkbox label="Mail" aria_label="Mail" id="inlinecheck03" />
                                        </FormGroup>
                                        <FormGroup is_inline={true} field_id="basic-form-radio-group" label="Time zone">
                                            <Radio name="basic-inline-radio" label="Eastern" id="basic-inline-radio-01" />
                                            <Radio name="basic-inline-radio" label="Central" id="basic-inline-radio-02" />
                                            <Radio name="basic-inline-radio" label="Pacific" id="basic-inline-radio-03" />
                                        </FormGroup>
                                        <FormGroup label="Additional note" field_id="simple-form-note-01">
                                            <TextInput is_disabled={true} id="simple-form-note-01" value="disabled" />
                                        </FormGroup>
                                        <FormGroup field_id="checkbox01">
                                            <Checkbox label="I'd like updates via email." id="checkbox01" aria_label="Update via email" />
                                        </FormGroup>
                                    </Form>
                                </CardBody>
                            </Card>
                        </Tab>
                        <Tab
                            event_key="HorizontalForm"
                            title={html!{<TabTextTitle>{"Horizontal Form"}</TabTextTitle>}}
                        >
                            <Card>
                                <CardTitle><Title heading_level={TitleHeadingLevels::H2}>{"Horizontal Form Demo"}</Title></CardTitle>
                                <CardBody>
                                    <Form is_horizontal={true}>
                                        <FormAlert>
                                            <Alert variant={AlertVariant::Danger} title="Demo form alert" is_inline={true} />
                                        </FormAlert>
                                        <FormGroup label="Status" field_id="horizontal-form-title">
                                            <FormSelect
                                                value={self.status.to_string()}
                                                onchange={ctx.link().callback(Self::Message::OnSelectStatus)}
                                                id="horizontal-form-title"
                                                aria_label="Your title"
                                            >
                                                <FormSelectOption is_disabled={false} key={1} value={FormDemoStatus::StatusOne.to_string()} label={"Status One"} />
                                                <FormSelectOption key={2} value={FormDemoStatus::SecondStatus.to_string()} label={"Second Status"} />
                                                <FormSelectOption key={1} value={FormDemoStatus::AnotherStatus.to_string()} label={"Another Status"} />
                                                <FormSelectOption is_disabled={true} key={1} value={FormDemoStatus::LastStatus.to_string()} label={"Last Status"} />
                                                <FormSelectOption value={FormDemoStatus::Disabled.to_string()} label={"Disabled"} />
                                            </FormSelect>
                                        </FormGroup>
                                        <FormFieldGroup
                                            header={html!{
                                                <FormFieldGroupHeader
                                                    title_text={FormFiledGroupHeaderTitleTextObject{
                                                        text: html!{<Title heading_level={TitleHeadingLevels::H4}>{"Field group (non-expandable)"}</Title>},
                                                        id: String::from("field-group-non-expandable-titleText-id")}
                                                    }
                                                    title_description="Field group description text."
                                                    actions={html!{
                                                        <>
                                                            <Button variant={ButtonVariant::Link} onclick={Callback::noop()}>{"Delete all"}</Button>
                                                            <Button variant={ButtonVariant::Secondary} onclick={ctx.link().callback(|_| Msg::OnClickAddParameter)}>{"Add parameter"}</Button>
                                                        </>
                                                    }}
                                                />
                                            }}
                                        >
                                            <FormGroup
                                                label="Label 1" is_required={true} field_id="4-nonexpand-label1"
                                                helper_text={Some(FormHelperTextTypes::FormHelperText(html_nested!{
                                                    <FormHelperText
                                                        icon={icc_patternfly_yew_icons::exclamation_circle_icon!{}}
                                                    >
                                                        {"Helper text for this field"}
                                                    </FormHelperText>
                                                }))}
                                            >
                                            <TextInput
                                                is_required={true}
                                                id="4-nonexpand-label1"
                                                value="Validated field"
                                                validated={ValidatedOptions::Success}
                                            />
                                            </FormGroup>
                                            <FormGroup
                                                label="Label 2" is_required={true} field_id="4-nonexpand-label2"
                                                helper_text={Some(FormHelperTextTypes::FormHelperText(html_nested!{
                                                    <FormHelperText
                                                        is_error={true}
                                                        icon={icc_patternfly_yew_icons::exclamation_circle_icon!{}}
                                                    >
                                                        {"Please enter a valid value"}
                                                    </FormHelperText>
                                                }))}
                                            >
                                                <TextInput
                                                    is_required={true}
                                                    id="4-nonexpand-label2"
                                                    value="Invalid field"
                                                    validated={ValidatedOptions::Error}
                                                />
                                            </FormGroup>
                                        </FormFieldGroup>
                                    </Form>
                                </CardBody>
                            </Card>
                        </Tab>
                    </Tabs>
                </div>
            </PageSection>
            <Modal
                is_open={self.modal_is_open}
                onclose={ctx.link().callback(|_| Msg::CloseDialog)}
                title="Add Parameter Modal"
                variant={ModalVariants::Medium}
            >
                {"Modal Dialog"}
            </Modal>
            </>
        }
    }
}
