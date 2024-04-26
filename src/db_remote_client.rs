use std::error::Error;

use docker_api::{models::ContainerSummary, opts::ContainerListOpts};

use crate::cli_formatter;

pub async fn list_remote_docker_instances() -> Result<(), Box<dyn Error>>{
    let docker = docker_api::Docker::new("unix:///var/run/docker.sock")?;

    let show_container_opts = ContainerListOpts::builder().all(true).build();
    let res = docker.containers().list(&show_container_opts).await?;

    let running_containers = get_running_db_containers(res);

    let mut db_container_names: Vec<String> = vec![];
    running_containers
        .iter()
        .for_each(|container| {
            if let Some(names) = container.names.as_ref() {
                filter_out_db_containers(names, &mut db_container_names)
            }
        });
    
    match db_container_names.len() {
        length if length > 0 => {
            let chosen_container_name = cli_formatter::render_selection_list(&db_container_names, "Choose container");
            get_db_from_container(chosen_container_name);
        },
        _ => { println!("No containers found"); }
    }

    Ok(())
}

fn filter_out_db_containers(names: &Vec<String>, array_buffer: &mut Vec<String>) {
    names
        .iter()
        .for_each(|name| {
            if name.contains("db") {
                if let Some(db_container_name) = name.strip_prefix("/") {
                    array_buffer.push(db_container_name.to_string());
                }
            }
        })
}

fn get_db_from_container(container_name: &String) {
    println!("Got db from {}", container_name);
}

fn get_running_db_containers(docker_container_response: Vec<ContainerSummary>) -> Vec<ContainerSummary> {
    docker_container_response
        .into_iter()
        .filter(|container| {
            if let Some(state) = container.state.as_ref() {
                return *state != "Exited".to_string()
            }
    
            false
        })
        .collect()
}

pub fn list_remote_instances() {

}