// import { getBreakpoint, getVerticalBreakpoint } from '../../helpers/util';

use yew::Callback;

#[derive(Clone, PartialEq)]
pub struct PageContext
{
  pub is_managed_sidebar: bool,
  pub on_sidebar_toggle: Option<Callback<()>>,
  pub is_sidebar_open: bool,
  pub width: i32,
  pub height: i32,
//   getBreakpoint: (width: number | null) => 'default' | 'sm' | 'md' | 'lg' | 'xl' | '2xl';
//   getVerticalBreakpoint: (height: number | null) => 'default' | 'sm' | 'md' | 'lg' | 'xl' | '2xl';
}
