use crate::command::Command;
use self::env::ConfigEnv;
pub mod env;

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
}