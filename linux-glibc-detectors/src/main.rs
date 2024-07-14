#![no_std]
#![no_main]

// Link with libc
#[link(name = "c")]
extern "C" {}

extern crate libc;

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        libc::puts(libc::gnu_get_libc_version());
    }
    0
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
