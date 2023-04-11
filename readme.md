# Docker Commands

**Version: 1.0.7**

Docker commands provide a lightweight docker compose development kit with the aim of :

- avoiding any other dependencies or heavy VM like DDEV or Docksal projects.
- using VHOST proxy through docker container
- making your life easier by automating long and reccurent commands
- sharing project easily with teammates

**Does the project still working without Docker Commands CLI ?**

Yes, Docker Commands it's just a wrapper, not a requirement, it's up to the developer to use it or not. If you don't want to use it, setup a dns server on your own to keep VHOST working otherwise use localhost as most of the project under docker compose.

**How does it work ?**

- A lightweight CLI to use custom, built-in commands and run local DNS server
- A simple configuration to resolve VHOSTS from you local machine

# Get Started

## CLI installation

As the project should come with the least possible dependencies, the CLI is a standalone executable and it's the only thing you need to install.

| OS | Architecture | Link |
|----|--------------|------|
| MacOS | apple chip (arm64) | [download](https://github.com/PaulWeinsberg/dcmd/raw/main/cli/bin/aarch64-apple-darwin/dcmd) |
| MacOS | intel chip (amd64) | [download](https://github.com/PaulWeinsberg/dcmd/raw/main/cli/bin/x86_64-apple-darwin/dcmd) |
| Linux | aarch64 (arm64) | [download](https://github.com/PaulWeinsberg/dcmd/raw/main/cli/bin/aarch64-unknown-linux-gnu/dcmd) |
| Linux (or Windows WSL2) | x86_64 (amd64) | [download](https://github.com/PaulWeinsberg/dcmd/raw/main/cli/bin/x86_64-unknown-linux-gnu/dcmd) |

To make it accessible from anywhere on your local machine as an executable run the two lines below in your shell or in WSL for Windows :

```sh
chmod +x dcmd
sudo mv dcmd /usr/local/bin/dcmd
```

Or find any other path that exist in your $PATH variable.

**Fix MacOS quarantine issue :**

```sh
sudo xattr -d com.apple.quarantine /usr/local/bin/dcmd
```

## DNS domain

Docker commands needs to resolve VHOSTS from your local machine, in order to do so you need to setup a custom DNS domain that point to your local DNS server.

### MacOS

**Setup custom DNS resolver**

```sh
sudo mkdir -v /etc/resolver
echo "nameserver 127.0.0.1" | sudo tee /etc/resolver/dev.local
```

### Linux (major distributions)

**Setup custom DNS resolver**

```sh
echo "nameserver 127.0.0.1" | sudo tee -a /etc/resolv.conf
```

Note that if the queries are too long maybe you already have a nameserver configured which it used before and then switch to the right one, in that case you can edit the file /etc/resolv.conf and move the previous line on top of it.

### Windows WSL2

**Setup custom DNS resolver**

Run PowerShell as administrator then run the following commands :

```sh
Add-DnsClientNrptRule -Namespace ".dev.local" -NameServers "127.0.0.1"
```

## DNS server

### Start and stop the DNS server

In order to resolves *.dev.local domains from your host start the docker DNS server by running :

```sh
dcmd dns start
```

You can stop it whenever you want

```sh
dcmd dns stop
```

### Common issues
<details>
  <summary>Port 53 already alocated</summary>
  <br>
  If you have any issue related to the port 53, check if you have another DNS server running on your machine (dnsmasq, docksal, ddev dns for example), if so stop it and try again.

  Example with docksal :

  ```sh
  fin system stop
  ```

  Example with dnsmaq on MacOS :

  ```sh
  sudo brew services stop dnsmasq
  ```

  Example with dnsmaq on Linux :

  ```sh
  sudo systemctl stop dnsmasq
  # Can be disabled to avoid restarting it on reboot
  sudo systemctl disable dnsmasq
  ```

  Example with systemd-resolved :

  ```sh
  sudo systemctl stop systemd-resolved
  # Can be disabled to avoid restarting it on reboot
  sudo systemctl disable systemd-resolved
  ```
  <br>
</details>

## Project configuration

**Docker environment variables**

Copy .docker/.env.example to .dockser/.env, then set the right variables values for your project :

| Name |	Description |	Example |
|------|--------------|---------|
| SUBNET_BASE	  | Each project should have its own subnet base | 192.168.[1-254] or 172.17.[1-254]
| PROJECT_ROOT  | Where your project is located	| /Users/me/Projects/myproject
| PROJECT_NAME	| The name of your project      |	my_awesome_project

**Project name**

In the .docker/docker-compose.yml file you must set the name of the project on the top (can't be loaded from env variables). my_awesome_project for example;

```yaml
version: '3.9'

name: my_awesome_project

services:
  # services below...
```

## Built-in commands

Built-in commands are the ones that are provided by the CLI. Project scoped commands should run from the project root directory, otherwise you will get an error.

| Command | Description | Type |
|---------|-------------|------|
| dcmd up | Initialise the project by creating services, should run again if you make a changes in your compose file | Project |
| dcmd down | Remove project containers and related data | Project |
| dcmd start [service] | Start the project or a service | Project |
| dcmd stop [service] | Stop the project or a service | Project |
| dcmd restart [service] | Restart the project or a service | Project |
| dcmd ls | List available commands | Project |
| dcmd help | Show help | System |
| dcmd dns start | Start the DNS server | System |
| dcmd dns stop | Stop the DNS server | System |

## Custom commands

These templates provide a user friendly way to write and execute your own commands, they also come with some pre-configured commands that require no external dependencies, just shell script. Then you can run any commands that you can find and check in the .docker/commands path. For example in a Drupal project you can run the command below to import your database :

```sh
dcmd db-import ~/Desktop/my_awesome_db.sql
```

another exeample with params :

```sh
dcmd npm i --dev
```

If you need several commands, separated by apps for example you can create a dir in the commands folder and execute commands as well :

```sh
dcmd my_first_app npm install
dcmd my_second_app npm install
```

Finally every variables that you set in the .docker/.env file can be used in your commands script.

## Project structure

**Default project structure**

```bash
project_root
├── .docker
│   ├── .env
│   ├── commands
│   │   ├── my_custom_command
│   ├── docker-compose.yml
│   ├── etc
│   │   └── service_name
│   │       └── service_config.conf
│   └── services
│       └── service_name
│           └── Dockerfile
├── dcmd.toml # optional
└── ...every files related to your project, framework, etc
```

**Custom project structure**

If you need to change the default project structure you can do it by creating a dcmd.toml file at project root and set the right paths.

All the paths are relative to the project root but you can use absolute paths as well. Note that every variables are required once this file is created.

```toml
[docker]
folder = '.docker'
compose_file = '.docker/docker-compose.yml'
compose_env_file = '.docker/.env'
stop_timeout = 3
```

# Project templates

## Drupal

This configuration has been tested on Drupal 8, 9 and 10.
The node container is optional but may be required for frontend development it has been configured to work with the [agnostic-bundler](https://github.com/PaulWeinsberg/agnostic-bundler) in a .bundler folder at project root.

# Tested Docker versions

## MacOS

| Docker Desktop | Docker Engine | Docker Compose |
|----------------|---------------|----------------|
| 4.18.0         | 20.10.24      | 2.17.2         |
| 4.17.0         | 20.10.23      | 2.15.1         |

## Linux

| Docker Engine | Docker Compose |
|---------------|----------------|
| 23.0.3        | 2.17.2         |
| 20.10.24      | 2.17.2         |
| 20.10.23      | 2.15.1         |

## Windows WSL2

| Docker Desktop | Docker Engine | Docker Compose |
|----------------|---------------|----------------|
| 4.18.0         | 20.10.24      | 2.17.2         |