use libloading::{Library, Symbol};
use std::path::Path;

use crate::Socket;

pub struct SocketHandler {
    socket: *mut Socket,
    library: Library,
}

impl SocketHandler {
    pub fn switch_state(&mut self) {
        switch_state(&self.library, self.socket);
    }
    pub fn get_state(&mut self) {
        get_state(&self.library, self.socket);
    }
}

impl Default for SocketHandler {
    fn default() -> Self {
        let library = new_library();
        let socket = new_socket(&library);
        SocketHandler { library, socket }
    }
}

fn new_library() -> Library {
    unsafe { Library::new(lib_pth()).unwrap() }
}

impl Drop for SocketHandler {
    fn drop(&mut self) {
        drop(&self.library, self.socket);
    }
}
#[cfg(target_os = "linux")]
pub fn lib_pth() -> &'static Path {
    Path::new("target/release/libsocket.so")
}

#[cfg(target_os = "windows")]
pub fn lib_pth() -> &'static Path {
    Path::new("target/release/libsocket.dll")
}
#[cfg(target_os = "macos")]
pub fn lib_pth() -> &'static Path {
    Path::new("target/release/libsocket.dylib")
}

type NewSocket = unsafe extern "C" fn() -> *mut Socket;
type SwitchSocketState = unsafe extern "C" fn(*mut Socket);
type GetSocketState = unsafe extern "C" fn(*const Socket);
type DropSocket = unsafe extern "C" fn(*const Socket);

fn new_socket(library: &Library) -> *mut Socket {
    unsafe {
        let fnc: Symbol<NewSocket> = library.get(b"new_socket").unwrap();
        fnc()
    }
}

fn switch_state(library: &Library, socket: *mut Socket) {
    unsafe {
        let fnc: Symbol<SwitchSocketState> = library.get(b"switch_state").unwrap();
        fnc(socket)
    }
}

fn get_state(library: &Library, socket: *const Socket) {
    unsafe {
        let fnc: Symbol<GetSocketState> = library.get(b"get_state").unwrap();
        fnc(socket)
    }
}

fn drop(library: &Library, socket: *const Socket) {
    unsafe {
        let fnc: Symbol<DropSocket> = library.get(b"drop").unwrap();
        fnc(socket)
    }
}
