# Docker Compose templates

These docker templates provide a complete docker compose development kit with the aim of :

- avoid any other dependencies or heavy VM like DDEV or Docksal projects.
- use VHOST proxy throught docker container
- make your life easier by automating long and reccurent commands
- share project easily with teammates

## OS configuration (example for MacOS)

**Install dnsmasq**

`brew install dnsmasq`

**Register new mask**

`echo "address=/dev.local/127.0.0.1" >> $(brew --prefix)/etc/dnsmasq.conf`

`echo "port=53" >> $(brew --prefix)/etc/dnsmasq.conf`

Start or restart the service if is already running

`sudo brew services restart dnsmasq`


**Setup custom DNS resolver**

`sudo mkdir -v /etc/resolver`

`sudo echo "servername 127.0.0.1" >> /etc/resolver/dev.local`

✅ You're good to go

## Docksal cohabitation conf 🚧 (skip it if you're not affected)

If you still running some docksal projects, you'll have to turn off the Docksal built-in DNS by running the following command :

`fin config set --global DOCKSAL_DNS_DISABLED=1`

`fin system reset`

These commands will disable the Docksal DNS so you need to update our own DNS service by following these steps :

`echo "address=/docksal.site/127.0.0.1" >> $(brew --prefix)/etc/dnsmasq.conf`

`sudo echo "servername 127.0.0.1" >> /etc/resolver/docksal.site`

`sudo brew services restart dnsmasq`

Then, you're local will run and resolve Docksal project as before.

Note that you can not run any Docker project that listen to port 80 at the same time with Docksal, so you have to stop its system ahead by running :

`fin system stop`

Also you should redo this command after start (sometimes its required)

`sudo cp /etc/resolver/dev.local /etc/resolver/docksal.site && brew services restart dnsmasq`

**Add the CLI executable**

As the project should come with the least possible dependencies, the CLI is a simple bash file that you can customize as well.
To make it accessible from anywhere on your local machine as an executable run the two lines below :

`chmod +x dcmd`

`sudo cp dcmd /usr/local/bin`


## Project configuration

**Docker environment variables**

Copy .docker/.env.example ro .dockser/.env, then set the right variables values :

| Name |	Description |	Example |
|------|--------------|---------|
| PROJECT_ROOT	| Where your project is located	| /Users/me/Projects/myproject
| PROJECT_NAME	  | The name of your project |	my_awesome_project

**Project name**

In the .docker/docker-compose.yml file you must set the name of the project on the top (can't be loaded from env variables). my_awesome_project for example;

```yaml
version: '3.9'

name: my_awesome_project

services:
  # services below...
```

## Basics commands

**Initialisation**

In the .docker parent directory run the following command :

`dcmd up`

If you've made a change in Dockerfiles or in the docker-compose.yml you should rebuild the containers by running the same command.

**Start a project**

`dcmd start`

**Stop a project**

`dcmd stop`

**Delete project and remove containers and data**

`dcmd down`

## Custom commands

These templates provide a user friendly way to write and execute your own commands, they also come with some preconfigured commands that require no external dependencies, just shell script.

Then you can run any commands that you can find and check in the .docker/commands path. For example in a Drupal project you can run the command below to import your database :

`dcmd db-import ~/Desktop/my_awesome_db.sql`

another exeample with params :

`dcmd npm i --dev`

If you need several commands, separated by apps for example you can create a dir in the commands folder and execute commands as well :

`dcmd my_first_app npm install`

`dcmd my_second_app npm install`

## Project templates

### Drupal

This configuration has been tested on Drupal 8, 9 and 10.
The node container is optional but may be required for frontend development it has been configured to work with the [agnostic-bundler](https://github.com/PaulWeinsberg/agnostic-bundler) in a .bundler folder at project root.