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
