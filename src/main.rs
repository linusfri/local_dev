use std::fmt::Display;
use std::env;
use std::fs;
use std::io::Write;

use dialoguer::Select;
use include_dir::include_dir;

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

enum FileType<'a> {
    ComposeFile(&'a str),
    DockerFile(&'a str),
    NginxConfig(&'a str)
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

    match project_type {
        ProjectType::PHP => { include_str!("../config/docker/php.yml"); },
        ProjectType::Rust => {
            let required_files = vec![
                FileType::DockerFile("config/docker/Dockerfile-rust"),
                FileType::ComposeFile("config/docker/rust.yml"),
                FileType::NginxConfig("config/nginx/rust.conf")
            ];
            
            create_project_files(required_files);
        },
    };
}

fn create_project_files(files: Vec<FileType>) {
    files
        .iter()
        .for_each(|f| {
            match f {
                FileType::ComposeFile(path) => {
                    create_file(path, "docker-compose.yml")
                },
                FileType::DockerFile(path) => {
                    create_file(path, "Dockerfile")
                },
                FileType::NginxConfig(path) => {
                    create_file(path, "default.conf")
                },
            };
        });
}

fn create_file(path: &str, name: &str) {
    let file_contents = fs::read_to_string(path).unwrap();
    let mut file_path = env::current_dir().unwrap();

    file_path.push(name);
    let mut created_file = fs::File::create(file_path).unwrap();

    created_file.write(file_contents.as_bytes()).unwrap();
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