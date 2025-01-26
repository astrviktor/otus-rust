import ctypes

# Загрузка динамической библиотеки
lib = ctypes.CDLL('./libsocket.so')

# Определение типов функций
lib.socket_new.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_bool, ctypes.c_float]
lib.socket_new.restype = ctypes.c_void_p

lib.socket_free.argtypes = [ctypes.c_void_p]
lib.socket_free.restype = None

lib.socket_turn_on.argtypes = [ctypes.c_void_p]
lib.socket_turn_on.restype = None

lib.socket_turn_off.argtypes = [ctypes.c_void_p]
lib.socket_turn_off.restype = None

lib.socket_get_state.argtypes = [ctypes.c_void_p]
lib.socket_get_state.restype = ctypes.c_bool

lib.socket_get_power_consumption.argtypes = [ctypes.c_void_p]
lib.socket_get_power_consumption.restype = ctypes.c_float

lib.socket_get_name.argtypes = [ctypes.c_void_p]
lib.socket_get_name.restype = ctypes.c_char_p

lib.socket_describe.argtypes = [ctypes.c_void_p]
lib.socket_describe.restype = ctypes.c_char_p


class Socket:
    def __init__(self, name: str, description: str, is_on: bool, power_consumption: float):
        self.name = name
        self.description = description
        self.socket_ptr = lib.socket_new(
            name.encode('utf-8'),
            description.encode('utf-8'),
            is_on,
            power_consumption
        )

    def __del__(self):
        if self.socket_ptr:
            lib.socket_free(self.socket_ptr)

    def turn_on(self) -> None:
        lib.socket_turn_on(self.socket_ptr)

    def turn_off(self) -> None:
        lib.socket_turn_off(self.socket_ptr)

    def get_state(self) -> bool:
        return bool(lib.socket_get_state(self.socket_ptr))

    def get_power_consumption(self) -> float:
        return float(lib.socket_get_power_consumption(self.socket_ptr))

    def get_name(self) -> str:
        name_ptr = lib.socket_get_name(self.socket_ptr)
        if not name_ptr:
            return "Unknown"
        return ctypes.string_at(name_ptr).decode('utf-8')

    def describe(self) -> str:
        description_ptr = lib.socket_describe(self.socket_ptr)
        if not description_ptr:
            return "Unknown"
        return ctypes.string_at(description_ptr).decode('utf-8')

    def info(self) -> str:
        state = 'On' if self.get_state() else 'Off'
        return (
            f"Device info - Socket name: {self.get_name()}, "
            f"description: {self.describe()}, "
            f"power consumption: {self.get_power_consumption():.1f}, "
            f"state: {state}\n"
        )


def main():
    socket = Socket("MySocket", "A smart socket", False, 50.5)
    print(socket.info())

    socket.turn_on()
    print(socket.info())

    socket.turn_off()
    print(socket.info())


if __name__ == "__main__":
    main()