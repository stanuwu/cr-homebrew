#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![feature(core_intrinsics)]

include!(concat!(env!("OUT_DIR"), "\\bindings.rs"));

pub const fn null<T>() -> *mut T { 0 as *mut T }

#[no_mangle]
pub extern "C" fn run() {
    unsafe {
        consoleInit(null());
        printf("uwu".as_ptr() as *const i8);
        while appletMainLoop() {
            consoleUpdate(null());
        }
        consoleExit(null());
    }
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

pub mod lang_items;