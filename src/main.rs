mod cpu;
mod render;

fn main() {
    let mut test_cpu = cpu::CPU::new();
    test_cpu.reg.set_af(0xFF00);
    let _ = test_cpu.reg.get_af();
    println!("{0}", test_cpu.reg.get_af());
    println!("Hello from Main!");
    //render::main();
}
