# ciudse-telemetry

This repository includes the frontend and backend code for CIUDSE's telemetry system

## Prerequisites

Docker
Git

## Use

```
wget https://github.com/CIUDSE/ciudse-telemetry/releases/download/latest/ciudse-telemetry-arm64.tar.gz
docker load -i ciudse-telemetry-arm64.tar.gz
wget https://github.com/CIUDSE/ciudse-telemetry/releases/download/latest/docker-compose.yml
docker-compose up
```

Clone repository

From the root directory:
```
COMPOSE_DOCKER_CLI_BUILD=1 DOCKER_BUILDKIT=1 docker-compose build
docker-compose up
```

It is also possible to import the images built by GitHub Actions with `docker import`, instead of building them from the source code. In that case, it is only necessary to copy the `docker-compose.yml` file.

This will build and run 2 containers: the telemetry server (which also serves the frontend) and the database. OpenMCT is available at localhost:8080

The database container will mount the `./db` directory and will save data there. This allows making backups and transfering data between different systems with ease

## Development

The repository contains a configuration for a devcontainer for VSCode which includes all tools necessary for development. To use it, first install VSCode and open the repository folder. VSCode will automatically recommend to install the "Remote - Containers" extension, if not, install it manually

After, press the green arrow in the bottom left corner of VSCode, a menu will apear, select "Reopen Folder Locally". VSCode will build the devcontainer and open the repo in the devcontainer. You will now have all the development tools and libraries available to you: python, cargo, npm, etc. and the vscode extension that will make development easier

**Frontend Development**

Navigate to the `telemetry-frontend` folder. Development dependecies must be installed first with `npm install`, and then one can use a development server for the frontend with the command `npm run dev`.

**Backend Development**

The backend can be built with docker, or if needed, it can also be built and run locally with `cargo run`. In the case of executing the server locally, it might be needed to change the ports of the server. It is recommended to build the server with docker, since it doesn't take much time and the ports and database are configured correctly

**Generate telemetry**

The `telemetry-utils` folder contains Python scripts that generate example temetry data and send it to the server via WebSockets.