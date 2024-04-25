
use std::fmt::Display;
use dialoguer::Select;

/// Renders a selection list which contains every item in items.
/// By first item in the list is selected by default.
pub fn render_selection_list<'a, T>(items: &'a Vec<T>, prompt: &str) -> &'a T
where T: Display + Clone
{
    let selected_index = Select::new()
        .with_prompt(prompt)
        .items(&items)
        .default(0)
        .interact()
        .expect("Couldn't render list with selectable items.");

    fetch_list_item(&items, selected_index)
}

/// Returns a reference to item at specified index
fn fetch_list_item<'a, T>(items: &'a Vec<T>, selected_index: usize) -> &'a T {
    &items[selected_index]
}