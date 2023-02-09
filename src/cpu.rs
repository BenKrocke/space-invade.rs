#![allow(dead_code)] // TODO: Remove later

#[derive(Copy, Clone)]
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

impl Default for Registers {
    fn default() -> Registers {
        Registers {
            a: u8::MIN,
            f: u8::MIN,
            b: u8::MIN,
            c: u8::MAX,
            d: u8::MIN,
            e: u8::MIN,
            h: u8::MIN,
            l: u8::MIN,
            sp: u16::MIN,
            pc: u16::MIN,
        }
    }
}

pub struct CPU {
    registers: Registers,
}

impl Default for CPU {
    fn default() -> CPU {
        CPU {
            registers: Registers::default(),
        }
    }
}

pub fn execute_opcode(mut cpu: CPU, opcode: u8) -> CPU {
    match opcode {
        0x40 => {
            cpu.registers.b = cpu.registers.b;
        }
        0x41 => {
            cpu.registers.b = cpu.registers.c;
        }
        _default => {
            panic!("Opcode not implemented");
        }
    }

    cpu
}

#[test]
fn test_mov_b_c() {
    let cpu = CPU::default();
    assert_eq!(u8::MAX, execute_opcode(cpu, 0x41).registers.b);
}
