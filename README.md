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

Despues, presionar la flecha verde en la esquina inferior izquierda de VSCode, un menu aparecera, seleccionar "Reopen Folder Locally". VSCode reconstruira el contenedor de desarrollo y reabrira el repositorio dentro de este contenedor. Ahora tiene aceso a todas las herramientas y librerias necesarias: python, cargo, npm, etc. y las extensiones de vscode que facilitan el desarrollo dentro del contenedor.

El contenedor de desarrollo tambien esta configurado para tener disponible la instancia de docker del host. Puedes usar `docker build` y `docker-compose` desde la terminal integrada de VSCode.

## Ejemplo

En el contenedor de desarrollo abrir una nueva terminal y ejecutar
```
docker-compose build
docker-compose up
```

Abrir una nueva terminal, navegar al directorio de `telemetry-utils` y ejecutar `example_telemetry_generator.py`
```
cd telemetry-utils
python example_telemetry_generator.py
```

Abrir un buscador web en la pagina `localhost:8080`

Seleccionar el punto de telemetria "Test Spaceship - Fuel". En la control de banda de tiempo de OpenMCT, seleccionar el modo de tiempo real. Se deberia de ver una onda senoidal. Refrescar la pagina, los datos generados deberian de permanecer, se puede navegar a la izquierda y derecha en el tiempo sosteniendo la tecla ALT y arrastrando con el mouse por la grafica o por la banda de tiempo.

## Nota

Aveces el boton para subir los cambios a GitHub de VSCode no funciona. Si ese es el caso, intentar usar el CLI directamente:
```
git push
```
