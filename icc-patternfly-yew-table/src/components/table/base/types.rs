use yew::prelude::*;

use crate::{ISortBy, OnSort};

// use super::{ISortBy, OnSort};

// // Cell Type
// export interface CellType {
//     property?: number | string;
//     transforms?: transformsType;
//     formatters?: formattersType;
//     props?: object;
//   }
  
//   export interface TdSelectType {
//     /** The selectable variant */
//     variant?: 'checkbox' | 'radio';
//     /** Callback on select */
//     onSelect?: OnSelect;
//     /** Whether the cell is selected */
//     isSelected: boolean;
//     /** Whether the selection is disabled */
//     isDisabled?: boolean;
//     /** The row index */
//     rowIndex: number;
//     /** Additional props forwarded to select rowData */
//     props?: any;
//   }
  
//   export interface TdActionsType {
//     /** The row index */
//     rowIndex?: number;
//     /** Cell actions */
//     items: IActions;
//     /** Whether the actions are disabled */
//     isDisabled?: boolean;
//     /** Actions dropdown position */
//     dropdownPosition?: 'right' | 'left';
//     /** Actions dropdown direction */
//     dropdownDirection?: 'up' | 'down';
//     /** The container to append the dropdown menu to. Defaults to 'inline'.
//      * If your menu is being cut off you can append it to an element higher up the DOM tree.
//      * Some examples:
//      * menuAppendTo="parent"
//      * menuAppendTo={() => document.body}
//      * menuAppendTo={document.getElementById('target')}
//      */
//     menuAppendTo?: HTMLElement | (() => HTMLElement) | 'inline' | 'parent';
//     /** Custom toggle for the actions menu */
//     actionsToggle?: (props: CustomActionsToggleProps) => React.ReactNode;
//   }
  
//   export interface TdExpandType {
//     /** Flag indicating the child row associated with this cell is expanded */
//     isExpanded: boolean;
//     /** The row index */
//     rowIndex: number;
//     /** The column index */
//     columnIndex?: number;
//     /** On toggling the expansion */
//     onToggle?: OnCollapse;
//     /** Id prefix for expandable rows **/
//     expandId?: string;
//   }
  
//   export interface TdCompoundExpandType {
//     /** determines if the corresponding expansion row is open */
//     isExpanded: boolean;
//     /** Callback on toggling of the expansion */
//     onToggle?: OnExpand;
//     /** Id prefix for expandable cells **/
//     expandId?: string;
//     /** The row index */
//     rowIndex?: number;
//     /** The column index */
//     columnIndex?: number;
//   }
  
//   export interface TdFavoritesType {
//     /** Whether the corresponding row is favorited */
//     isFavorited: boolean;
//     /** Callback on clicking the favorites button */
//     onFavorite?: OnFavorite;
//     /** The row index */
//     rowIndex?: number;
//     /** Additional props forwarded to the FavoritesCell */
//     props?: any;
//   }
  
//   export interface TdTreeRowType {
//     /** Callback when user expands/collapses a row to reveal/hide the row's children */
//     onCollapse: OnTreeRowCollapse;
//     /** (optional) Callback when user changes the checkbox on a row */
//     onCheckChange?: OnCheckChange;
//     /** (optional) Callback when user shows/hides the row details in responsive view. */
//     onToggleRowDetails?: OnToggleRowDetails;
//     /** The row index */
//     rowIndex?: number;
//     /** Additional props forwarded to the title cell of the tree row */
//     props?: any;
//   }
  
//   export interface TdDraggableType {
//     /** Id of the draggable row */
//     id: string;
//   }
  
//   // Columns Types
//   export type ColumnsType = ColumnType[] | any[];
  
//   export interface ColumnType {
//     property?: string;
//     cell?: CellType;
//     props?: object;
//     header?: HeaderType;
//   }
//   export interface HeaderType {
//     label?: string;
//     transforms?: transformsType;
//     formatters?: formattersType;
//     props?: object;
//     property?: string;
//     info?: ThInfoType;
//   }
#[derive(Clone, PartialEq)]
pub struct ThInfoType
{
    pub tooltip: Option<Html>,
    // tooltipProps?: Omit<TooltipProps, 'content'>;
    // popover?: React.ReactNode;
    // popoverProps?: Omit<PopoverProps, 'bodyContent'>;
    pub aria_label: Option<String>,
    pub classes: Classes,
}

