use application::Socket;

#[no_mangle]
pub extern "C" fn new_socket() -> *mut Socket {
    Box::into_raw(Box::new(Socket::default()))
}

/// # Safety
/// Believe me, this unsafe is ok :)
#[no_mangle]
pub unsafe extern "C" fn switch_state(ptr: *mut Socket) {
    let socket = {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    match socket.is_on() {
        true => socket.switch_off(),
        false => socket.switch_on(),
    }
}

/// # Safety
/// Believe me, this unsafe is ok :)
#[no_mangle]
pub unsafe extern "C" fn drop(ptr: *mut Socket) {
    if ptr.is_null() {
        return;
    }
    Box::from_raw(ptr);
}

/// # Safety
/// Believe me, this unsafe is ok :)
#[no_mangle]
pub unsafe extern "C" fn get_state(ptr: *const Socket) {
    let socket = {
        assert!(!ptr.is_null());
        &*ptr
    };
    println!("{}", socket);
}
