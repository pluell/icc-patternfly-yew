
use rand::{distributions::Alphanumeric, Rng};
use web_sys::{Element};


pub fn utils_get_unique_id(prefix: Option<String>) -> String
{
    let pf = prefix.unwrap_or(String::from("pf"));

    // Generate random UID
    let uid: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(24)
        .map(char::from)
        .collect();

    format!("{}-{}", pf, uid)
}

/** This function returns whether or not an element is within the viewable area of a container. If partial is true,
 * then this function will return true even if only part of the element is in view.
 *
 * @param {Element} container  The container to check if the element is in view of.
 * @param {Element} element    The element to check if it is view
 * @param {boolean} partial   true if partial view is allowed
 *
 * @returns { boolean } True if the component is in View.
 */
pub fn is_element_in_view(container: &Element, element: &Element, partial: bool) -> bool
{
    // Get bounding rectangles
    let container_bounds = container.get_bounding_client_rect();
    let element_bounds = element.get_bounding_client_rect();

    // Get side bounds
    let container_bounds_left = container_bounds.left().floor();
    let container_bounds_right = container_bounds.right().floor();
    let element_bounds_left = element_bounds.left().floor();
    let element_bounds_right = element_bounds.right().floor();
  
    // Check if in view
    let is_totally_in_view = element_bounds_left >= container_bounds_left && element_bounds_right <= container_bounds_right;
    let is_partially_in_view =
      partial &&
      ((element_bounds_left < container_bounds_left && element_bounds_right > container_bounds_left) ||
        (element_bounds_right > container_bounds_right && element_bounds_left < container_bounds_right));

    // Return outcome
    is_totally_in_view || is_partially_in_view
  }