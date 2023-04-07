use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConfigToml {
  pub docker: ConfigDocker
}

#[derive(Deserialize)]
pub struct ConfigDocker {
  pub folder: String,
  pub compose_file: String,
  pub compose_env_file: String,
  pub stop_timeout: u32,
}