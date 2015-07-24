mod internals {
    extern {
        pub fn hello() -> i32;
    }
}

#[no_mangle]
pub extern fn hello_node() -> i32 {
    unsafe { internals::hello() }
}
