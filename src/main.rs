#![no_main]
#![no_std]
mod allocator;
mod tice;

extern crate alloc;
use alloc::string::String;

#[no_mangle]
unsafe fn main() -> i32 {
    let str = String::from("Hello from Rust allocator!");
    tice::os::clr_lcd();
    tice::os::clr_lcd();
    tice::os::home_up();
    tice::os::draw_status_bar();
    tice::os::put_str_full("Hello world from Rust!");
    tice::os::new_line();
    tice::os::put_str_full(&str);
    // wait for key press
    while tice::os::get_csc() == 0 {}
    0
}

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
