use std::fmt::Display;
use std::env;
use std::fs;

use dialoguer::Select;

#[derive(Clone)]
enum ProjectType {
    Rust,
    PHP
}

impl Display for ProjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectType::Rust => write!(f, "Rust"),
            ProjectType::PHP => write!(f, "PHP"),
        }
    }
}

#[derive(Clone)]
enum Action {
    Create,
    Delete
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Create => write!(f, "Create"),
            Action::Delete => write!(f, "Delete")
        }
    }
}

fn main() -> () {
    let action = select_action();

    match action {
        Action::Create => create_project(),
        Action::Delete => delete_project()
    };
}

fn create_project() {
    let project_type = select_project_type();

    let file_contents = match project_type {
        ProjectType::PHP => include_str!("../docker/php.yml"),
        ProjectType::Rust => include_str!("../docker/rust.yml"),
    };

    println!("{}", file_contents);
    let current_dir = env::current_dir().unwrap(); 
}

fn delete_project() {
    println!("You deleted project");
}
    
fn select_action() -> Action {
    let actions = vec![
        Action::Create,
        Action::Delete
    ];

    let action = render_selection_list(&actions, "Select action");

    action.clone()
}

fn select_project_type() -> ProjectType {
    let items = vec![
        ProjectType::Rust,
        ProjectType::PHP
    ];

    let selected_project_type = render_selection_list(&items, "Select project type");

    selected_project_type.clone()
}

fn render_selection_list<'a, T>(items: &'a Vec<T>, prompt: &str) -> &'a T
where T: Display + Clone
{
    let selected_index = Select::new()
        .with_prompt(prompt)
        .items(&items)
        .default(0)
        .interact()
        .expect("Something went wrong");

    fetch_list_item(&items, selected_index)
}

fn fetch_list_item<'a, T>(items: &'a Vec<T>, selected_index: usize) -> &'a T {
    &items[selected_index]
}