pub struct Int24 {
    // use a pointer because in ez80 a pointer is 3 bytes with rust it get convert to i24 but with C it's a struct of 3 u8 so can make problem for output
    data: [u8; 3],
}

impl Int24 {
    // conversions should be improved
    pub fn to_i32(&self) -> i32 {
        let mut value = unsafe { core::mem::transmute::<[u8; 4], i32>([self.data[0], self.data[1], self.data[2], 0]) };
        // sign extend
        if self.data[2] & 0x80 != 0 {
            value *= -1;
        }
        value
    }

    pub fn from_i32(value: i32) -> Self {
        let arr = unsafe { core::mem::transmute::<i32, [u8; 4]>(value) };
        Self { data: [arr[0], arr[1], arr[2]] }
    }
}

pub struct Uint24 {
    data: [u8; 3],
}

impl Uint24 {
    pub fn to_u32(&self) -> u32 {
        unsafe { core::mem::transmute::<[u8; 4], u32>([self.data[0], self.data[1], self.data[2], 0]) }
    }

    pub fn from_u32(value: u32) -> Self {
        let arr = unsafe { core::mem::transmute::<u32, [u8; 4]>(value) };
        Self { data: [arr[0], arr[1], arr[2]] }
    }
}
