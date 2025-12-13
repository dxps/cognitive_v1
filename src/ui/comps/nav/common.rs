/// Navigation header specific function to highlight the link based on the current path.
pub fn style_nav_item_link(curr_path: &String, link_path: String) -> &'static str {
    //
    if *curr_path == link_path {
        "text-sm text-green-600 py-2 px-4 hover:bg-gray-100 rounded-lg transition duration-200"
    } else {
        "text-sm text-gray-600 py-2 px-4 hover:bg-gray-100 rounded-lg transition duration-200"
    }
}
