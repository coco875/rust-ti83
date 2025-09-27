#![no_main]
#![no_std]
mod allocator;
extern crate alloc;

use alloc::string::String;

#[no_mangle]
unsafe fn main() -> i32 {
    let str = String::from("Hello from Rust allocator!");
    os::clr_lcd();
    os::home_up();
    os::draw_status_bar();
    os::put_str_full("Hello world from Rust!");
    os::new_line();
    os::put_str_full(&str);
    // wait for key press
    while os::get_csc() == 0 {}
    0
}

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

mod os {
    use alloc::ffi::CString;

extern "C" {
    fn wrapper_os_ClrLCD();
    fn wrapper_os_HomeUp();
    fn os_DrawStatusBar();
    fn wrapper_os_PutStrFull(str: *const u8) -> i32;
    fn os_GetCSC() -> u8;
    fn os_NewLine();
}
pub fn clr_lcd() {
    unsafe { wrapper_os_ClrLCD() }
}
pub fn home_up() {
    unsafe { wrapper_os_HomeUp() }
}
pub fn draw_status_bar() {
    unsafe { os_DrawStatusBar() }
}
pub fn put_str_full(str: &str) -> i32 {
    let cstr = CString::new(str).unwrap();
    unsafe { wrapper_os_PutStrFull(cstr.as_ptr()) }
}
pub fn get_csc() -> u8 {
    unsafe { os_GetCSC() }
}
pub fn new_line() {
    unsafe { os_NewLine() }
}
}