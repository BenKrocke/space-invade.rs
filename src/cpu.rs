#![allow(dead_code)] // TODO: Remove later

#[derive(Copy, Clone, Default)]
struct Registers {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}

impl Registers {
    pub fn get_hl_address(&self) -> usize {
        combine_u8_to_u16(self.h, self.l) as usize
    }
}

#[derive(Clone)]
pub struct CPU {
    registers: Registers,
    memory: Vec<u8>,
}

trait Opcodes {
    fn execute_opcode(&mut self, opcode: u8);
}

impl Opcodes for CPU {
    fn execute_opcode(&mut self, opcode: u8) {
        match opcode {
            0x00 => {}
            0x40 => {
                self.registers.b = self.registers.b;
            }
            0x41 => {
                self.registers.b = self.registers.c;
            }
            0x42 => {
                self.registers.b = self.registers.d;
            }
            0x43 => {
                self.registers.b = self.registers.e;
            }
            0x44 => {
                self.registers.b = self.registers.h;
            }
            0x45 => {
                self.registers.b = self.registers.l;
            }
            0x46 => {
                self.registers.b = self.memory[self.registers.get_hl_address()];
            }
            0x47 => {
                self.registers.b = self.registers.a;
            }
            0x48 => {
                self.registers.c = self.registers.b;
            }
            0x49 => {
                self.registers.c = self.registers.c;
            }
            0x4a => {
                self.registers.c = self.registers.d;
            }
            0x4b => {
                self.registers.c = self.registers.e;
            }
            0x4c => {
                self.registers.c = self.registers.h;
            }
            0x4d => {
                self.registers.c = self.registers.l;
            }
            0x4e => {
                self.registers.c = self.memory[self.registers.get_hl_address()];
            }
            0x4f => {
                self.registers.c = self.registers.a;
            }
            0x50 => {
                self.registers.d = self.registers.b;
            }
            0x51 => {
                self.registers.d = self.registers.c;
            }
            0x52 => {
                self.registers.d = self.registers.d;
            }
            0x53 => {
                self.registers.d = self.registers.e;
            }
            0x54 => {
                self.registers.d = self.registers.h;
            }
            0x55 => {
                self.registers.d = self.registers.l;
            }
            0x56 => {
                self.registers.d = self.memory[self.registers.get_hl_address()];
            }
            0x57 => {
                self.registers.d = self.registers.a;
            }
            0x58 => {
                self.registers.e = self.registers.b;
            }
            0x59 => {
                self.registers.e = self.registers.c;
            }
            0x5a => {
                self.registers.e = self.registers.d;
            }
            0x5b => {
                self.registers.e = self.registers.e;
            }
            0x5c => {
                self.registers.e = self.registers.h;
            }
            0x5d => {
                self.registers.e = self.registers.l;
            }
            0x5e => {
                self.registers.e = self.memory[self.registers.get_hl_address()];
            }
            0x5f => {
                self.registers.e = self.registers.a;
            }
            0x60 => {
                self.registers.h = self.registers.b;
            }
            0x61 => {
                self.registers.h = self.registers.c;
            }
            0x62 => {
                self.registers.h = self.registers.d;
            }
            0x63 => {
                self.registers.h = self.registers.e;
            }
            0x64 => {
                self.registers.h = self.registers.h;
            }
            0x65 => {
                self.registers.h = self.registers.l;
            }
            0x66 => {
                self.registers.h = self.memory[self.registers.get_hl_address()];
            }
            0x67 => {
                self.registers.h = self.registers.a;
            }
            0x68 => {
                self.registers.l = self.registers.b;
            }
            0x69 => {
                self.registers.l = self.registers.c;
            }
            0x6a => {
                self.registers.l = self.registers.d;
            }
            0x6b => {
                self.registers.l = self.registers.e;
            }
            0x6c => {
                self.registers.l = self.registers.h;
            }
            0x6d => {
                self.registers.l = self.registers.l;
            }
            0x6e => {
                self.registers.l = self.memory[self.registers.get_hl_address()];
            }
            0x6f => {
                self.registers.l = self.registers.a;
            }
            0x70 => {
                self.memory[self.registers.get_hl_address()] = self.registers.b;
            }
            0x71 => {
                self.memory[self.registers.get_hl_address()] = self.registers.c;
            }
            0x72 => {
                self.memory[self.registers.get_hl_address()] = self.registers.d;
            }
            0x73 => {
                self.memory[self.registers.get_hl_address()] = self.registers.e;
            }
            0x74 => {
                self.memory[self.registers.get_hl_address()] = self.registers.h;
            }
            0x75 => {
                self.memory[self.registers.get_hl_address()] = self.registers.l;
            }
            0x77 => {
                self.memory[self.registers.get_hl_address()] = self.registers.a;
            }
            0x78 => {
                self.registers.a = self.registers.b;
            }
            0x79 => {
                self.registers.a = self.registers.c;
            }
            0x7a => {
                self.registers.a = self.registers.d;
            }
            0x7b => {
                self.registers.a = self.registers.e;
            }
            0x7c => {
                self.registers.a = self.registers.h;
            }
            0x7d => {
                self.registers.a = self.registers.l;
            }
            0x7e => {
                self.registers.a = self.memory[self.registers.get_hl_address()];
            }
            0x7f => {
                self.registers.a = self.registers.a;
            }
            _default => {
                panic!("Opcode not implemented");
            }
        }
    }
}

