use std::{env};

use crate::{config::Config, command::handle};

mod command;
mod config;

fn main() {
  let args: Vec<String> = env::args().collect();
  let default_command = String::from("help");
  let base_command = command::get_command(&args, &default_command);
  let arguments = command::get_arguments(&args);
  let config = Config::new(base_command, arguments);

  handle(&config);
}