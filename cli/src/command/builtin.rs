use std::{io::{Error}, process::{Command}, fs::{self, DirEntry}, path::Path, env};

use curl::easy::Easy;

use crate::config::Config;

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

pub fn handle_update(config: &Config) {
  println!("Updating CLI for platform: {}", config.get_env().get_platform());

  let mut url = String::from(env!("GITHUB_URL"));
  url.push_str("/main/cli/bin/");
  url.push_str(config.get_env().get_platform());
  url.push_str("/dcmd");

  let mut easy = Easy::new();
  easy.url(url.as_str()).unwrap();

  easy
  .transfer()
  .write_function(|data| {
    fs::write(env::current_exe().unwrap(), data)
    .expect("Can't write file to the destination path.");
    Ok(data.len())
  })
  .expect("Error while writing the file");

  easy
  .perform()
  .expect("Error while fetching data from the remote server");

  println!("CLI successfully updated ✅");
}

pub fn handle_version(config: &Config) {
  println!("Docker Commands version: {}", config.get_env().get_version());
  println!("Platform: {}", config.get_env().get_platform());
}

pub fn handle_up(config: &Config) {
  let mut command = Command::new("docker");
  initialize_docker_command(&mut command, config);

  command
  .arg("--build")
  .arg("-d");

  finalize_docker_command(&mut command, config);
  super::exec_command(command);
}

pub fn handle_down(config: &Config) {
  let mut command = Command::new("docker");
  initialize_docker_command(&mut command, config);

  command
  .arg("-v")
  .arg("-t")
  .arg(config.get_env().get_stop_timeout().to_string());

  finalize_docker_command(&mut command, config);
  super::exec_command(command);
}

pub fn handle_stop(config: &Config) {
  let mut command = Command::new("docker");
  initialize_docker_command(&mut command, config);

  command
  .arg("-t")
  .arg(config.get_env().get_stop_timeout().to_string());

  finalize_docker_command(&mut command, config);
  super::exec_command(command);
}

pub fn handle_start(config: &Config) {
  let mut command = Command::new("docker");
  initialize_docker_command(&mut command, config);
  finalize_docker_command(&mut command, config);
  super::exec_command(command);
}

pub fn handle_restart(config: &Config) {
  let mut command = Command::new("docker");
  initialize_docker_command(&mut command, config);
  command
  .arg("-t")
  .arg(config.get_env().get_stop_timeout().to_string());

  finalize_docker_command(&mut command, config);
  super::exec_command(command);
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
