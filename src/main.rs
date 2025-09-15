#![no_main]
#![no_std]
mod int;

use crate::int::Int24;

#[no_mangle]
unsafe fn main() -> Int24 {
    unsafe { os_ClrLCD() };
    unsafe { os_HomeUp() };
    unsafe { os_DrawStatusBar() };
    unsafe { os_PutStrFull(b"Hello from Rust!\x00" as *const u8) };
    while unsafe { os_GetCSC() } == 0 {}
    Int24::from_i32(0)
}

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

extern "C" {
    fn os_ClrLCD();
    fn os_HomeUp();
    fn os_DrawStatusBar();
}
extern "Rust" {
    fn os_PutStrFull(str: *const u8) -> Int24;
}
extern "C" {
    fn os_GetCSC() -> u8;
}