#[derive(Clone, PartialEq)]
pub struct ThSortType
{
    // /** Wraps the content in a button and adds a sort icon - Click callback on the sortable cell */
    pub onsort: Option<OnSort>,
    // /** Provide the currently active column's index and direction */
    pub sort_by: ISortBy,
    /** The column index */
    pub column_index: i32,
    /** True to make this a favoritable sorting cell */
    pub is_favorites: bool,
}

#[derive(Clone, PartialEq)]
pub struct ThSelectType
{
    // /** Callback on select */
    // onSelect?: OnSelect;
    /** Whether the cell is selected */
    pub is_selected: bool,
    /** Flag indicating the select checkbox in the th is disabled */
    pub is_header_select_disabled: bool,
    /** Whether to disable the selection */
    pub is_disabled: bool,
    // /** Additional props forwarded to select rowData */
    // props?: any;
}

#[derive(Clone, PartialEq)]
pub struct ThExpandType
{
    // /** On toggling the expansion */
    // onToggle?: OnCollapse;
    /** Whether all are expanded */
    pub are_all_expanded: bool,
    /** Alternative aria label */
    pub collapse_all_aria_label: String,
}
  
//   // Rows Types
//   export type RowsType = RowType[] | [][];
//   export type RowKeyType = Function | string;
//   export interface RowType {
//     header?: HeaderType;
//     cell?: CellType;
//     [key: string]: any;
//   }
  
//   // Table Defaults
//   export const TableDefaults = {
//     renderers: {
//       table: Table,
//       header: {
//         wrapper: Thead,
//         row: Tr,
//         cell: Th
//       },
//       body: {
//         wrapper: Tbody,
//         row: Tr,
//         cell: Td
//       }
//     }
//   };
  
//   // Formatters Types
//   export type formatterValueType = object | string | React.ElementType;
//   export interface ExtraParamsType {
//     rowData?: RowType;
//     column?: ColumnType;
//     columnIndex?: number;
//     property?: string;
//     rowIndex?: number;
//     rowKey?: RowKeyType;
//   }
//   export type formatterType = (value: string | object, extra: ExtraParamsType) => formatterValueType;
//   export type formattersType = formatterType[];
  
//   // Transforms Types
//   export type transformType = (value: string | object, extra: ExtraParamsType) => object;
//   export type transformsType = transformType[];
  
//   // Renderers Types
//   export type createElementType = string | React.ComponentClass<any, any> | React.FunctionComponent<any>;
//   export type rendererType =
//     | string
//     | Function
//     | React.ComponentClass<any, any>
//     | React.FunctionComponent<any>
//     | React.Component<any, {}, any>;
//   export interface RendererType {
//     wrapper?: rendererType;
//     row?: rendererType;
//     cell?: rendererType;
//   }
//   export interface RenderersTypes {
//     columns: ColumnsType;
//     renderers?: {
//       table?: any;
//       header?: RendererType;
//       body?: RendererType;
//     };
//     components?: {
//       table?: any;
//       header?: {
//         wrapper?: rendererType;
//         row?: rendererType;
//         cell?: rendererType;
//       };
//       body?: {
//         wrapper?: rendererType;
//         row?: rendererType;
//         cell?: rendererType;
//       };
//     };
//   }
  
//   // Editable cell props
  
//   export interface EditableTextCellProps {
//     /** Name of the input */
//     name: string;
//     /** Value to display in the cell */
//     value: string;
//     /** arbitrary data to pass to the internal text input in the editable text cell */
//     [key: string]: any;
//   }
  
//   export interface EditableSelectInputProps {
//     /** Name of the select input */
//     name: string;
//     /** Value to display in the cell */
//     value: string | string[];
//     /** Flag controlling isOpen state of select */
//     isSelectOpen: boolean;
//     /** Single select option value for single select menus, or array of select option values for multi select. You can also specify isSelected on the SelectOption */
//     selected: any | any[];
//     /** Array of react elements to display in the select menu */
//     options: React.ReactElement[];
//     /** Props to be passed down to the select component */
//     editableSelectProps?: SelectProps;
//     /** arbitrary data to pass to the internal select component in the editable select input cell */
//     [key: string]: any;
//   }