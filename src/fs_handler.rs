use std::error::Error;
use std::env;
use std::fs;
use std::io::Write;
use reqwest::Method;
use tokio::runtime::Runtime;
use crate::cli_formatter;
use crate::git_host_client::{GitHostClient, GithubRepoResponse};
use crate::project_manager;
use lazy_static::lazy_static;

lazy_static! {
    static ref APP_PATH: String = {
        get_config_location()
    };
}

pub enum FileType {
    ComposeFile(String),
    DockerFile(String),
    NginxConfig(String)
}

/// Gets the real path based on environment variable LOCAL_DEV_PATH or DEV_FILE_PATH,
/// depending on which variable is currently set. I both are set, DEV_FILE_PATH will be returned.
pub fn get_real_path(file_path: String) -> String {
    let mut real_path = APP_PATH.to_string();

    real_path.push_str(file_path.as_str());

    real_path
}

/// Reads from every input file in files Vec and outputs
/// contents in output file corresponding to input files enum variant. 
pub fn create_project_files(files: Vec<FileType>) -> Result<(), Box<dyn Error>> {
    files
        .iter()
        .try_for_each(|f| {
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
            }
        })?;
    
    Ok(())
}

/// Creates a file in the same directory that the CLI is invoked from
pub fn create_file(file_to_read_from: &str, filename_to_be_created: &str) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(file_to_read_from)?;
    let mut file_to_create_path = env::current_dir()?;

    file_to_create_path.push(filename_to_be_created);
    let mut created_file = fs::File::create(file_to_create_path)?;

    created_file.write(file_contents.as_bytes())?;

    Ok(())
}

pub fn create_docker_base() -> Result<(), Box<dyn Error>> {
    let project_type = project_manager::select_project_type()?;

    match project_type {
        project_manager::ProjectType::PHP => {
            todo!();
        },
        project_manager::ProjectType::Rust => {
            let required_files = vec![
                FileType::DockerFile(get_real_path("config/docker/rust/Dockerfile".to_string())),
                FileType::ComposeFile(get_real_path("config/docker/rust/docker-compose.yml".to_string())),
                FileType::NginxConfig(get_real_path("config/nginx/rust/default.conf".to_string()))
            ];
            
            create_project_files(required_files)?;
        },
    };

    Ok(())
}

pub fn clone_repo() -> Result<(), Box<dyn Error>>{
    let repos = Runtime::new()?
        .block_on(GitHostClient::new().request::<Vec<GithubRepoResponse>>(Method::GET, "/user/repos", None))?
        .into_iter()
        .map(|repo| {
            repo.ssh_url
        })
        .collect();

    let selected_repo = cli_formatter::render_selection_list(&repos, "Repos")?;
    
    std::process::Command::new("bash")
        .arg("-c")
        .arg(format!("{}{}", "git clone ", selected_repo))
        .output()?;

    Ok(())
}

fn get_config_location() -> String {
    let config_location = match env::var("DEV_FILE_PATH") {
        Ok(path) => Some(path.to_string()),
        Err(_) => None,
    };
    
    if let Some(config_location) = config_location {
        config_location
    } else {
        match env::var_os("LOCAL_DEV_PATH") {
            Some(os_path) => os_path.to_str().unwrap().to_string(),
            None => { panic!("No PATH to local dev config files. Do you have LOCAL_DEV_PATH in your .zshrc / .bashrc?") }
        }
    }
}