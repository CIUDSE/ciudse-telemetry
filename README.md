# ciudse-telemetry

Este repositorio incluye el codigo de frontend y backend para el sistema de telemetria de CIUDSE

[English Version](README_english.md)

## Requisitos previos

Docker
Git

## Uso

```
wget https://github.com/CIUDSE/ciudse-telemetry/releases/download/latest/ciudse-telemetry-arm64.tar.gz
docker load -i ciudse-telemetry-arm64.tar.gz
wget https://github.com/CIUDSE/ciudse-telemetry/releases/download/latest/docker-compose.yml
docker-compose up
```

Clonar repositorio

Desde el directorio del repositorio:
```
COMPOSE_DOCKER_CLI_BUILD=1 DOCKER_BUILDKIT=1 docker-compose build
docker-compose up
```

Tambien es posible importar las imagenes construidas por GitHub Actions con `docker import`, en vez de construirlas desde el codigo fuente. En este caso solo se necesitaria copiar el archivo de `docker-compose.yml`.

Esto va construir y correr 2 contenedores: el servidor de telemetria (que tambien sirve el frontend) y la base de datos. OpenMCT estara disponible desde localhost:8080

El contenedor de la base de datos montara el directorio local `./db` y guardara los datos ahi. Esto permite respaldar y transferir la base de datos entre diferentes sistemas con facilidad.

## Desarrollo

El repositorio tiene una configuracion de contenedor de desarrollo para VSCode que contiene todas las herramientas necesarias para desarrollo. Para usarlo primero instale VSCode y abra la carpeta del repositorio. VSCode automaticamente recomendara instalar la extension "Remote - Containers", si no es el caso, instalarla manualmente.

Despues, presionar la flecha verde en la esquina inferior izquierda de VSCode, un menu aparecera, seleccionar "Reopen Folder Locally". VSCode reconstruira el contenedor de desarrollo y reabrira el repositorio dentro de este contenedor. Ahora tiene aceso a todas las herramientas y librerias necesarias: python, cargo, npm, etc. y las extensiones de vscode que facilitan el desarrollo dentro del contenedor.

**Desarrollo de frontend**

Navegar a la carpeta `telemetry-frontend`. Dentro de esta carpeta, primero se deben instalar las dependencias `npm install` y despues puedes usar un servidor de desarrollo para el frontend con el comando `npm run dev`.

**Desarrollo de backend**

Se puede construir el backend con docker, o si se desea, puede construirse y ejecutarse localmente con `cargo run`. En el caso de ejecutar el servidor localmente, puede ser que se tenga que cambiar el puerto del servidor. Es recomendable reconstruirlo con docker, ya que no toma mucho tiempo y los puertos de la base de datos son configurados correctamente.

**Generar telemetria**
La carpeta `telemetry-utils` contiene programas de Python que generan datos de telemetria muestra y los mandan al servidor por WebSockets.