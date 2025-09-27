#include "stdint.h"
#include "stdlib.h"
#include <stdint.h>
#include <tice.h>

// wrap function who have uint24_t type or the tiflags call convention

void* wrapper_malloc(uint32_t size) {
    return malloc(size);
}

void* wrapper_realloc(void* ptr, uint32_t size) {
    return realloc(ptr, size);
}

void wrapper_os_MoveUp(void) {
    os_MoveUp();
}

void wrapper_os_MoveDown(void) {
    os_MoveDown();
}

void wrapper_os_HomeUp(void) {
    os_HomeUp();
}

void wrapper_os_ClrLCDFull(void) {
    os_ClrLCDFull();
}

void wrapper_os_ClrLCD(void) {
    os_ClrLCD();
}

void wrapper_os_ClrTxtShd(void) {
    os_ClrTxtShd();
}

uint32_t wrapper_os_PutStrFull(const char* string) {
    return os_PutStrFull(string);
}