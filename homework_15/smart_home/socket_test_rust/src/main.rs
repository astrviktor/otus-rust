use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_float};

#[repr(C)]
pub struct Socket {
    pub name: *const c_char,
    pub description: *const c_char,
    pub is_on: bool,
    pub power_consumption: c_float,
}

// Объявляем внешние функции из нашей библиотеки
extern "C" {
    fn socket_new(
        name: *const c_char,
        description: *const c_char,
        is_on: bool,
        power_consumption: c_float,
    ) -> *mut Socket;
    fn socket_free(socket: *mut Socket);
    fn socket_turn_on(socket: *mut Socket);
    fn socket_turn_off(socket: *mut Socket);
    fn socket_get_state(socket: *const Socket) -> bool;
    fn socket_get_power_consumption(socket: *const Socket) -> c_float;
    fn socket_get_name(socket: *const Socket) -> *const c_char;
    fn socket_describe(socket: *const Socket) -> *const c_char;
}

pub struct SafeSocket {
    ptr: *mut Socket,
}

impl SafeSocket {
    pub fn new(name: &str, description: &str, is_on: bool, power_consumption: f32) -> SafeSocket {
        let name = CString::new(name).unwrap();
        let description = CString::new(description).unwrap();

        unsafe {
            SafeSocket {
                ptr: socket_new(
                    name.as_ptr(),
                    description.as_ptr(),
                    is_on,
                    power_consumption,
                ),
            }
        }
    }

    pub fn turn_on(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                socket_turn_on(self.ptr);
            }
        }
    }

    pub fn turn_off(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                socket_turn_off(self.ptr);
            }
        }
    }

    pub fn get_state(&self) -> bool {
        unsafe {
            if !self.ptr.is_null() {
                socket_get_state(self.ptr)
            } else {
                false
            }
        }
    }

    pub fn get_power_consumption(&self) -> f32 {
        unsafe {
            if !self.ptr.is_null() {
                socket_get_power_consumption(self.ptr)
            } else {
                0.0
            }
        }
    }

    pub fn get_name(&self) -> String {
        unsafe {
            if !self.ptr.is_null() {
                CStr::from_ptr(socket_get_name(self.ptr))
                    .to_string_lossy()
                    .into_owned()
            } else {
                "Unknown".to_string()
            }
        }
    }

    pub fn describe(&self) -> String {
        unsafe {
            if !self.ptr.is_null() {
                CStr::from_ptr(socket_describe(self.ptr))
                    .to_string_lossy()
                    .into_owned()
            } else {
                "Unknown".to_string()
            }
        }
    }

    pub fn info(&self) -> String {
        format!(
            "Device info - Socket name: {}, description: {}, power consumption: {:.1}, state: {}\n",
            self.get_name(),
            self.describe(),
            self.get_power_consumption(),
            if self.get_state() { "On" } else { "Off" }
        )
    }
}

impl Drop for SafeSocket {
    fn drop(&mut self) {
        unsafe {
            socket_free(self.ptr);
        }
    }
}

fn main() {
    let mut socket = SafeSocket::new("MySocket", "A sample socket", false, 10.5);

    println!("{}", socket.info());

    socket.turn_on();
    println!("After turning on:\n{}", socket.info());

    socket.turn_off();
    println!("After turning off:\n{}", socket.info());
}
