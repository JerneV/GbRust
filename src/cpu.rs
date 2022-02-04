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
    // Total Memory
    memory: [u8; 0xFFFF]
}


impl CPU {
    pub fn new() -> Self{
        CPU {
            pc : 0,
            sp : 0,
            reg : registers::Registers::new(),
            flags : flags::Flags::new(),
            memory : [0; 0xFFFF],
        }
    }

    // RESET THE CPU
    //TODO make sure everything is pointed at the right place
    fn reset(&mut self){
        self.pc = 0;
        self.sp = 0;
        self.reg.reset();
        self.flags.reset();
    }

    fn read_memory(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn write_memory(&mut self, address: u16, data: u8){
        self.memory[address as usize] = data
    }
}

