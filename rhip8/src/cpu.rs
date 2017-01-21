extern crate rand;

struct Display {
  pixels: [[bool; 64]; 32],
}

struct Input {
  keys: [bool; 16],
}

struct Cpu {
  display: Display,
  input: Input,
  dm: DataMemory,
  R: [u8; 16],
  RSoundTimer: u8,
  RDelayTimer: u8,
  PC: usize,
  SP: usize,
  VX: usize,
  VY: usize,
  I: usize,
}

struct DataMemory {
  stack: [u16; 16],
  memory: [u8; 4096],
}

impl Cpu {
  fn clearScreen(&self)  { self.display.pixels = [[false; 64]; 32]; }

  fn pcReturn(&self) {
    self.PC = self.dm.stack[self.SP as usize] as usize;
    self.SP -= 1;
  }
  
  fn jumpNNN(&self, opcode: u16) {
    self.PC = (opcode & 0x0FFF) as usize;
  }

  fn callNNN(&self, opcode:u16) {
    self.dm.stack[self.SP] = self.PC as u16;
    self.SP += 1;
    self.PC = (opcode & 0x0FFF) as usize;
  }

  fn skipPCIfVxIsNN(&self, opcode:u16) {
    let NN = (opcode & 0x00FF) as u8;
    if self.R[self.VX ] == NN {
      self.PC += 2;
    }
  }

  fn skipPCIfVxIsNotNN(&self, opcode:u16) {
    let NN = (opcode & 0x00FF) as u8;
    if self.R[self.VX ] != NN {
      self.PC += 2;
    }
  }

  fn skipPCIfVxIsVy(&self, opcode:u16) {
    if self.R[self.VX ] == self.R[self.VY ] {
      self.PC += 2;
    }
  }

  fn setVxToNN(&self, opcode:u16) {
    let NN = (opcode & 0x00FF) as u8;
    self.R[self.VX ] = NN;
  }

  fn addNNToVx(&self, opcode:u16) {
    let NN = (opcode & 0x00FF) as u8;
    self.R[self.VX ] += NN;
  }

  fn setVxToVy(&self) {
    self.R[self.VX ] = self.R[self.VY ];
  }

  fn setVxToVxOrVy(&self) {
    self.R[self.VX ] = self.R[self.VX ] | self.R[self.VY ];
  }

  fn setVxToVxAndVy(&self) {
    self.R[self.VX ] = self.R[self.VX] & self.R[self.VY];
  }

  fn setVxToVxXorVy(&self) {
    self.R[self.VX ] = self.R[self.VX] ^ self.R[self.VY];
  }

  fn addVyToVx(&self) {
    if self.R[self.VX ] + self.R[self.VY] > 0xff {
      self.R[self.VX ] = 1;
    } else {
      self.R[self.VX ] = 0;
    }
    self.R[self.VX ] = self.R[self.VX] + self.R[self.VY];
  }
  
  fn skipPCIfVxIsNotVy(&self) {
    if self.R[self.VX ] != self.R[self.VY ] {
      self.PC += 2;
    }
  }

  fn setIToNNN(&self, opcode: u16) {
    self.I = (opcode & 0x0FFF) as usize;
  }

  fn jumpNNNPlusV0(&self, opcode:u16) {
    self.I = ((opcode & 0x0FFF) + self.R[0] as u16) as usize;
  }

  fn badOpcode(&self) {
  }

  fn cycle(&self, opcode: u16) {
    self.PC += 2;

    match opcode & 0xF000 {
      0x0000 => match opcode {
        0x00E0 => self.clearScreen(),
        0x00EE => self.pcReturn(),
        _ => self.badOpcode(),
      },
      0x1000 => self.jumpNNN(opcode),
      0x2000 => self.callNNN(opcode),
      0x3000 => self.skipPCIfVxIsNN(opcode), 
      0x4000 => self.skipPCIfVxIsNotNN(opcode),
      0x5000 => self.skipPCIfVxIsVy(opcode),
      0x6000 => self.setVxToNN(opcode), 
      0x7000 => self.addNNToVx(opcode), 
      0x8000 => {
        match opcode & 0xF00F {
          0x800 => self.setVxToVy(),
          0x8001 => self.setVxToVxOrVy(), 
          0x8002 => self.setVxToVxAndVy(),
          0x8003 => self.setVxToVxXorVy(),
          0x8004 => self.addVyToVx(),
        }
      }
      0x9000 => self.skipPCIfVxIsNotVy(),
      0xA000 => self.setIToNNN(opcode),
      0xB000 => self.jumpNNNPlusV0(opcode),
      0xC000 => {
        self.R[self.VX] = (rand::<u8>() & (opcode & 0x00ff)) as u8;
      }
      0xE000 => {
        match opcode & 0x00FF {
          0x009E => {
            if self.input.keys[self.R[self.VX] as usize] == true {
              self.PC += 1;
            }
          }
          0x00A1 => {
            if self.input.keys[self.R[self.VX] as usize] == false {
              self.PC += 1;
            }
          }
        }
      }
      0xF000 => {
        match opcode & 0x00FF {
          0x0007 => {
            self.R[self.VX ] = self.RDelayTimer;
          }
          0x000A => {
            // BLOCK to get key
          }
          0x0015 => {
            self.RDelayTimer = self.R[self.VX];
          }
          0x0018 => {
            self.RSoundTimer = self.R[self.VX];
          }
          0x001E => {
            self.I += self.R[self.VX] as usize;
          }
        }
      } 
    }
  }
}

fn main() {
  let display = Display {
    pixels: [[false; 64]; 32],
  };

  let input = Input {
    keys: [false; 16],
  };

  let dm = DataMemory {
    stack: [0; 16],
    memory: [0; 4096],
  };

  let cpu = Cpu {
    display: display,
    input: input,
    dm: dm,
    R: [0; 16],
    RSoundTimer: 0,
    RDelayTimer: 0,
    PC: 0,
    SP: 0,
    VX: 0,
    VY: 0,
    I: 0,
  };
}
