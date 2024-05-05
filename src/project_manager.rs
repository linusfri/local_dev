use crate::{cli_formatter, fs_handler};
use std::{error::Error, fmt::Display};

#[derive(Clone)]
enum Answer {
    Yes,
    No
}

#[derive(Clone)]
pub enum ProjectType {
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


impl Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Answer::Yes => write!(f, "Yes"),
            Answer::No => write!(f, "No"),
        }
    }
}

pub fn create_project() -> Result<(), Box<dyn Error>> {
    let options = vec![Answer::Yes, Answer::No];

    let do_clone_repo = cli_formatter::render_selection_list(&options, "Do you want to clone a repo?")?;
    let do_create_docker_base = cli_formatter::render_selection_list(&options, "Do you want to create a base for running docker?")?;
    
    match (do_clone_repo, do_create_docker_base) {
        (Answer::Yes, Answer::Yes) => {
            fs_handler::clone_repo()?;
            fs_handler::create_docker_base()?;
        },
        (Answer::Yes, Answer::No) => {
            fs_handler::clone_repo()?;
        },
        (Answer::No, Answer::Yes) => {
            fs_handler::create_docker_base()?;
        },
        (Answer::No, Answer::No) => {
            println!("No action taken.");
        }
    }

    Ok(())
}

pub fn delete_project() -> Result<(), Box<dyn Error>> {
    println!("You deleted project");

    Ok(())
}

pub fn select_project_type() -> Result<ProjectType, Box<dyn Error>> {
    let items = vec![
        ProjectType::Rust,
        ProjectType::PHP
    ];

    let selected_project_type = cli_formatter::render_selection_list(&items, "Select project type")?;

    Ok(selected_project_type.clone())
}