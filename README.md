# ciudse-telemetry

Este repositorio contiene una imagen de Docker para el sistema de telemetria. Esto incluye: el servidor de telemetria, frontend y base de datos.

## Requisitos previos

Docker
Git

## Uso

Clonar repositorio

Desde el directorio del repositorio:
```
docker-compose build
docker-compose up
```

Esto va construir y correr 2 contenedores: el servidor de telemetria y la base de datos. OpenMCT estara disponible desde localhost:8080

El contenedor de la base de datos montara el directorio local `./db` y guardara los datos ahi. Esto permite respaldar y transferir la base de datos entre diferentes sistemas con facilidad.

## Desarrollo

El repositorio tiene una configuracion de contenedor de desarrollo para VSCode que contiene todas las herramientas necesarias para desarrollo. Para usarlo primero instale VSCode y abra la carpeta del repositorio. VSCode automaticamente recomendara instalar la extension "Remote - Containers", si no es el caso, instalarla manualmente.

Despues, presionar la flecha verde en la esquina inferior izquierda de VSCode, un menu aparecera, seleccionar "Reopen Folder Locally". VSCode reconstruira el contenedor de desarrollo y reabrira el repositorio dentro de este contenedor. Ahora tiene aceso a todas las herramientas y librerias necesarias: python, cargo, npm, etc.

El contenedor de desarrollo tambien esta configurado para tener disponible la instancia de docker del host. Puedes usar `docker build` y `docker-compose` desde la terminal integrada de VSCode.
