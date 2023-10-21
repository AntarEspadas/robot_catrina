import socket
import struct
from typing import Callable
from enum import Enum


# enum with values TRACKING and LOST
class State(Enum):
    TRACKING = 0
    LOST = 1


def to_dict(tup: tuple):
    keys = [
        "timestamp",
        "id",
        "width",
        "height",
        "blink_l",
        "blink_r",
        "success",
        "pnp_error",
        "q0",
        "q1",
        "q2",
        "q3",
        "yaw",
        "pitch",
        "roll",
        "x",
        "y",
        "z",
    ]

    return {key: value for key, value in zip(keys, tup)}


class FaceTracker:
    def __init__(self, address: str, port: int) -> None:
        self.server_socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        self.server_socket.settimeout(15)
        self.address = address
        self.port = port

        self.on_data: Callable[[dict], None] | None = None
        self.on_face_lost: Callable[[], None] | None = None
        self.on_timeout: Callable[[], None] | None = None
        self.state = State.LOST

    def start(self):
        self.server_socket.bind((self.address, self.port))

        while True:
            try:
                message, _ = self.server_socket.recvfrom(2048)
            except TimeoutError:
                if self.state == State.TRACKING:
                    self.server_socket.settimeout(15)
                    self.state = State.LOST
                    if self.on_face_lost is not None:
                        self.on_face_lost()
                elif self.state == State.LOST:
                    self.state = State.LOST
                    self.server_socket.settimeout(None)
                    if self.on_timeout is not None:
                        self.on_timeout()
                continue

            self.state = State.TRACKING
            self.server_socket.settimeout(0.2)
            unpacked_data = struct.unpack(
                ("di" + ("f" * 4) + "B"),
                message[:29],
            )
            unpacked_data += struct.unpack("f" * 11, message[29:73])

            data = to_dict(unpacked_data)

            if self.on_data is not None:
                self.on_data(data)
