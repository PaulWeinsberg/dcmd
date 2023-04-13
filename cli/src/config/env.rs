use std::{fs, env};

use indexmap::IndexMap;

use super::toml::ConfigToml;
pub struct ConfigEnv {
  docker_folder: String,
  docker_compose_file: String,
  docker_env_vars: IndexMap<String, String>,
  stop_timeout: u32,
  version: &'static str,
  platform: &'static str,
}

impl Clone for ConfigEnv {
  fn clone(&self) -> Self {
    Self {
      docker_folder: self.docker_folder.clone(),
      docker_compose_file: self.docker_compose_file.clone(),
      docker_env_vars: self.docker_env_vars.clone(),
      stop_timeout: self.stop_timeout,
      version: self.version,
      platform: self.platform
    }
  }
}

impl ConfigEnv {
  pub fn generate() -> Self {
    let toml_config = Self::get_toml_config();

    let docker_env_vars = envmnt::parse_file(&toml_config.docker.compose_env_file)
    .unwrap_or(IndexMap::new());

    Self {
      docker_folder: toml_config.docker.folder,
      docker_compose_file: toml_config.docker.compose_file,
      docker_env_vars,
      stop_timeout: toml_config.docker.stop_timeout,
      version: env!("CARGO_PKG_VERSION"),
      platform: current_platform::CURRENT_PLATFORM,
    }
  }

  pub fn get_platform(&self) -> &str {
    self.platform
  }

  pub fn get_version(&self) -> &str {
    self.version
  }

  pub fn get_stop_timeout(&self) -> &u32 {
    &self.stop_timeout
  }

  pub fn get_docker_folder(&self) -> &str {
    self.docker_folder.as_str()
  }

  pub fn get_docker_compose_file(&self) -> &str {
    &self.docker_compose_file
  }

  pub fn get_docker_env_vars(&self) -> &IndexMap<String, String> {
    &self.docker_env_vars
  }

  pub fn get_toml_config() -> ConfigToml {
    let toml_config = fs::read_to_string("dcmd.toml").unwrap_or(String::from(r#"
    [docker]
    folder = '.docker'
    compose_file = '.docker/docker-compose.yml'
    compose_env_file = '.docker/.env'
    stop_timeout = 3"#));

    match toml::from_str::<ConfigToml>(toml_config.as_str()) {
      Ok(toml) => toml,
      Err(err) => {
        println!("dcmd.toml is not deserializable, error: {}", err.message());
        panic!()
      }
    }

  }

}
