pub struct Core {

  // General-purpose Registers
  ax: u16, bx: u16,
  cx: u16, dx: u16,

  // Index & Pointer registers
  si: u16, di: u16,
  bp: u16, sp: u16,

  // Segment & Instruction Pointer registers
  cs: u16, ds: u16,
  ss: u16, es: u16,
  ip: u16,

  // Flag Register
  flags: u16,

  // Memory (1MB)
  memory: [u8; 0x100000],
}

impl Core {
  pub fn new() -> Self {
    Self {
      ax: 0, bx: 0,
      cx: 0, dx: 0,

      si: 0, di: 0,
      bp: 0, sp: 0,

      cs: 0, ds: 0,
      ss: 0, es: 0,
      ip: 0,

      flags: 0x0200, // Interrupt Flag Set
      memory: [0; 0x100000],
    }
  }

  pub fn run(&mut self) {loop { self.execute(); }}

  fn mem_read(&self, address: usize) -> u8 {
    self.memory[address] // Implicit return
  }

  fn mem_write(&mut self, address: usize, value: u8) {
    self.memory[address] = value;
  }
  
  fn fetch_instruction(&mut self) -> u8 {
    let opcode = self.mem_read(self.ip as usize);
    self.ip = self.ip.wrapping_add(1);
    opcode
  }

  fn execute(&mut self) {
    let opcode = self.fetch_instruction();

    match opcode {
      0x90 => self.nop(),
      _    => panic!("Unhandled Instruction: {:02X}", opcode) 
    }
  }

  fn nop(&self) {}
}