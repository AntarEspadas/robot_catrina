import socket
import struct
import serial
from face_tracker import FaceTracker

OPEN_SEE_FACE_PORT = 11573
ADDRESS = "127.0.0.1"
SERVO_PIN = 10


class Arduino:
    def __init__(self, port: str, baudrate: int):
        self.serial = serial.Serial(port, baudrate, timeout=1)
        self.serial.reset_input_buffer()

    def write(self, pin: int, value: int):
        self.serial.write(bytearray([pin, value]))


arduino = Arduino("/dev/ttyUSB0", 9600)


def on_data(data: dict):
    point = (data["y"] * 100, data["x"] * 100)

    if point[0] < 0:
        print("left")
        arduino.write(SERVO_PIN, 95)
    elif point[0] > 0:
        print("right")
        arduino.write(SERVO_PIN, 80)


def on_face_lost():
    arduino.write(SERVO_PIN, 90)


def main():
    try:
        tracker = FaceTracker(ADDRESS, OPEN_SEE_FACE_PORT)
        tracker.on_data = on_data
        tracker.on_face_lost = on_face_lost
        tracker.on_timeout = lambda: print("timeout")
        tracker.start()
    except:
        arduino.write(SERVO_PIN, 90)


if __name__ == "__main__":
    main()
