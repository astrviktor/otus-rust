extern crate libc;

use std::ffi::{CStr, CString};

#[repr(C)]
pub struct Socket {
    pub name: *const std::os::raw::c_char,
    pub description: *const std::os::raw::c_char,
    pub is_on: bool,
    pub power_consumption: f32,
}

impl Drop for Socket {
    fn drop(&mut self) {
        unsafe {
            if !self.name.is_null() {
                libc::free(self.name as *mut _);
            }
            if !self.description.is_null() {
                libc::free(self.description as *mut _);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn socket_new(
    name: *const std::os::raw::c_char,
    description: *const std::os::raw::c_char,
    is_on: bool,
    power_consumption: f32,
) -> *mut Socket {
    let name = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };
    let description = unsafe { CStr::from_ptr(description).to_string_lossy().into_owned() };

    let socket = Socket {
        name: CString::new(name)
            .unwrap_or_else(|_| CString::new("Unknown").unwrap())
            .into_raw(),
        description: CString::new(description)
            .unwrap_or_else(|_| CString::new("Unknown").unwrap())
            .into_raw(),
        is_on,
        power_consumption,
    };

    Box::into_raw(Box::new(socket))
}

#[no_mangle]
pub extern "C" fn socket_free(socket: *mut Socket) {
    if !socket.is_null() {
        unsafe {
            let _ = Box::from_raw(socket);
        }
    }
}

#[no_mangle]
pub extern "C" fn socket_turn_on(socket: *mut Socket) {
    if !socket.is_null() {
        unsafe {
            (*socket).is_on = true;
        }
    }
}

#[no_mangle]
pub extern "C" fn socket_turn_off(socket: *mut Socket) {
    if !socket.is_null() {
        unsafe {
            (*socket).is_on = false;
        }
    }
}

#[no_mangle]
pub extern "C" fn socket_get_state(socket: *const Socket) -> bool {
    if !socket.is_null() {
        unsafe { (*socket).is_on }
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn socket_get_power_consumption(socket: *const Socket) -> f32 {
    if !socket.is_null() {
        unsafe { (*socket).power_consumption }
    } else {
        0.0
    }
}

#[no_mangle]
pub extern "C" fn socket_get_name(socket: *const Socket) -> *const std::os::raw::c_char {
    if !socket.is_null() {
        unsafe { (*socket).name }
    } else {
        CString::new("Unknown")
            .unwrap_or_else(|_| CString::new("").unwrap())
            .into_raw()
    }
}

#[no_mangle]
pub extern "C" fn socket_describe(socket: *const Socket) -> *const std::os::raw::c_char {
    if !socket.is_null() {
        unsafe { (*socket).description }
    } else {
        CString::new("Unknown")
            .unwrap_or_else(|_| CString::new("").unwrap())
            .into_raw()
    }
}
