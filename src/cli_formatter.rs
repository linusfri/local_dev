
use std::fmt;
use std::io::{self, Write};
use dialoguer::Select;

#[derive(Debug)]
struct IndexError {
    details: String
}

impl IndexError {
    fn new(details: String) -> Self {
        Self { details }
    }
}

impl fmt::Display for IndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl std::error::Error for IndexError {}

/// Renders a selection list which contains every item in items.
/// By first item in the list is selected by default.
pub fn render_selection_list<'a, T>(items: &'a Vec<T>, prompt: &str) -> Result<&'a T, Box<dyn std::error::Error>>
where T: std::fmt::Display + Clone
{
    let selected_index = Select::new()
        .with_prompt(prompt)
        .items(&items)
        .default(0)
        .interact()?;

    fetch_list_item(&items, selected_index)
}

/// Returns a reference to item at specified index
fn fetch_list_item<'a, T>(items: &'a Vec<T>, selected_index: usize) -> Result<&'a T, Box<dyn std::error::Error>> {
    match items.get(selected_index) {
        Some(value) => Ok(value),
        None => Err(Box::new(IndexError::new(String::from("Index out of bounds"))))
    }
}

pub fn prompt_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?; // Ensure the prompt is displayed immediately
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim_end().to_string())
}