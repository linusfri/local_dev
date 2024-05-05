use std::{error::Error, io::Read};

use docker_api::{models::ContainerSummary, opts::{ContainerFilter, ContainerListOpts, ContainerStatus, ImageName}};

pub async fn list_remote_docker_instances() -> Result<(), Box<dyn Error>> {
    let docker = docker_api::Docker::new("unix:///var/run/docker.sock")?;

    let filters = vec![
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

    println!("{:#?}", container_names);

    Ok(())
}

pub fn list_remote_instances() {

}