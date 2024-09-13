
use yew::prelude::*;

use super::SortByDirection;

#[derive(Clone, PartialEq)]
pub enum TableGridBreakpoint
{
    None,
    Grid,
    GridMd,
    GridLg,
    GridXl,
    Grid2xl,
}

impl TableGridBreakpoint
{
    pub fn get_class(&self) -> &'static str
    {
        match self
        {
            TableGridBreakpoint::None => "",
            TableGridBreakpoint::Grid => "pf-m-grid",
            TableGridBreakpoint::GridMd => "pf-m-grid-md",
            TableGridBreakpoint::GridLg => "pf-m-grid-lg",
            TableGridBreakpoint::GridXl => "pf-m-grid-xl",
            TableGridBreakpoint::Grid2xl => "pf-m-grid-2xl",
        }
    }

    pub fn get_tree_grid_class(&self) -> &'static str
    {
        match self
        {
            TableGridBreakpoint::None => "",
            TableGridBreakpoint::Grid => "pf-m-tree-view-grid",
            TableGridBreakpoint::GridMd => "pf-m-tree-view-grid-md",
            TableGridBreakpoint::GridLg => "pf-m-tree-view-grid-lg",
            TableGridBreakpoint::GridXl => "pf-m-tree-view-grid-xl",
            TableGridBreakpoint::Grid2xl => "pf-m-tree-view-grid-2xl",
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum TableVariant
{
    Compact
}

impl TableVariant
{
    pub fn get_class(&self) -> &'static str
    {
        match self
        {
            TableVariant::Compact => "pf-m-compact",
        }
    }
}

// export type RowEditType = 'save' | 'cancel' | 'edit';

// export interface RowErrors {
//   [name: string]: string[];
// }

pub struct OnSortParams
{
    // event: React.MouseEvent, 
    pub column_index: i32,
    pub sort_by_direction: SortByDirection,
    pub extra_data: IExtraColumnData
}
pub type OnSort = Callback<OnSortParams>;
// export type OnCollapse = (
//   event: React.MouseEvent,
//   rowIndex: number,
//   isOpen: boolean,
//   rowData: IRowData,
//   extraData: IExtraData
// ) => void;
// export type OnExpand = (
//   event: React.MouseEvent,
//   rowIndex: number,
//   colIndex: number,
//   isOpen: boolean,
//   rowData: IRowData,
//   extraData: IExtraData
// ) => void;
// export type OnSelect = (
//   event: React.FormEvent<HTMLInputElement>,
//   isSelected: boolean,
//   rowIndex: number,
//   rowData: IRowData,
//   extraData: IExtraData
// ) => void;
// export type OnRowEdit = (
//   event: React.MouseEvent<HTMLButtonElement>,
//   type: RowEditType,
//   isEditable?: boolean,
//   rowIndex?: number,
//   validationErrors?: RowErrors
// ) => void;
// export type OnFavorite = (
//   event: React.MouseEvent,
//   isFavorited: boolean,
//   rowIndex: number,
//   rowData: IRowData,
//   extraData: IExtraData
// ) => void;

// export type OnTreeRowCollapse = (event: any, rowIndex: number, title: React.ReactNode, rowData: IRowData) => void;
// export type OnToggleRowDetails = (event: any, rowIndex: number, title: React.ReactNode, rowData: IRowData) => void;
// export type OnCheckChange = (
//   event: React.FormEvent<HTMLInputElement>,
//   isChecked: boolean,
//   rowIndex: number,
//   title: React.ReactNode,
//   rowData: IRowData
// ) => void;

// // Todo: Update type with next breaking change release
// // export type IHeaderRow = ColumnType;

// export interface IHeaderRow extends ColumnType {}

// export interface IRowData extends IRow {
//   disableActions?: boolean;
// }

#[derive(Default, Clone, PartialEq)]
pub struct IColumn
{
//   extraParams: {
    pub sort_by: Option<ISortBy>,
    pub onsort: Option<OnSort>,
//     onCollapse?: OnCollapse;
//     onExpand?: OnExpand;
//     onSelect?: OnSelect;
//     selectVariant?: 'checkbox' | 'radio';
//     collapseAllAriaLabel?: string;
//     onRowEdit?: OnRowEdit;
//     rowLabeledBy?: string;
//     expandId?: string;
//     contentId?: string;
//     dropdownPosition?: DropdownPosition;
//     dropdownDirection?: DropdownDirection;
//     menuAppendTo?: HTMLElement | (() => HTMLElement) | 'inline' | 'parent';
//     actionsToggle?: (props: CustomActionsToggleProps) => React.ReactNode;
//     actionsPopperProps?: any;
//     allRowsSelected?: boolean;
//     allRowsExpanded?: boolean;
//     isHeaderSelectDisabled?: boolean;
//     onFavorite?: OnFavorite;
//   };
}

// export interface IExtraRowData {
//   rowIndex?: number;
//   rowKey?: RowKeyType;
//   id?: string;
// }

#[derive(Default, Clone, PartialEq)]
pub struct IExtraColumnData
{
  pub column_index: Option<i32>,
  pub column: Option<IColumn>,
  pub property: Option<String>,
}

// export interface IExtraData extends IExtraColumnData, IExtraRowData {}

// export interface IExtra extends IExtraData {
#[derive(Default)]
pub struct IExtra
{
//   rowData?: IRowData;
    pub classes: Option<Classes>,
//   ariaLabel?: string;
//   tooltip?: React.ReactNode;
//   tooltipProps?: Omit<TooltipProps, 'content'>;
//   tooltipHasDefaultBehavior?: boolean;
    pub extra_column_data: IExtraColumnData,
}

// export type IFormatterValueType = formatterValueType & {
//   title?: React.ReactNode;
//   props?: any;
// };

#[derive(Clone, PartialEq)]
pub struct ISortBy
{
  /** Index of the current sorted column */
  pub index: Option<i32>,
  /** Current sort direction */
  pub direction: Option<SortByDirection>,
  /** Defaulting sorting direction. Defaults to "asc". */
  pub default_direction: Option<SortByDirection>,
}

// export interface IAction extends Omit<DropdownItemProps, 'title' | 'onClick'>, Pick<ButtonProps, 'variant'> {
//   /** Flag indicating an item on actions menu is a separator, rather than an action */
//   isSeparator?: boolean;
//   /** Key of actions menu item */
//   itemKey?: string;
//   /** Content to display in the actions menu item */
//   title?: React.ReactNode;
//   /** Render item as aria-disabled option */
//   isAriaDisabled?: boolean;
//   /** Props for adding a tooltip to a menu item. This is used to display tooltip when hovered over the item  */
//   tooltipProps?: TooltipProps;
//   /** Click handler for the actions menu item */
//   onClick?: (event: React.MouseEvent, rowIndex: number, rowData: IRowData, extraData: IExtraData) => void;
//   /** Flag indicating this action should be placed outside the actions menu, beside the toggle */
//   isOutsideDropdown?: boolean;
//   /** Flag indicating whether the actions dropdown should close after an item is clicked. */
//   shouldCloseOnClick?: boolean;
// }

// export interface ISeparator extends IAction {
//   isSeparator: boolean;
// }

// export type IActions = (IAction | ISeparator)[];
// export type IActionsResolver = (rowData: IRowData, extraData: IExtraData) => (IAction | ISeparator)[];
// export type IAreActionsDisabled = (rowData: IRowData, extraData: IExtraData) => boolean;

// // to be removed in future, this interface is no longer accurate
// export interface IDecorator extends React.HTMLProps<HTMLElement> {
//   isVisible: boolean;
//   children?: React.ReactNode;
// }

#[derive(Default)]
pub struct DecoratorReturnType
{
    pub classes: Option<Classes>, // className?: string;
    pub aria_sort: Option<String>,
    pub children: Option<Html>, //Children?
    pub text_center: bool, // optional?
    // component?: string;
    // isVisible?: boolean;
    // title?: React.ReactNode;
    // props?: any;
    // scope?: string;
    // parentId?: number;
    // colSpan?: number;
    // id?: React.ReactText;
}

// export type ITransform = (label?: IFormatterValueType, extra?: IExtra) =>    ;

// export type IFormatter = (data?: IFormatterValueType, extra?: IExtra) => formatterValueType & decoratorReturnType;

// export interface ICell {
//   /* cell contents */
//   title?: React.ReactNode;
//   /** transformations applied to the header cell */
//   transforms?: ITransform[];
//   /** transformations applied to the cells within the column's body */
//   cellTransforms?: ITransform[];
//   /** transformations applied to the whole column */
//   columnTransforms?: ITransform[];
//   /** formatters applied to the header cell */
//   formatters?: IFormatter[];
//   /** formatters applied to the cells within the column's body */
//   cellFormatters?: IFormatter[];
//   /** Additional header props, it contains the info prop as well which can be used to add tooltip/popover */
//   header?: HeaderType;
//   /** Additional props passed into the rendered column header element */
//   props?: any;
//   data?: any;
//   cell?: any;
//   /** Text to display when data from this column is rendered in mobile view */
//   dataLabel?: string;
// }

// export type RowCellContent<T = any> = (value?: string, rowIndex?: number, cellIndex?: number, props?: T) => void;

// export interface IRowCell<T = any> {
//   title?: React.ReactNode | RowCellContent<T>;
//   props?: T;
//   formatters?: IFormatter[];
// }

// export interface IValidatorDef {
//   validator: (value: string) => boolean;
//   errorText: string;
//   name: string;
// }

// export interface IRow extends RowType {
//   cells?: (React.ReactNode | IRowCell)[];
//   isOpen?: boolean;
//   isEditable?: boolean;
//   isClickable?: boolean;
//   isRowSelected?: boolean;
//   isValid?: boolean;
//   /** An array of validation functions to run against every cell for a given row */
//   rowEditValidationRules?: IValidatorDef[];
//   /** Aria label for edit button in inline edit */
//   rowEditBtnAriaLabel?: (idx: number) => string;
//   /** Aria label for save button in inline edit */
//   rowSaveBtnAriaLabel?: (idx: number) => string;
//   /** Aria label for cancel button in inline edit */
//   rowCancelBtnAriaLabel?: (idx: number) => string;
//   parent?: number;
//   compoundParent?: number;
//   fullWidth?: boolean;
//   noPadding?: boolean;
//   heightAuto?: boolean;
//   showSelect?: boolean;
//   isExpanded?: boolean;
//   isFirstVisible?: boolean;
//   isLastVisible?: boolean;
//   /** Whether the row checkbox/radio button is selected */
//   selected?: boolean;
//   /** @deprecated Use disableSelection instead - Whether the row checkbox is disabled */
//   disableCheckbox?: boolean;
//   /** Whether the row checkbox/radio button is disabled */
//   disableSelection?: boolean;
//   /** Whether the row is favorited */
//   favorited?: boolean;
//   /** Any additional props forwarded to the favorites cell */
//   favoritesProps?: any;
//   /** any additional row props */
//   props?: any;
// }