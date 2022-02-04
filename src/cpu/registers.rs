pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            a : 0,
            b : 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
        }
    }

    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | (self.f as u16)
    }

    pub fn set_af(&mut self, af: u16) {
        // Extract high byte and shift to low byte
        self.a = ((af & 0xFF00) >> 8) as u8;
        // Extract low byte and store
        self.f = (af & 0xFF) as u8;
    }
}