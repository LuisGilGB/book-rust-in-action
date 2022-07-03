struct CPU {
  program_counter: usize,
  registers: [u8; 16],
  memory: [u8; 4096],
  stack: [u16; 16],
  stack_pointer: usize,
}

impl CPU {
  fn read_opcode(&self) -> u16 {
    (self.memory[self.program_counter] as u16) << 8 | (self.memory[self.program_counter + 1] as u16)
  }
  fn add_xy(&mut self, x: u8, y: u8) {
    let arg1 = self.registers[x as usize];
    let arg2 = self.registers[y as usize];

    let (val, overflow) = arg1.overflowing_add(arg2);
    self.registers[x as usize] = val;

    if overflow {
      self.registers[0xf] = 1;
    } else {
      self.registers[0xf] = 0;
    }
  }
  fn call(&mut self, nnn: usize) {
    let sp = self.stack_pointer;
    let stack = &mut self.stack;

    if sp >= stack.len() {
      panic!("Stack overflow!");
    }

    stack[sp] = self.program_counter as u16;
    self.stack_pointer += 1;
    self.program_counter = nnn;
  }
  fn ret(&mut self) {
    if self.stack_pointer == 0 {
      panic!("Stack underflow!");
    }

    self.stack_pointer -= 1;
    let call_address = self.stack[self.stack_pointer];
    self.program_counter = call_address as usize;
  }
  fn run(&mut self) {
    loop {
      let opcode = self.read_opcode();
      self.program_counter += 2;

      let c = ((opcode & 0xf000) >> 12) as u8;
      let x = ((opcode & 0x0f00) >> 8) as u8;
      let y = ((opcode & 0x00f0) >> 4) as u8;
      let d = ((opcode & 0x000f) >> 0) as u8;

      let nnn = ((opcode & 0x0fff) >> 0) as usize;

      match (c, x, y, d) {
        (0, 0, 0, 0) => { return; }
        (0x0, 0x0, 0xe, 0xe) => self.ret(),
        (0x2, _, _, _) => self.call(nnn),
        (0x8, _, _, 0x4) => self.add_xy(x, y),
        _ => todo!("opcode {:04x}", opcode),
      }
    }
  }
}

fn main() {
  let mut cpu = CPU {
    program_counter: 0x000,
    registers: [0x00; 16],
    memory: [0x000; 4096],
    stack: [0x00; 16],
    stack_pointer: 0,
  };

  cpu.registers[0] = 5;
  cpu.registers[1] = 10;
  cpu.registers[2] = 10;
  cpu.registers[3] = 10;

  let mem = &mut cpu.memory;

  let add_twice: [u8; 6] = [
    0x80, 0x14,
    0x80, 0x14,
    0x00, 0xee,
  ];

  mem[0x0100..0x106].copy_from_slice(&add_twice);

  mem[0] = 0x21;
  mem[1] = 0x00;
  mem[2] = 0x21;
  mem[3] = 0x00;

  cpu.run();

  assert_eq!(cpu.registers[0], 45);

  println!("5 + (10 * 2) + (10 * 2) = {} (stored in the register 0)", &cpu.registers[0]);
}
