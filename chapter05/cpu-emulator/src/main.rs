struct CPU {
  program_counter: usize,
  registers: [u8; 16],
  memory: [u8; 0x1000],
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
  fn run(&mut self) {
    loop {
      let opcode = self.read_opcode();
      self.program_counter += 2;

      let c = ((opcode & 0xf000) >> 12) as u8;
      let x = ((opcode & 0x0f00) >> 8) as u8;
      let y = ((opcode & 0x00f0) >> 4) as u8;
      let d = ((opcode & 0x000f) >> 0) as u8;

      match (c, x, y, d) {
        (0, 0, 0, 0) => { return; }
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
  };

  cpu.registers[0] = 5;
  cpu.registers[1] = 10;
  cpu.registers[2] = 10;
  cpu.registers[3] = 10;

  let mem = &mut cpu.memory;
  mem[0] = 0x80; mem[1] = 0x14;
  mem[2] = 0x80; mem[3] = 0x24;
  mem[4] = 0x80; mem[5] = 0x34;

  cpu.run();

  assert_eq!(cpu.registers[0], 35);

  println!("5 + 10 + 10 + 10 = {} (stored in the register 0)", &cpu.registers[0]);
}
