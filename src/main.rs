use core::panic;
use std::fmt::Display;
use std::env;
use std::fs;
use std::io::Write;
use tokio::runtime::Runtime;
use dialoguer::Select;
use lazy_static::lazy_static;

use local_dev::git_host_client::GitHostClient;

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

enum FileType {
    ComposeFile(String),
    DockerFile(String),
    NginxConfig(String)
}

lazy_static! {
    static ref APP_PATH: String = {
        get_config_location()
    };
}

fn main() {
    Runtime::new().unwrap().block_on(get_projects_from_git_host());
    let action = select_action();

    match action {
        Action::Create => create_project(),
        Action::Delete => delete_project()
    };
}

fn get_config_location() -> String {
    let config_location = match env::var("DEV_FILE_PATH") {
        Ok(path) => Some(path.to_string()),
        Err(_) => None,
    };
    
    if let Some(config_location) = config_location {
        config_location
    } else {
        println!("No DEV_FILE_PATH found, checking os variables.");
        match env::var_os("LOCAL_DEV_PATH") {
            Some(os_path) => os_path.to_str().unwrap().to_string(),
            None => { panic!("No PATH to local dev config files. Do you have LOCAL_DEV_PATH in your .zshrc / .bashrc?") }
        }
    }
}

fn select_action() -> Action {
    let actions = vec![
        Action::Create,
        Action::Delete
    ];

    let action = render_selection_list(&actions, "Select action");

    action.clone()
}

fn create_project() {
    let project_type = select_project_type();

    match project_type {
        ProjectType::PHP => {
            
        },
        ProjectType::Rust => {
            let required_files = vec![
                FileType::DockerFile(get_real_path("config/docker/rust/Dockerfile".to_string())),
                FileType::ComposeFile(get_real_path("config/docker/rust/docker-compose.yml".to_string())),
                FileType::NginxConfig(get_real_path("config/nginx/rust/default.conf".to_string()))
            ];
            
            create_project_files(required_files);
        },
    };
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

/// Gets the real path based on environment variable LOCAL_DEV_PATH or DEV_FILE_PATH,
/// depending on which variable is currently set. I both are set, DEV_FILE_PATH will be returned.
fn get_real_path(file_path: String) -> String {
    let mut real_path = APP_PATH.to_string();

    real_path.push_str(file_path.as_str());

    real_path
}

fn create_project_files(files: Vec<FileType>) {
    files
        .iter()
        .for_each(|f| {
            match f {
                FileType::ComposeFile(file_to_read_from) => {
                    create_file(file_to_read_from, "docker-compose.yml")
                },
                FileType::DockerFile(file_to_read_from) => {
                    create_file(file_to_read_from, "Dockerfile")
                },
                FileType::NginxConfig(file_to_read_from) => {
                    create_file(file_to_read_from, "default.conf")
                },
            };
        });
}

/// Creates a file in the same directory that the CLI is invoked from
fn create_file(file_to_read_from: &str, filename_to_be_created: &str) {
    let file_contents = fs::read_to_string(file_to_read_from).unwrap();
    let mut file_to_create_path = env::current_dir().unwrap();

    file_to_create_path.push(filename_to_be_created);
    let mut created_file = fs::File::create(file_to_create_path).unwrap();

    created_file.write(file_contents.as_bytes()).unwrap();
}

fn delete_project() {
    println!("You deleted project");
}

async fn get_projects_from_git_host() -> String {
    let client = GitHostClient::new(
        String::from("https://google.com"),
        String::from("token")
    );

    let string = client.request("GET", None).await.unwrap();

    println!("{}", string);
    string
}