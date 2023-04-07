use std::path::Path;
use crate::command::Command;
use self::env::ConfigEnv;
pub mod env;
pub mod toml;

pub struct Config<'a> {
  env: ConfigEnv,
  command: Command<'a>,
  arguments: Vec<String>
}

impl<'a> Clone for Config<'a> {
  fn clone(&self) -> Config<'a> {
    Self {
      env: self.env.clone(),
      command: self.command.clone(),
      arguments: self.arguments.clone()
    }
  }
}

impl<'a> Config<'a> {
  pub fn new(
    command: Command<'a>,
    arguments: Vec<String>
  ) -> Self {
    Self {
      env: ConfigEnv::generate(),
      command,
      arguments
    }
  }

  pub fn get_env(&self) -> &ConfigEnv {
    &self.env
  }

  pub fn get_arguments(&self) -> &Vec<String> {
    &self.arguments
  }

  pub fn get_command(&self) -> &Command {
    &self.command
  }

  pub fn validate(&self) -> bool {
    let docker_folder = Self::validate_docker_folder(self);
    let docker_compose_file = Self::validate_docker_compose_file(self);
    let docker_env = Self::validate_docker_env(self);
    let docker_envs = Self::validate_docker_envs(self);

    if !docker_folder {
      println!("⚠️ - Docker folder does not exist, please specify a valid path or check file permissions");
      println!("→ - Docker commands lookups in .docker folder by default, you can override [docker]folder path in the dcmd.toml file at root of your execution path");
    }

    if !docker_compose_file {
      println!("⚠️ - Docker compose file does not exist, please specify a valid path or check file permissions");
      println!("→ - Docker commands lookups in .docker folder for a docker-compose.yml file by default, you can override [docker]compose_file path in the dcmd.toml file at root of your execution path");
    }

    if !docker_env {
      println!("⚠️ - Docker environment file does not exist, please specify a valid path or check file permissions");
      println!("→ - Docker commands lookups in .docker folder for a .env fileby default, you can override [docker]compose_env_file path in the dcmd.toml file at root of your execution path");
    }

    if !docker_envs {
      println!("⚠️ - Docker environment file does not contain every required variables");
      println!("→ - SUBNET_BASE should be defined, 192.168.3 for example");
      println!("→ - PROJECT_NAME should be defined, 192.168.3 for my_awesome_project for example");
      println!("→ - PROJECT_ROOT should be defined, /Users/awesome_user/projects/my_awesome_project");
    }

    docker_compose_file &&
    docker_folder &&
    docker_env
  }

  fn validate_docker_folder(&self) -> bool {
    Path::new(self.get_env().get_docker_folder()).is_dir()
  }

  fn validate_docker_compose_file(&self) -> bool {
    Path::new(self.get_env().get_docker_compose_file()).is_file()
  }

  fn validate_docker_env(&self) -> bool {
    Path::new(self.get_env().get_docker_folder()).join(".env").is_file()
  }

  fn validate_docker_envs(&self) -> bool {
    let vars = self.get_env().get_docker_env_vars();
    ["SUBNET_BASE", "PROJECT_NAME", "PROJECT_ROOT"]
    .iter()
    .all(|&var| vars.get(var).is_some())
  }

}