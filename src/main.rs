mod cpu;

fn main() {
    let mut test_cpu = cpu::CPU::new();
    test_cpu.say_hello();
    test_cpu.reg.set_af(0xFF00);
    println!("Hello from Main!");
}
