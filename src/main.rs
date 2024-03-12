mod cpu;

fn main() {
  let mut cpu = cpu::Core::new();
  cpu.run();
}
