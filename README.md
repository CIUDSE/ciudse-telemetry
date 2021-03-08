# ciudse-telemetry

Este es un repositorio con el programa de cliente para visualizacion de telemetria de misiones espaciales.

Esta basado en la libraria OpenMCT de NASA [repositorio en GitHub](https://github.com/nasa/openmct)

## Comenzar aqui

Para desarrollar este programa necesitara Git para control de version, y NPM como manejador de dependencias.
[Descargar Git](https://git-scm.com/downloads)
[Descargar Nodejs](https://nodejs.org/en/)
*Use la version LTS de Node*

Recomiendo usar el editor Visual Studio Code
[Descargar VS Code](https://code.visualstudio.com/download)

Para empezar, clone el repositorio
```
git clone https://github.com/skrobchik/ciudse-telemetry.git
```

y abra la carpeta en Visual Studio Code. Puede abrir una terminal dentro de VS Code presionando las teclas `Ctrl` y `~` en su teclado. La tecla `~` se encuentra en la parte superior izquierda del teclado.

La terminal se abrira en el directorio del proyecto.

Para instalar todas las dependencias del proyecto use
```
npm install
```
esto puede tardar unos minutos

Ya que se instalaron todas las dependencias, puede usar el comando
```
npm run start:dev
```
para prender un servidor web de desarrollo que se actualizara automaticamente si cambia algo en el programa.
Navege a [localhost:8000](http://localhost:8000) para observar el resultado

Puede dirigirse a la documentacion de OpenMCT en su repositorio de GitHub para ver como sirve y como desarrollar plugins para el.