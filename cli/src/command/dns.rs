use std::process::Command;

use crate::config::Config;


pub fn handle_dns(config: &Config) {
  let action = config.get_arguments().get(0).expect("Use dns start or stop, no one specified");
  if action == "start" {
    handle_dns_start();
  } else {
    handle_dns_stop();
  }
}

fn handle_dns_start() {
  let mut command = Command::new("docker");
  command
  .arg("run")
  .arg("--name")
  .arg("dcmddns")
  .arg("-p")
  .arg("53:53/udp")
  .arg("--rm")
  .arg("-d")
  .arg("paulweinsberg/dcmddns:latest");

  super::exec_command(command);
}
fn handle_dns_stop() {
  let mut command = Command::new("docker");
  command
  .arg("stop")
  .arg("dcmddns");

  super::exec_command(command);
}