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
                let address = combine_u8_to_u16(self.registers.h, self.registers.l) as usize;
                self.registers.b = self.memory[address];
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
                let address = combine_u8_to_u16(self.registers.h, self.registers.l) as usize;
                self.registers.c = self.memory[address];
            }
            0x4f => {
                self.registers.c = self.registers.a;
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
