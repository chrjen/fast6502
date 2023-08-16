use core::num::Wrapping;

use crate::memory::Memory;

pub struct Mcu6502<M: Memory> {
    /// The accumulator is the processor register used in arithmetic and
    /// logical operations, and push/pop stack operations.
    a: Wrapping<u8>,

    /// X is a processor register used in memory addressing modes and serve
    /// as an index/offset for memory related operations.
    x: Wrapping<u8>,

    /// Y is a processor register used in memory addressing modes and serve
    /// as an index/offset for memory related operations.
    y: Wrapping<u8>,

    /// The stack pointer is the processor register containing the address of
    /// the next free location on the stack. This is the lower 8-bits of the
    /// address with the upper 8-bits always being 0x01.
    sp: Wrapping<u8>,

    /// The program counter is the processor register containing the address of
    /// the next instruction.
    pc: Wrapping<u16>,

    /// Processor status flags change as the results of running various
    /// operations and may be used to modify the behaviour of later instructions.
    flags: u8,

    /// The memory attached to this processor. All memory related operations
    /// will read and write to this memory.
    mem: M,
}

impl<M: Memory> Mcu6502<M> {
    pub fn new(mem: M) -> Mcu6502<M> {
        Mcu6502 {
            a: Wrapping(0),
            x: Wrapping(0),
            y: Wrapping(0),
            sp: Wrapping(0),
            pc: Wrapping(0),
            flags: 0,
            mem,
        }
    }

    pub fn reg_a(&self) -> u8 {
        self.a.0
    }

    pub fn set_reg_a(&mut self, a: u8) {
        self.a = Wrapping(a);
    }

    pub fn reg_x(&self) -> u8 {
        self.x.0
    }

    pub fn set_reg_x(&mut self, x: u8) {
        self.x = Wrapping(x);
    }

    pub fn reg_y(&self) -> u8 {
        self.y.0
    }

    pub fn set_reg_y(&mut self, y: u8) {
        self.y = Wrapping(y);
    }

    pub fn reg_sp(&self) -> u8 {
        self.sp.0
    }

    pub fn set_reg_sp(&mut self, sp: u8) {
        self.sp = Wrapping(sp);
    }

    pub fn reg_pc(&self) -> u16 {
        self.pc.0
    }

    pub fn set_reg_pc(&mut self, pc: u16) {
        self.pc = Wrapping(pc);
    }

    pub fn flags(&self) -> u8 {
        self.flags
    }

    pub fn set_flags(&mut self, flags: u8) {
        self.flags = flags;
    }

    pub fn mem(&self) -> &M {
        &self.mem
    }

    pub fn mem_mut(&mut self) -> &mut M {
        &mut self.mem
    }
}