trait TestValues {
    fn default_test_values() -> Self;
}

impl TestValues for CPU {
    fn default_test_values() -> Self {
        CPU {
            registers: Registers {
                b: 1,
                c: 2,
                d: 3,
                e: 4,
                h: 5,
                l: 6,
                a: 7,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl Default for CPU {
    fn default() -> Self {
        CPU {
            registers: Registers::default(),
            memory: vec![0; 0x10000],
        }
    }
}

fn combine_u8_to_u16(high_byte: u8, low_byte: u8) -> u16 {
    return ((high_byte as u16) << 8) | low_byte as u16;
}

#[test]
fn test_combine_u8_to_u16() {
    assert_eq!(257, combine_u8_to_u16(0x1, 0x1))
}

#[test]
fn test_mov_into_b() {
    let mut cpu: CPU = CPU::default_test_values();
    cpu.memory[combine_u8_to_u16(5, 6) as usize] = 10;
    cpu.execute_opcode(0x40);
    assert_eq!(1, cpu.registers.b);
    cpu.execute_opcode(0x41);
    assert_eq!(2, cpu.registers.b);
    cpu.execute_opcode(0x42);
    assert_eq!(3, cpu.registers.b);
    cpu.execute_opcode(0x43);
    assert_eq!(4, cpu.registers.b);
    cpu.execute_opcode(0x44);
    assert_eq!(5, cpu.registers.b);
    cpu.execute_opcode(0x45);
    assert_eq!(6, cpu.registers.b);
    cpu.execute_opcode(0x46);
    assert_eq!(10, cpu.registers.b);
    cpu.execute_opcode(0x47);
    assert_eq!(7, cpu.registers.b);
}

#[test]
fn test_mov_into_c() {
    let mut cpu: CPU = CPU::default_test_values();
    cpu.memory[combine_u8_to_u16(5, 6) as usize] = 10;
    cpu.execute_opcode(0x48);
    assert_eq!(1, cpu.registers.c);
    cpu.execute_opcode(0x49);
    assert_eq!(1, cpu.registers.c);
    cpu.execute_opcode(0x4a);
    assert_eq!(3, cpu.registers.c);
    cpu.execute_opcode(0x4b);
    assert_eq!(4, cpu.registers.c);
    cpu.execute_opcode(0x4c);
    assert_eq!(5, cpu.registers.c);
    cpu.execute_opcode(0x4d);
    assert_eq!(6, cpu.registers.c);
    cpu.execute_opcode(0x4e);
    assert_eq!(10, cpu.registers.c);
    cpu.execute_opcode(0x4f);
    assert_eq!(7, cpu.registers.c);
}

#[test]
fn test_mov_into_d() {
    let mut cpu: CPU = CPU::default_test_values();
    cpu.memory[combine_u8_to_u16(5, 6) as usize] = 10;
    cpu.execute_opcode(0x50);
    assert_eq!(1, cpu.registers.d);
    cpu.execute_opcode(0x51);
    assert_eq!(2, cpu.registers.d);
    cpu.execute_opcode(0x52);
    assert_eq!(2, cpu.registers.d);
    cpu.execute_opcode(0x53);
    assert_eq!(4, cpu.registers.d);
    cpu.execute_opcode(0x54);
    assert_eq!(5, cpu.registers.d);
    cpu.execute_opcode(0x55);
    assert_eq!(6, cpu.registers.d);
    cpu.execute_opcode(0x56);
    assert_eq!(10, cpu.registers.d);
    cpu.execute_opcode(0x57);
    assert_eq!(7, cpu.registers.d);
}

#[test]
fn test_mov_into_e() {
    let mut cpu: CPU = CPU::default_test_values();
    cpu.memory[combine_u8_to_u16(5, 6) as usize] = 10;
    cpu.execute_opcode(0x58);
    assert_eq!(1, cpu.registers.e);
    cpu.execute_opcode(0x59);
    assert_eq!(2, cpu.registers.e);
    cpu.execute_opcode(0x5a);
    assert_eq!(3, cpu.registers.e);
    cpu.execute_opcode(0x5b);
    assert_eq!(3, cpu.registers.e);
    cpu.execute_opcode(0x5c);
    assert_eq!(5, cpu.registers.e);
    cpu.execute_opcode(0x5d);
    assert_eq!(6, cpu.registers.e);
    cpu.execute_opcode(0x5e);
    assert_eq!(10, cpu.registers.e);
    cpu.execute_opcode(0x5f);
    assert_eq!(7, cpu.registers.e);
}

#[test]
fn test_mov_into_h() {
    let mut cpu: CPU = CPU::default_test_values();
    cpu.memory[combine_u8_to_u16(6, 6) as usize] = 10;
    cpu.execute_opcode(0x60);
    assert_eq!(1, cpu.registers.h);
    cpu.execute_opcode(0x61);
    assert_eq!(2, cpu.registers.h);
    cpu.execute_opcode(0x62);
    assert_eq!(3, cpu.registers.h);
    cpu.execute_opcode(0x63);
    assert_eq!(4, cpu.registers.h);
    cpu.execute_opcode(0x64);
    assert_eq!(4, cpu.registers.h);
    cpu.execute_opcode(0x65);
    assert_eq!(6, cpu.registers.h);
    cpu.execute_opcode(0x66);
    assert_eq!(10, cpu.registers.h);
    cpu.execute_opcode(0x67);
    assert_eq!(7, cpu.registers.h);
}

#[test]
fn test_mov_into_l() {
    let mut cpu: CPU = CPU::default_test_values();
    cpu.memory[combine_u8_to_u16(5, 5) as usize] = 10;
    cpu.execute_opcode(0x68);
    assert_eq!(1, cpu.registers.l);
    cpu.execute_opcode(0x69);
    assert_eq!(2, cpu.registers.l);
    cpu.execute_opcode(0x6a);
    assert_eq!(3, cpu.registers.l);
    cpu.execute_opcode(0x6b);
    assert_eq!(4, cpu.registers.l);
    cpu.execute_opcode(0x6c);
    assert_eq!(5, cpu.registers.l);
    cpu.execute_opcode(0x6d);
    assert_eq!(5, cpu.registers.l);
    cpu.execute_opcode(0x6e);
    assert_eq!(10, cpu.registers.l);
    cpu.execute_opcode(0x6f);
    assert_eq!(7, cpu.registers.l);
}

#[test]
fn test_mov_into_m() {
    let mut cpu: CPU = CPU::default_test_values();
    cpu.memory[combine_u8_to_u16(5, 6) as usize] = 10;
    cpu.execute_opcode(0x70);
    assert_eq!(
        1,
        cpu.memory[combine_u8_to_u16(cpu.registers.h, cpu.registers.l) as usize]
    );
    cpu.execute_opcode(0x71);
    assert_eq!(
        2,
        cpu.memory[combine_u8_to_u16(cpu.registers.h, cpu.registers.l) as usize]
    );
    cpu.execute_opcode(0x72);
    assert_eq!(
        3,
        cpu.memory[combine_u8_to_u16(cpu.registers.h, cpu.registers.l) as usize]
    );
    cpu.execute_opcode(0x73);
    assert_eq!(
        4,
        cpu.memory[combine_u8_to_u16(cpu.registers.h, cpu.registers.l) as usize]
    );
    cpu.execute_opcode(0x74);
    assert_eq!(
        5,
        cpu.memory[combine_u8_to_u16(cpu.registers.h, cpu.registers.l) as usize]
    );
    cpu.execute_opcode(0x75);
    assert_eq!(
        6,
        cpu.memory[combine_u8_to_u16(cpu.registers.h, cpu.registers.l) as usize]
    );
    cpu.execute_opcode(0x77);
    assert_eq!(
        7,
        cpu.memory[combine_u8_to_u16(cpu.registers.h, cpu.registers.l) as usize]
    );
}

#[test]
fn test_mov_into_a() {
    let mut cpu: CPU = CPU::default_test_values();
    cpu.memory[combine_u8_to_u16(5, 6) as usize] = 10;
    cpu.execute_opcode(0x78);
    assert_eq!(1, cpu.registers.a);
    cpu.execute_opcode(0x79);
    assert_eq!(2, cpu.registers.a);
    cpu.execute_opcode(0x7a);
    assert_eq!(3, cpu.registers.a);
    cpu.execute_opcode(0x7b);
    assert_eq!(4, cpu.registers.a);
    cpu.execute_opcode(0x7c);
    assert_eq!(5, cpu.registers.a);
    cpu.execute_opcode(0x7d);
    assert_eq!(6, cpu.registers.a);
    cpu.execute_opcode(0x7e);
    assert_eq!(10, cpu.registers.a);
    cpu.execute_opcode(0x7f);
    assert_eq!(10, cpu.registers.a);
}
