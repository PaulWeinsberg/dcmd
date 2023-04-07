use std::{process, env};

use indexmap::IndexMap;
pub struct ConfigEnv {
  docker_folder: String,
  docker_compose_file: String,
  stop_timeout: u8,
  docker_env: IndexMap<String, String>,
  version: String,
}

impl Clone for ConfigEnv {
  fn clone(&self) -> Self {
    Self {
      docker_folder: self.docker_folder.clone(),
      docker_compose_file: self.docker_compose_file.clone(),
      stop_timeout: self.stop_timeout,
      docker_env: self.docker_env.clone(),
      version: self.version.clone(),
    }
  }
}

impl ConfigEnv {
  pub fn generate() -> Self {

    let docker_folder = env::var("DOCKER_FOLDER").unwrap_or(String::from(".docker"));
    let stop_timeout: u8 = env::var("DCMD_STOP_TIMEOUT").unwrap_or(String::from("3")).parse()
    .unwrap_or_else(|_| {
      println!("Cannot create a number for DCMD_STOP_TIMEOUT, please check your value and try again");
      process::exit(1);
    });

    let mut env_path = docker_folder.clone();
    env_path.push_str("/.env");

    let docker_env = envmnt::parse_file(&env_path)
    .unwrap_or_else(|_| {
      println!("{} does not exists or is corrupted", env_path);
      process::exit(1);
    });

    let mut docker_compose_file = docker_folder.clone();
    docker_compose_file.push_str("/docker-compose.yml");

    Self {
      docker_folder,
      docker_compose_file,
      stop_timeout,
      version: String::from("1.0.5"),
      docker_env
    }
  }

  pub fn get_version(&self) -> &str {
    self.version.as_str()
  }

  pub fn get_stop_timeout(&self) -> &u8 {
    &self.stop_timeout
  }

  pub fn get_docker_folder(&self) -> &str {
    self.docker_folder.as_str()
  }

  pub fn get_docker_compose_file(&self) -> &str {
    &self.docker_compose_file
  }

  pub fn get_docker_env_vars(&self) -> &IndexMap<String, String> {
    &self.docker_env
  }

}
