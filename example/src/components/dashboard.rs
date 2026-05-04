use yew::prelude::*;

use icc_patternfly_yew_core::{
    Card, CardBody, CardTitle, EmptyState, EmptyStateHeader, EmptyStateIcon, PageSection,
    PageSectionVariants, Spinner, Text, TextContent, TextVariants, Title, TitleHeadingLevels,
};
use icc_patternfly_yew_table::{Table, TableGridBreakpoint, TableVariant, Tbody, Td, Th, Tr};

pub struct Dashboard {}

impl Component for Dashboard {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <>
            <PageSection variant={PageSectionVariants::Light}>
                <TextContent>
                    <Text component={TextVariants::H1}>{"Dashboard"}</Text>
                </TextContent>
            </PageSection>
            <PageSection>
                <div class={"pf-v5-l-grid pf-m-gutter pf-m-all-12-col-on-sm pf-m-all-6-col-on-md pf-m-all-4-col-on-lg"}>
                    <Card>
                        <CardTitle><Title heading_level={TitleHeadingLevels::H1}>{"Product Info"}</Title></CardTitle>
                        <CardBody>
                            <Table
                                variant={TableVariant::Compact}
                                grid_break_point={TableGridBreakpoint::None}
                            >
                                <Tbody>
                                    <Tr>
                                        <Th>{"Product name"}</Th>
                                        <Td>{"Demo Product"}</Td>
                                    </Tr>
                                    <Tr>
                                        <Th>{"Description"}</Th>
                                        <Td>{"The product for the demo description"}</Td>
                                    </Tr>
                                    <Tr>
                                        <Th>{"Vendor"}</Th>
                                        <Td>{"Demo Vendor"}</Td>
                                    </Tr>
                                    <Tr>
                                        <Th>{"Serial number"}</Th>
                                        <Td>{"123ABC"}</Td>
                                    </Tr>
                                </Tbody>
                            </Table>
                        </CardBody>
                    </Card>
                    <Card>
                        <CardTitle><Title heading_level={TitleHeadingLevels::H1}>{"Resources"}</Title></CardTitle>
                        <CardBody>
                            <Table
                                variant={TableVariant::Compact}
                                grid_break_point={TableGridBreakpoint::None}
                            >
                                <Tbody>
                                    <Tr>
                                        <Th>{"CPU"}</Th>
                                        <Td>{"Demo CPU"}</Td>
                                    </Tr>
                                    <Tr>
                                        <Th>{"RAM"}</Th>
                                        <Td>{"Lots 'o RAM"}</Td>
                                    </Tr>
                                    <Tr>
                                        <Th>{"Storage"}</Th>
                                        <Td>{"12345"}</Td>
                                    </Tr>
                                </Tbody>
                            </Table>
                        </CardBody>
                    </Card>
                    <Card>
                        <CardTitle><Title heading_level={TitleHeadingLevels::H1}>{"More Demo Info"}</Title></CardTitle>
                        <CardBody>
                            <Table
                                variant={TableVariant::Compact}
                                grid_break_point={TableGridBreakpoint::None}
                            >
                                <Tbody>
                                    <Tr>
                                        <Th>{"What is this?"}</Th>
                                        <Td>{"Demo more info"}</Td>
                                    </Tr>
                                    <Tr>
                                        <Th>{"Testing demo info"}</Th>
                                        <Td>{"Demo testing info"}</Td>
                                    </Tr>
                                    <Tr>
                                        <Th>{"More"}</Th>
                                        <Td>{"Less"}</Td>
                                    </Tr>
                                </Tbody>
                            </Table>
                        </CardBody>
                    </Card>
                    <Card>
                        <CardTitle><Title heading_level={TitleHeadingLevels::H1}>{"Waiting for info"}</Title></CardTitle>
                        <CardBody>
                            <EmptyState>
                                <EmptyStateHeader title_text="Loading" heading_level={TitleHeadingLevels::H4} icon={html!{<EmptyStateIcon icon={html!{<Spinner is_svg={true} />}} />}} />
                            </EmptyState>
                        </CardBody>
                    </Card>
                </div>
            </PageSection>
            </>
        }
    }
}
