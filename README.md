# Robot Catrina

Repositorio de código para la Catrina animatrónica del concurso de Catrinas animatrónicas UAM 2023

Este proyecto requiere una tarjeta Arduino, una cámara web, una bocina, y una Raspberry Pi o similar.

Las instrucciones a continuación deben ser ejecutadas en la Raspberry Pi, al menos que se indique lo contrario.

## Instalar el proyecto y sus dependencias

### Instalar OpenSee Face

##### Clonar el repositorio

Ejecutar el siguiente comando en la terminal:

```
git clone https://github.com/emilianavt/OpenSeeFace.git
cd OpenSeeFace
```

##### Crear un entorno virtual de Python 3.9

Instalar `pyenv` con el siguiente comando:

```
sudo apt install python3-pyenv
```

Después, instalar la versión 3.9 de Python:

```
pyenv install 3.9
```

Es posible que este comando tarde algunos minutos en ejecutarse.

Finalmente, crear un entorno virtual de Python 3.9:

```
PYENV_VERSION=3.9 pyenv exec python -m venv .venv
```

##### Instalar las dependencias

Primero, activar el entorno virtual de Python 3.9

```
source .venv/bin/activate
```

Instalar las dependencias con el siguiente comando:

```
pip install onnxruntime==1.19 opencv-python==4.5.4.58 pillow==9.3.0 numpy==1.22
```

### Instalar el proyecto

##### Clonar el repositorio

En una carpeta distinta, ejecutar el siguiente comando en la terminal:

```
git clone https://github.com/AntarEspadas/robot_catrina.git
```

#### Instalar `rust` y `cargo` mediante Rustup

Seguir las [instrucciones oficiales](https://rustup.rs/) para instalar Rustup

#### Instalar dependencias

Instalar las dependencias del proyecto ejecutando los siguientes comandos:

```
sudo apt update
sudo apt install pkg-config libusb-1.0-0-dev libftdi1-dev libudev-dev
```

### Ejecutar el proyecto

##### Conectar periféricos

Asegurarse de que el Arduino, la cámara web y la bocina estén conectados a la Raspberry Pi

##### Ejecutar OpenSeeFace

Abrir una terminal en la raíz de la carpeta `OpenSeeFace` y ejecutar el siguiente comando para activar el entorno virtual de Python

```
source .venv/bin/activate
```

Una vez activado el entorno virtual, ejecutar OpenSeeFace con el siguiente comando

```
python facetracker.py --visualize 0 --pnp-points 1 --max-threads 2 --capture 0 --model 0
```

Para activar la ventana de visualización del reconocimiento facial, cambiar `--visualize 0` por `--visualize 1`.

También es posible aumentar la precisión del reconocimiento facial incrementando el valor de `--model 0` hasta un máximo de 4.

##### Cargar el programa en el Arduino

Usar la IDE de Arduino para abrir el proyecto en la carpeta `arduino` y cargarlo en la tarjeta.

El proyecto requiere la instalación de la biblioteca `Servo`.

> Nota: Este paso puede realizarse desde cualquier computadora, no es necesario hacerlo desde la Raspberry PI

##### Editar el archivo de configuración

Abrir el archivo `config.json` que se encuentra en la raíz de la carpeta `robot_catrina` y editarlo según se requiera.

```json
{
  "arduinoSerialPort": "/dev/ttyUSB0",
  "openSeeFaceAddress": "127.0.0.1:11573",
  "audioFolder": null,
  "pins": {
    "neck": 9,
    "pivot": 5,
    "leds": 12,
    "leftShoulder": 3,
    "leftElbow": 4,
    "leftWrist": 8
  }
}
```

- **arduinoSerialPort:** ubicación del puerto serial para la comunicación con el Arduino. Dependiendo del tipo de Arduino y de cómo se conecte, puede tener valores como `/dev/ttyUSB0`, `/dev/ttyUSB1`, `/dev/ttyACM0`, etc.
- **openSeeFaceAddress:** la dirección y el puerto en donde se ejecuta OpenSeeFace. A no ser que se pase alguna opción especial a OpenSeeFace, no hay razón para modificar este valor.
- **audioFolder:** ubicación de la carpeta donde se encuentran los sonidos que reproducirá la Catrina (por ejemplo `~/Music/catrina`). Para funcionar correctamente, la carpeta en cuestión debe tener dos subcarpetas, llamadas `tracking` y `lost` respectivamente. Cada una debe contener al menos un archivo de audio. Usar `null` para desactivar las funciones de audio.
- **pins:** especifica los números de los pines en la tarjeta Arduino a los cuales están conectados los diferentes servos y los leds. Modificar según convenga.

##### Ejecutar el programa principal

Abrir una terminal en la raíz de la carpeta `robot_catrina` y ejecutar el siguiente comando para compilar y ejecutar el código:

```
cargo run
```
