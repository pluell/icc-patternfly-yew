
use yew::prelude::*;

use crate::{DecoratorReturnType, IExtra, IExtraColumnData, ISortBy, OnSort, OnSortParams, SortByDirection, SortColumn};


pub fn sortable(children: &Html, extra_data: IExtra) -> DecoratorReturnType
{
    let mut is_sorted_by = false;
    let mut onsort = None;
    let mut sort_by = None;

    if let Some(column) = &extra_data.extra_column_data.column
    {
        onsort = column.onsort.clone();
        sort_by = column.sort_by.clone();

        is_sorted_by = if let Some(sort_by) = &column.sort_by {
            sort_by.index == extra_data.extra_column_data.column_index
        } else {
            false
        };
    }
    
    let children = Some(html!{
        <SortableHeader
            {is_sorted_by}
            extra_column_data={extra_data.extra_column_data.clone()}
            {onsort}
            {sort_by}
        >
            {children.clone()}
        </SortableHeader>
    });

    DecoratorReturnType {
        classes: Some(classes!("pf-v5-c-table__sort", if is_sorted_by {"pf-m-selected"} else {""}, extra_data.classes.clone())),
        // aria_sort: Some(), // 'aria-sort': isSortedBy ? `${sortBy.direction}ending` : 'none',
        children,
        ..Default::default()
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct SortableHeaderProps
{
    children: Html,
    is_sorted_by: bool,
    extra_column_data: IExtraColumnData,
    onsort: Option<OnSort>,
    sort_by: Option<ISortBy>,
}

#[function_component]
fn SortableHeader(props: &SortableHeaderProps) -> Html
{
    let sort_clicked = {
        let column_index = props.extra_column_data.column_index.unwrap_or(-1).clone(); // props.column_index;
        let onsort = props.onsort.clone();
        let sort_by = props.sort_by.clone();
        let is_sorted_by = props.is_sorted_by.clone();

        let extra_data = props.extra_column_data.clone();

        Callback::from(move |_| {
            let mut reversed_direction = SortByDirection::Asc;

            if let Some(sort_by) = &sort_by
            {
                if !is_sorted_by
                {
                    if let Some(default_direction) = &sort_by.default_direction
                    {
                        reversed_direction = default_direction.clone();
                    }
                }
                else
                {
                    if let Some(direction) = &sort_by.direction
                    {
                        if direction == &SortByDirection::Asc
                        {
                            reversed_direction = SortByDirection::Desc;
                        }
                    }
                }
            }

            if let Some(onsort) = &onsort {
                onsort.emit(OnSortParams{
                    column_index, 
                    sort_by_direction: reversed_direction,
                    extra_data: extra_data.clone()
                });
            }}
        )
    };

    html!{
        <SortColumn
            is_sorted_by={props.is_sorted_by}
            sort_direction={if props.is_sorted_by {props.sort_by.as_ref().unwrap().direction.clone()} else {None}}
            onsort={Some(sort_clicked)}
            // aria-label={ariaLabel}
            // tooltip={tooltip}
            // tooltipProps={tooltipProps}
            // tooltipHasDefaultBehavior={tooltipHasDefaultBehavior}
        >
            {props.children.clone()}
        </SortColumn>
    }
}