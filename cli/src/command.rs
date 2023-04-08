use std::{slice::Iter, fmt::{Display, self}};
use crate::config::Config;

pub mod builtin;
pub mod custom;
pub mod dns;

#[derive(Debug)]
pub enum Command<'a> {
  Start(&'a str),
  Stop(&'a str),
  Restart(&'a str),
  Up(&'a str),
  Down(&'a str),
  Update(&'a str),
  Help(&'a str),
  Version(&'a str),
  List(&'a str),
  Custom(&'a str),
  Dns(&'a str),
}

impl<'a> Clone for Command<'a> {
  fn clone(&self) -> Self {
    match *self {
      Command::Start(value) => Command::Start(value),
      Command::Stop(value) => Command::Stop(value),
      Command::Restart(value) => Command::Restart(value),
      Command::Up(value) => Command::Up(value),
      Command::Down(value) => Command::Down(value),
      Command::Update(value) => Command::Update(value),
      Command::Help(value) => Command::Help(value),
      Command::Version(value) => Command::Version(value),
      Command::List(value) => Command::List(value),
      Command::Custom(value) => Command::Custom(value),
      Command::Dns(value) => Command::Custom(value),
    }
  }
}

impl<'a> Display for Command<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Command::Start(value) => write!(f, "{}", value),
      Command::Stop(value) => write!(f, "{}", value),
      Command::Restart(value) => write!(f, "{}", value),
      Command::Up(value) => write!(f, "{}", value),
      Command::Down(value) => write!(f, "{}", value),
      Command::Help(value) => write!(f, "{}", value),
      Command::Version(value) => write!(f, "{}", value),
      Command::List(value) => write!(f, "{}", value),
      Command::Update(value) => write!(f, "{}", value),
      Command::Custom(value) => write!(f, "{}", value),
      Command::Dns(value) => write!(f, "{}", value),
    }
  }
}

impl Command<'static> {
  pub fn iter_builtin() -> Iter<'static, Command<'static>> {
    static COMMANDS: [Command; 10] = [
      Command::Start("start"),
      Command::Stop("strop"),
      Command::Restart("restart"),
      Command::Up("up"),
      Command::Down("down"),
      Command::Help("Help"),
      Command::Version("version"),
      Command::List("list"),
      Command::Update("update"),
      Command::Dns("dns"),
    ];
    COMMANDS.iter()
  }
}

pub fn handle(config: &Config) {
  match config.get_command() {
    Command::Start(_) => builtin::handle_start(config),
    Command::Stop(_) => builtin::handle_stop(config),
    Command::Restart(_) => builtin::handle_restart(config),
    Command::Up(_) => builtin::handle_up(config),
    Command::Down(_) => builtin::handle_down(config),
    Command::Help(_) => builtin::handle_help(),
    Command::Version(_) => builtin::handle_version(config),
    Command::List(_) => builtin::handle_list(config),
    Command::Update(_) => println!("Feature will be implemented soon..."),
    Command::Custom(_) => custom::handle_custom(config),
    Command::Dns(_) => dns::handle_dns(config),
  }
}

pub fn get_arguments(args: &Vec<String>) -> Vec<String> {
  if args.len() >= 2 {
    return Vec::from(&args[2..])
  }
  Vec::new()
}

pub fn get_command<'a>(args: &'a [String], default: &'a String) -> Command<'a> {
  match args.get(1).unwrap_or(default).as_str() {
    "start" => Command::Start("start"),
    "stop" => Command::Stop("stop"),
    "restart" => Command::Restart("restart"),
    "up" => Command::Up("up"),
    "down" => Command::Down("down"),
    "update" => Command::Update("update"),
    "version" => Command::Version("version"),
    "-v" => Command::Version("version"),
    "--version" => Command::Version("version"),
    "help" => Command::Help("help"),
    "--help" => Command::Help("help"),
    "list" => Command::List("list"),
    "ls" => Command::List("list"),
    "--ls" => Command::List("list"),
    "dns" => Command::Dns("dns"),
    custom => Command::Custom(custom)
  }
}

fn exec_command(mut command: std::process::Command) {
  command
  .status()
  .unwrap_or_else(|_| {
    println!("Something wrong happend while trying to run command {:?}", command.get_args());
    std::process::exit(1);
  });
}