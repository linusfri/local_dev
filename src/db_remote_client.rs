use std::error::Error;

use docker_api::opts::ContainerListOpts;

pub async fn list_remote_docker_instances() -> Result<(), Box<dyn Error>>{
    let docker = docker_api::Docker::new("unix:///var/run/docker.sock")?;

    let res = docker.containers().list(&ContainerListOpts::builder().all(true).build()).await?;

    println!("{:#?}", res);
    Ok(())
}

pub fn list_remote_instances() {

}