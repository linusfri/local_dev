use local_dev::cli_formatter;
use local_dev::db_remote_client;
use local_dev::project_manager;
use std::error::Error;
use std::fmt::Display;
use dotenv::dotenv;
use tokio::runtime::Runtime;

#[derive(Clone)]
enum Action {
    Create,
    Delete,
    GetDatabaseFromContainer
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Create => write!(f, "Create"),
            Action::Delete => write!(f, "Delete"),
            Action::GetDatabaseFromContainer => write!(f, "Get database from container"),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    // Load .env file
    dotenv().ok();

    let action = select_action()?;

    match action {
        Action::Create => project_manager::create_project()?,
        Action::Delete => project_manager::delete_project()?,
        Action::GetDatabaseFromContainer => Runtime::new().unwrap().block_on(db_remote_client::list_remote_docker_instances())?
    };

    Ok(())
}

fn select_action() -> Result<Action, Box<dyn Error>> {
    let actions = vec![
        Action::Create,
        Action::Delete,
        Action::GetDatabaseFromContainer
    ];

    let action = cli_formatter::render_selection_list(&actions, "Select action")?;

    Ok(action.clone())
}