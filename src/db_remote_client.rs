use std::{collections::HashMap, error::Error, process::{ExitStatus, Output}};
use cli_formatter::TerminalOutput;
use bollard::{container::ListContainersOptions, Docker};

use crate::cli_formatter;

pub async fn list_local_docker_instances() -> Result<(), Box<dyn Error>> {
    let docker = Docker::connect_with_local_defaults()?;

    let mut filters = HashMap::new();
    filters.insert("name".to_string(), vec![
        "db".to_string()
    ]);

    let list_container_opts = Some(ListContainersOptions::<String> {
        all: true,
        filters,
        ..Default::default()
    });

    let container_summary = &docker.list_containers(list_container_opts).await?;

    let container_names: Vec<String> = container_summary
        .iter()
        .flat_map(|container| container.names.iter())
        .flatten()
        .map(|name| name.trim().to_string())
        .collect();

    let container_choice = cli_formatter::render_selection_list(&container_names, "Choose container (currently only mariadb and mysql are supported)");
    match container_choice {
        Ok(container_choice) => { get_db_from_container(container_choice)?; },
        Err(err) => println!("{err}, you likely have no db containers running.")
    }

    Ok(())
}

fn get_db_from_container(container_name: &str) -> Result<(), Box<dyn Error>> {
    let mut db_user_password_buffer = String::new();
    let mut db_user_buffer = String::new();
    let mut output = Output {status: ExitStatus::default(), stdout: vec![], stderr: vec![]};

    loop {
        db_user_buffer = cli_formatter::prompt_input("User: ")?;
        db_user_password_buffer = cli_formatter::prompt_input("Password: ")?;

        output = std::process::Command::new("bash")
            .arg("-c")
            .arg(format!("docker exec {container_name} mysql -u {db_user_buffer} -p{db_user_password_buffer} -e \"SHOW DATABASES;\""))
            .output()?;

        if output.status.success() {
            break
        } else {
            println!("{}", TerminalOutput::try_from(output)?)
        }
    }

    let raw_db_string = String::from_utf8(output.stdout)?.trim().to_string();
    let container_databases: Vec<String> = raw_db_string
        .lines()
        .skip(1) // Skip the header
        .map(|db_name| {
            db_name.trim().to_string()
        })
        .collect();

    let db_choice = cli_formatter::render_selection_list(&container_databases, "Choose db to export");
    match db_choice {
        Ok(db_choice) => {
            let save_path = cli_formatter::prompt_input(
                "Where do you want to save the database file? Enter absolute path WITHOUT trailing slash: "
            )?;
            export_db(
                container_name,
                &db_user_buffer,
                &db_user_password_buffer,
                db_choice,
                &save_path
            )?
        },
        _ => println!("No db selected")
    }

    Ok(())
}

fn export_db(container_name: &str, user: &str, password: &str, db_name: &str, save_path: &str) -> Result<(), Box<dyn Error>> {
    let db_export_success = std::process::Command::new("bash")
        .arg("-c")
        .arg(format!("docker exec {container_name} mysqldump -u {user} -p{password} {db_name} > {save_path}/{db_name}.sql"))
        .output();

    match db_export_success {
        Ok(output) => {
            if output.status.success() {
                println!("Database exported to: {save_path}/{db_name}.sql");
            } else {
                println!("docker exec didn't exit successfully, error status: {}", output.status);
            }
        },
        Err(export_error) => {
            println!("Something went wrong, error status: {export_error}");
        }
    }
    Ok(())

}