# ciudse-telemetry

Este repositorio incluye el codigo de frontend y backend para el sistema de telemetria de CIUDSE\
*This repository includes the frontend and backend code for CIUDSE's telemetry system* 


## Requisitos previos  *Prerequisites*

Docker
Git

## Uso  *Use*

Clonar repositorio\
*Clone repository*

Desde el directorio del repositorio:\
*From the root directory:*
```
COMPOSE_DOCKER_CLI_BUILD=1 DOCKER_BUILDKIT=1 docker-compose build
docker-compose up
```

Tambien es posible importar las imagenes construidas por GitHub Actions con `docker import`, en vez de construirlas desde el codigo fuente. En este caso solo se necesitaria copiar el archivo de `docker-compose.yml`.\
*It is also possible to import the images built by GitHub Actions with `docker import`, instead of building them from the source code. In that case, it is only necessary to copy the `docker-compose.yml` file.*

Esto va construir y correr 2 contenedores: el servidor de telemetria (que tambien sirve el frontend) y la base de datos. OpenMCT estara disponible desde localhost:8080\
*This will build and run 2 containers: the telemetry server (which also serves the frontend) and the database. OpenMCT is available at localhost:8080*

El contenedor de la base de datos montara el directorio local `./db` y guardara los datos ahi. Esto permite respaldar y transferir la base de datos entre diferentes sistemas con facilidad.\
*The database container will mount the `./db` directory and will save data there. This allows making backups and transfering data between different systems with ease*

## Desarrollo  *Development*

El repositorio tiene una configuracion de contenedor de desarrollo para VSCode que contiene todas las herramientas necesarias para desarrollo. Para usarlo primero instale VSCode y abra la carpeta del repositorio. VSCode automaticamente recomendara instalar la extension "Remote - Containers", si no es el caso, instalarla manualmente.\
*The repository contains a configuration for a devcontainer for VSCode which includes all tools necessary for development. To use it, first install VSCode and open the repository folder. VSCode will automatically recommend to install the "Remote - Containers" extension, if not, install it manually*

Despues, presionar la flecha verde en la esquina inferior izquierda de VSCode, un menu aparecera, seleccionar "Reopen Folder Locally". VSCode reconstruira el contenedor de desarrollo y reabrira el repositorio dentro de este contenedor. Ahora tiene aceso a todas las herramientas y librerias necesarias: python, cargo, npm, etc. y las extensiones de vscode que facilitan el desarrollo dentro del contenedor.\
*After, press the green arrow in the bottom left corner of VSCode, a menu will apear, select "Reopen Folder Locally". VSCode will build the devcontainer and open the repo in the devcontainer. You will now have all the development tools and libraries available to you: python, cargo, npm, etc. and the vscode extension that will make development easier*

**Desarrollo de frontend  *Frontend Development***

Navegar a la carpeta `telemetry-frontend`. Dentro de esta carpeta, primero se deben instalar las dependencias `npm install` y despues puedes usar un servidor de desarrollo para el frontend con el comando `npm run dev`.\
*Navigate to the `telemetry-frontend` folder. Development dependecies must be installed first with `npm install`, and then one can use a development server for the frontend with the command `npm run dev`.*

**Desarrollo de backend *Backend Development***

Se puede construir el backend con docker, o si se desea, puede construirse y ejecutarse localmente con `cargo run`. En el caso de ejecutar el servidor localmente, puede ser que se tenga que cambiar el puerto del servidor. Es recomendable reconstruirlo con docker, ya que no toma mucho tiempo y los puertos de la base de datos son configurados correctamente.\
*The backend can be built with docker, or if needed, it can also be built and run locally with `cargo run`. In the case of executing the server locally, it might be needed to change the ports of the server. It is recommended to build the server with docker, since it doesn't take much time and the ports and database are configured correctly*

**Generar telemetria  *Generate telemetry***
La carpeta `telemetry-utils` contiene programas de Python que generan datos de telemetria muestra y los mandan al servidor por WebSockets.\
*The `telemetry-utils` folder contains Python scripts that generate example temetry data and send it to the server via WebSockets.*