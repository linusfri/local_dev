use std::error::Error;

pub fn list_remote_docker_instances() -> Result<(), Box<dyn Error>>{
    // WILL SPLIT BY WHITESPACES... Some columns in docker ps will fuck this up.
    let container_ids_output = std::process::Command::new("bash")
        .arg("-c")
        .arg("ssh do docker ps | awk '{print $1}'")
        .output()?;

    let container_ids_str = String::from_utf8(container_ids_output.stdout)?;
    
    println!("{}", container_ids_str);

    Ok(())
}

pub fn list_remote_instances() {

}