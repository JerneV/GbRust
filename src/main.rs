mod cpu;

fn main() {
    let mut test_cpu = cpu::CPU::new();
    test_cpu.reg.set_af(0xFF00);
    println!("{0}", test_cpu.reg.get_af());
    println!("Hello from Main!");
}
