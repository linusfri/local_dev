use std::{error::Error, process::{ExitStatus, Output}};

use docker_api::opts::{ContainerFilter, ContainerListOpts, ContainerStatus, ImageName};

use crate::cli_formatter;

pub async fn list_remote_docker_instances() -> Result<(), Box<dyn Error>> {
    let docker = docker_api::Docker::new("unix:///var/run/docker.sock")?;

    let filters = vec![
        ContainerFilter::Ancestor(ImageName::Tag{ image: "mysql".to_string(), tag: Some("5.7-debian".to_string()) }),
        ContainerFilter::Ancestor(ImageName::Tag{ image: "mysql".to_string(), tag: None }),
        ContainerFilter::Status(ContainerStatus::Running)
    ];
    let show_container_opts = ContainerListOpts::builder().filter(filters).build();
    let container_summary = docker.containers().list(&show_container_opts).await?;

    let container_names: Vec<String> = container_summary
        .iter()
        .flat_map(|container| container.names.iter())
        .flatten()
        .cloned()
        .collect();

    let container_choice = cli_formatter::render_selection_list(&container_names, "Choose container");
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
        output = std::process::Command::new("bash")
            .arg("-c")
            .arg(format!("docker exec {container_name} mysql -u {db_user_buffer} -p{db_user_password_buffer} -e \"SHOW DATABASES;\""))
            .output()?;
        
        if output.status.success()  {
            break;
        }

        db_user_buffer = cli_formatter::prompt_input("User: ")?;
        db_user_password_buffer = cli_formatter::prompt_input("Password: ")?;
    }

    let raw_db_string = String::from_utf8(output.stdout)?.trim().to_string();
    let container_databases: Vec<String> = raw_db_string
        .split("\n")
        .filter(|db_name| {
            match *db_name {
                "Database" => false,
                _ => true
            }
        })
        .map(|db_name| {
            db_name.to_string()
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
        _ => { println!("No db selected"); }
    }

    Ok(())
}

fn export_db(container_name: &str, user: &str, password: &str, db_name: &str, save_path: &str) -> Result<(), Box<dyn Error>> {
    std::process::Command::new("bash")
        .arg("-c")
        .arg(format!("docker exec {container_name} mysqldump -u {user} -p{password} {db_name} > {save_path}/{db_name}.sql"))
        .output()?;

    Ok(())

}
pub fn list_remote_instances() {

}