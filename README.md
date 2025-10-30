## Instalar el proyecto y sus dependencias

### 1. Clonar el repositorio

Ejecutar el siguiente comando en la terminal

```
git clone https://github.com/AntarEspadas/robot_catrina.git
```

### 2. Instalar `rust` y `cargo` mediante Rustup

Seguir las [instrucciones oficiales](https://rustup.rs/) para instalar Rustup

### 3. Instalar dependencias

Instalar las dependencias del proyecto ejecutando los siguientes comandos:

```
sudo apt update
sudo apt install pkg-config libusb-1.0-0-dev libftdi1-dev libudev-dev
```

## Ejecutar el proyecto

### 1. Conectar periféricos

Asegurarse de que el Arduino, la cámara web y la bocina estén conectados a la Raspberry Pi

### 2. Ejecutar OpenSeeFace

Abrir una terminal en la raíz de la carpeta `OpenSeeFace` y ejecutar el siguiente comando para activar el entorno virtual de Python

```
source .venv/bin/activate
```

Una vez activado el entorno virtual, ejecutar OpenSeeFace con el siguiente comando

```
QT_QPA_PLATFORM=xcb python facetracker.py --visualize 0 --pnp-points 1 --max-threads 2 --capture 0 --model 0
```

Para activar la ventana de visualización del reconocimiento facial, cambiar `--visualize 0` por `--visualize 1`.

También es posible aumentar la precisión del reconocimiento facial incrementando el valor de `--model 0` hastaun máximo de 4.

### 3. Ejecutar el programa principal

Abrir una terminal en la raíz de la carpeta `robot_catrina` y ejecutar el siguiente comando para compilar y ejecutar el código

```
cargo run
```
