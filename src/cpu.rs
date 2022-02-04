mod flags;
mod registers;

pub struct CPU {
    // Program Counter
    pub pc: u16,
    // Stack pointer
    pub sp: u16,
    // Registers
    pub reg: registers::Registers,
    // Flags
    pub flags : flags::Flags,
}


impl CPU {
    pub fn new() -> Self{
        CPU {
            pc : 0,
            sp : 0,
            reg : registers::Registers::new(),
            flags : flags::Flags::new(),
        }
    }
    pub fn say_hello(&self){
        println!("Hello from CPU");
    }
}

