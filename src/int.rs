pub struct Int24 {
    // use a pointer because in ez80 a pointer is 3 bytes
    data: *const core::ffi::c_void,
}

impl Int24 {
    // conversions should be improved
    pub fn to_i32(&self) -> i32 {
        let mut value = unsafe { core::mem::transmute::<*const core::ffi::c_void, i32>(self.data) };
        // clear last byte
        value &= 0x00FFFFFF;
        value
    }

    pub fn from_i32(value: i32) -> Self {
        Int24 {
            data: unsafe { core::mem::transmute::<i32, *const core::ffi::c_void>(value) },
        }
    }
}

pub struct Uint24 {
    data: *const core::ffi::c_void,
}

impl Uint24 {
    pub fn to_i32(&self) -> i32 {
        let mut value = unsafe { core::mem::transmute::<*const core::ffi::c_void, i32>(self.data) };
        // clear last byte
        value &= 0x00FFFFFF;
        value
    }

    pub fn from_i32(value: i32) -> Self {
        Int24 {
            data: unsafe { core::mem::transmute::<i32, *const core::ffi::c_void>(value) },
        }
    }
}
