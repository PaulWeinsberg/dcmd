use std::{io::{Error}, process::{self, Command}, fs::{self, DirEntry}, path::Path};

use crate::config::Config;

pub fn handle_custom(config: &Config) {
  let mut command = Command::new("bash");
  command.envs(config.get_env().get_docker_env_vars().into_iter());

  let custom_command_path = Path::new(config.get_env().get_docker_folder())
  .join("commands")
  .join(config.get_command().to_string());

  let is_nested: bool = match config.get_arguments().get(0) {
    Some(arg) => {
      Path::new(&custom_command_path)
      .join(arg)
      .is_file()
    },
    _ => false
  };

  if is_nested {
    let custom_command_path = custom_command_path
    .join(config.get_arguments().get(0).unwrap());
    let args = &mut config.get_arguments().clone()[1..];
    command
    .arg(custom_command_path)
    .args(args);
  } else {
    command
    .arg(custom_command_path)
    .args(config.get_arguments());
  }

  exec_command(command);

}

pub fn handle_list(config: &Config) {
  let commands_path = Path::new(config.get_env().get_docker_folder()).join("commands");

  let commands_dir = fs::read_dir(&commands_path)
  .expect("Impossible to access commands directory");

  println!("Available builtin commands: \n");

  super::Command::iter_builtin()
  .for_each(|command| println!("dcmd {}", command));

  println!("\nAvailable custom commands: \n");

  commands_dir
  .flat_map(|entry| -> Vec<String> {
    let entry = entry.unwrap();
    if entry.file_type().unwrap().is_dir() {
      let app = entry.file_name().into_string().unwrap();
      fs::read_dir(commands_path.join(&app))
      .expect("Impossible to access sub commands directory")
      .flat_map(|entry| Ok::<DirEntry, Error>(entry.unwrap()))
      .map(|entry| {
        let mut command = app.clone();
        command.push(' ');
        command.push_str(entry.file_name().into_string().unwrap().as_str());
        command
      })
      .collect::<Vec<String>>()

    } else {
      Vec::from([entry.file_name().into_string().unwrap()])
    }
  })
  .for_each(|result| println!("dcmd {}", result));
}

pub fn handle_help() {
  println!("Run dcmd ls to see all available commands");
}

pub fn handle_version(config: &Config) {
  println!("Docker Commands version: {}", config.get_env().get_version());
  println!("Rust build");
}

pub fn handle_up(config: &Config) {
  let mut command = Command::new("docker");
  initialize_docker_command(&mut command, config);

  command
  .arg("--build")
  .arg("-d");

  finalize_docker_command(&mut command, config);
  exec_command(command);
}

pub fn handle_down(config: &Config) {
  let mut command = Command::new("docker");
  initialize_docker_command(&mut command, config);

  command
  .arg("-v")
  .arg("-t")
  .arg(config.get_env().get_stop_timeout().to_string());

  finalize_docker_command(&mut command, config);
  exec_command(command);
}

pub fn handle_stop(config: &Config) {
  let mut command = Command::new("docker");
  initialize_docker_command(&mut command, config);

  command
  .arg("-t")
  .arg(config.get_env().get_stop_timeout().to_string());

  finalize_docker_command(&mut command, config);
  exec_command(command);
}

pub fn handle_start(config: &Config) {
  let mut command = Command::new("docker");
  initialize_docker_command(&mut command, config);
  finalize_docker_command(&mut command, config);
  exec_command(command);
}

pub fn handle_restart(config: &Config) {
  let mut command = Command::new("docker");
  initialize_docker_command(&mut command, config);
  command
  .arg("-t")
  .arg(config.get_env().get_stop_timeout().to_string());

  finalize_docker_command(&mut command, config);
  exec_command(command);
}

fn initialize_docker_command<'a>(command: &'a mut Command, config: &'a Config) -> &'a Command {

  command
  .arg("compose")
  .arg("-f")
  .arg(config.get_env().get_docker_compose_file())
  .arg(config.get_command().to_string())
  .envs(config.get_env().get_docker_env_vars().into_iter());

  command
}

fn finalize_docker_command<'a>(command: &'a mut Command, config: &Config) -> &'a Command {
  config.get_arguments()
  .iter()
  .for_each(|arg| {
    command.arg(arg);
  });

  command
}

fn exec_command(mut command: Command) {
  let output = command
  .status()
  .unwrap_or_else(|_| {
    println!("Something wrong happend while trying to run command {:?}", command.get_args());
    process::exit(1);
  });

  assert!(output.success());
}