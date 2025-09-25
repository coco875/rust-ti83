#![no_main]
#![no_std]
mod allocator;
extern crate alloc;

use alloc::string::String;
use alloc::ffi::CString;
use alloc::vec::Vec;

#[no_mangle]
unsafe fn main() -> i32 {
    let str = String::from("Hello, world!");
    let cstring = CString::new(str.clone()).unwrap();
    unsafe { os_ClrLCD() };
    unsafe { os_HomeUp() };
    unsafe { os_DrawStatusBar() };
    unsafe { os_PutStrFull(cstring.as_ptr()) };
    while unsafe { os_GetCSC() } == 0 {}
    0
}

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

extern "C" {
    fn os_ClrLCD();
    fn os_HomeUp();
    fn os_DrawStatusBar();
    fn os_PutStrFull(str: *const u8) -> i32;
    fn os_GetCSC() -> u8;
}