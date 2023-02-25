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

pub fn execute_opcode(mut cpu: CPU, opcode: u8) -> CPU {
    match opcode {
        0x00 => {}
        0x40 => {
            cpu.registers.b = cpu.registers.b;
        }
        0x41 => {
            cpu.registers.b = cpu.registers.c;
        }
        0x42 => {
            cpu.registers.b = cpu.registers.d;
        }
        0x43 => {
            cpu.registers.b = cpu.registers.e;
        }
        0x44 => {
            cpu.registers.b = cpu.registers.h;
        }
        0x45 => {
            cpu.registers.b = cpu.registers.l;
        }
        0x46 => {
            let address = combine_u8_to_u16(cpu.registers.h, cpu.registers.l) as usize;
            cpu.registers.b = cpu.memory[address];
        }
        0x47 => {
            cpu.registers.b = cpu.registers.a;
        }
        0x48 => {
            cpu.registers.c = cpu.registers.b;
        }
        0x49 => {
            cpu.registers.c = cpu.registers.c;
        }
        0x4a => {
            cpu.registers.c = cpu.registers.d;
        }
        0x4b => {
            cpu.registers.c = cpu.registers.e;
        }
        0x4c => {
            cpu.registers.c = cpu.registers.h;
        }
        0x4d => {
            cpu.registers.c = cpu.registers.l;
        }
        0x4e => {
            let address = combine_u8_to_u16(cpu.registers.h, cpu.registers.l) as usize;
            cpu.registers.c = cpu.memory[address];
        }
        0x4f => {
            cpu.registers.c = cpu.registers.a;
        }
        _default => {
            panic!("Opcode not implemented");
        }
    }

    cpu
}

#[test]
fn test_mov_into_b() {
    let mut cpu: CPU = CPU {
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
    };
    cpu.memory[combine_u8_to_u16(5, 6) as usize] = 10;
    assert_eq!(1, execute_opcode(cpu.clone(), 0x40).registers.b);
    assert_eq!(2, execute_opcode(cpu.clone(), 0x41).registers.b);
    assert_eq!(3, execute_opcode(cpu.clone(), 0x42).registers.b);
    assert_eq!(4, execute_opcode(cpu.clone(), 0x43).registers.b);
    assert_eq!(5, execute_opcode(cpu.clone(), 0x44).registers.b);
    assert_eq!(6, execute_opcode(cpu.clone(), 0x45).registers.b);
    assert_eq!(10, execute_opcode(cpu.clone(), 0x46).registers.b);
    assert_eq!(7, execute_opcode(cpu.clone(), 0x47).registers.b);
}

#[test]
fn test_mov_into_c() {
    let mut cpu = CPU {
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
    };
    cpu.memory[combine_u8_to_u16(5, 6) as usize] = 10;
    assert_eq!(1, execute_opcode(cpu.clone(), 0x48).registers.c);
    assert_eq!(2, execute_opcode(cpu.clone(), 0x49).registers.c);
    assert_eq!(3, execute_opcode(cpu.clone(), 0x4a).registers.c);
    assert_eq!(4, execute_opcode(cpu.clone(), 0x4b).registers.c);
    assert_eq!(5, execute_opcode(cpu.clone(), 0x4c).registers.c);
    assert_eq!(6, execute_opcode(cpu.clone(), 0x4d).registers.c);
    assert_eq!(10, execute_opcode(cpu.clone(), 0x4e).registers.c);
    assert_eq!(7, execute_opcode(cpu.clone(), 0x4f).registers.c);
}
