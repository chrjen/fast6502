/// The internal state registers of the 6502 chip.
#[derive(Debug, Default)]
pub struct M6502 {
    /// The accumulator is the processor register used in arithmetic and
    /// logical operations, and push/pop stack operations.
    pub a: u8,

    /// X is a processor register used in memory addressing modes and serve
    /// as an index/offset for memory related operations.
    pub x: u8,

    /// Y is a processor register used in memory addressing modes and serve
    /// as an index/offset for memory related operations.
    pub y: u8,

    /// The stack pointer is the processor register containing the address of
    /// the next free location on the stack. This is the lower 8-bits of the
    /// address with the upper 8-bits always being 0x01.
    pub sp: u8,

    /// The program counter is the processor register containing the address of
    /// the next instruction.
    pub pc: u16,

    /// Processor status flags change as the results of running various
    /// operations and may be used to modify the behaviour of later instructions.
    ///
    /// The flags are packet in the following order, `nv-bdizc`, with most significant bit
    /// first.
    ///
    /// Each bit can be gotten and set using the corresponding methods `flag_x` and `set_flag_x`.
    /// With `x` replaced with the letter of the flag. The exception is `b` or the break flag
    /// which is only seen when pushing the flags onto the stack.
    ///
    /// ### Flags bits
    ///  - `n` - Negative
    ///  - `v` - Overflow
    ///  - `-` - Unused
    ///  - `b` - Break (only seen on the stack)
    ///  - `d` - Decimal
    ///  - `i` - Interrupt Disable
    ///  - `z` - Zero
    ///  - `c` - Carry
    pub flags: u8,
}

impl M6502 {
    /// Bit location of the "Negative" flag in the status flags register.
    /// Used to indicate that a previous instruction resulted in a negative value
    /// (bit 7 set).
    pub const FLAG_N: u8 = 0x80;
    /// Bit location of the "Overflow" flag in the status flags register.
    /// Used to indicate that the arithmetic instruction resulted in a numeric overflow,
    /// i.e. the resulting signed value was too big to be represented with available bits.
    pub const FLAG_V: u8 = 0x40;

    /// Bit location of the "Break" flag in the status flags register.
    /// Used to indicate whether the flags were pushed onto the stack as a result of an
    /// interrupt or an instruction. Value `0` for interrupt and `1` for instruction.
    pub const FLAG_B: u8 = 0x10;
    /// Bit location of the "Decimal" flag in the status flags register.
    /// Used to toggle CPU into decimal mode where some arithmetic operations would
    /// treat their input as binary-coded decimal instead of regular binary.
    pub const FLAG_D: u8 = 0x08;
    /// Bit location of the "Interrupt Disable" flag in the status flags register.
    /// Used to toggle IRQ interrupts. When set, IRQ interrupts are ignored.
    pub const FLAG_I: u8 = 0x04;
    /// Bit location of the "Zero" flag in the status flags register.
    /// Used to indicate that a previous instruction resulted in the value zero.
    pub const FLAG_Z: u8 = 0x02;
    /// Bit location of the "Carry" flag in the status flags register.
    /// Used to indicate that a carry has occurred. It can be thought of as the value
    /// of the 9th bit had the accumulator been 9 bits instead of 8 bits.
    pub const FLAG_C: u8 = 0x01;

    // Returns a new M6502 initialised with all default values.
    pub fn new() -> M6502 {
        M6502::default()
    }

    /// Returns the current state of the "Negative" flag. See [`Self::FLAG_N`].
    #[inline]
    pub const fn flag_n(&self) -> bool {
        self.flags & Self::FLAG_N != 0
    }

    /// Sets the current state of the "Negative" flag. See [`Self::FLAG_N`].
    #[inline]
    pub fn set_flag_n(&mut self, status: bool) {
        if status {
            self.flags |= Self::FLAG_N;
        } else {
            self.flags &= !Self::FLAG_N;
        }
    }

    /// Returns the current state of the "Overflow" flag. See [`Self::FLAG_V`].
    #[inline]
    pub const fn flag_v(&self) -> bool {
        self.flags & Self::FLAG_V != 0
    }

    /// Sets the current state of the "Overflow" flag. See [`Self::FLAG_V`].
    #[inline]
    pub fn set_flag_v(&mut self, status: bool) {
        if status {
            self.flags |= Self::FLAG_V;
        } else {
            self.flags &= !Self::FLAG_V;
        }
    }

    /// Returns the current state of the "Decimal" flag. See [`Self::FLAG_D`].
    #[inline]
    pub const fn flag_d(&self) -> bool {
        self.flags & Self::FLAG_D != 0
    }

    /// Sets the current state of the "Decimal" flag. See [`Self::FLAG_D`].
    #[inline]
    pub fn set_flag_d(&mut self, status: bool) {
        if status {
            self.flags |= Self::FLAG_D;
        } else {
            self.flags &= !Self::FLAG_D;
        }
    }

    /// Returns the current state of the "Interrupt Disable" flag. See [`Self::FLAG_I`].
    #[inline]
    pub const fn flag_i(&self) -> bool {
        self.flags & Self::FLAG_I != 0
    }

    /// Sets the current state of the "Interrupt Disable" flag. See [`Self::FLAG_I`].
    #[inline]
    pub fn set_flag_i(&mut self, status: bool) {
        if status {
            self.flags |= Self::FLAG_I;
        } else {
            self.flags &= !Self::FLAG_I;
        }
    }

    /// Returns the current state of the "Zero" flag. See [`Self::FLAG_Z`].
    #[inline]
    pub const fn flag_z(&self) -> bool {
        self.flags & Self::FLAG_Z != 0
    }

    /// Sets the current state of the "Zero" flag. See [`Self::FLAG_Z`].
    #[inline]
    pub fn set_flag_z(&mut self, status: bool) {
        if status {
            self.flags |= Self::FLAG_Z;
        } else {
            self.flags &= !Self::FLAG_Z;
        }
    }

    /// Returns the current state of the "Carry" flag. See [`Self::FLAG_C`].
    #[inline]
    pub const fn flag_c(&self) -> bool {
        self.flags & Self::FLAG_C != 0
    }

    /// Sets the current state of the "Carry" flag. See [`Self::FLAG_C`].
    #[inline]
    pub fn set_flag_c(&mut self, status: bool) {
        if status {
            self.flags |= Self::FLAG_C;
        } else {
            self.flags &= !Self::FLAG_C;
        }
    }
}
