#![no_main]
#![no_std]

use i24::i24 as I24; // Import the i24 type

#[unsafe(no_mangle)]
unsafe fn main() -> I24 {
    unsafe { os_ClrLCD() };
    unsafe { os_HomeUp() };
    unsafe { os_DrawStatusBar() };
    unsafe { os_PutStrFull(b"Hello from Rust!\x00") };
    while unsafe { os_GetCSC() } == 0 {}
    I24::from(0u16)
}

// #[panic_handler]
// fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
//     loop {}
// }

#[allow(improper_ctypes)]
unsafe extern "C" {
    fn os_ClrLCD();
    fn os_HomeUp();
    fn os_DrawStatusBar();
    fn os_PutStrFull(str: *const [u8]) -> isize;
    fn os_GetCSC() -> i8;
}