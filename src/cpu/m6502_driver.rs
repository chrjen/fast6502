use super::{instruction::Opcode, M6502};
use crate::memory::Memory;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ClockCycle {
    Cycle1,
    Cycle2,
    Cycle3,
    Cycle4,
    Cycle5,
    Cycle6,
    Cycle7,
}

struct CycleTicker {
    cycle: ClockCycle,
    op: Opcode,
    addr: u16,
    state: M6502,
}

pub fn tick_cycle<M: Memory>(cpu: &mut CycleTicker, mem: &mut M) {
    match (cpu.op, cpu.cycle) {
        // Add with carry instructions.
        (Opcode::ADC_imm, _) => todo!("Not implemented OP!"),
        (Opcode::ADC_zpg, _) => todo!("Not implemented OP!"),
        (Opcode::ADC_zpX, _) => todo!("Not implemented OP!"),
        (Opcode::ADC_abs, _) => todo!("Not implemented OP!"),
        (Opcode::ADC_abX, _) => todo!("Not implemented OP!"),
        (Opcode::ADC_abY, _) => todo!("Not implemented OP!"),
        (Opcode::ADC_inX, _) => todo!("Not implemented OP!"),
        (Opcode::ADC_inY, _) => todo!("Not implemented OP!"),

        // Logical AND instruction.
        (Opcode::AND_imm, _) => todo!("Not implemented OP!"),
        (Opcode::AND_zpg, _) => todo!("Not implemented OP!"),
        (Opcode::AND_zpX, _) => todo!("Not implemented OP!"),
        (Opcode::AND_abs, _) => todo!("Not implemented OP!"),
        (Opcode::AND_abX, _) => todo!("Not implemented OP!"),
        (Opcode::AND_abY, _) => todo!("Not implemented OP!"),
        (Opcode::AND_inX, _) => todo!("Not implemented OP!"),
        (Opcode::AND_inY, _) => todo!("Not implemented OP!"),

        // Arithmetic shift left instructions.
        (Opcode::ASL_acc, _) => todo!("Not implemented OP!"),
        (Opcode::ASL_zpg, _) => todo!("Not implemented OP!"),
        (Opcode::ASL_zpX, _) => todo!("Not implemented OP!"),
        (Opcode::ASL_abs, _) => todo!("Not implemented OP!"),
        (Opcode::ASL_abX, _) => todo!("Not implemented OP!"),

        // Branch instructions.
        (Opcode::BCC_rel, _) => todo!("Not implemented OP!"),
        (Opcode::BCS_rel, _) => todo!("Not implemented OP!"),
        (Opcode::BEQ_rel, _) => todo!("Not implemented OP!"),
        (Opcode::BMI_rel, _) => todo!("Not implemented OP!"),
        (Opcode::BNE_rel, _) => todo!("Not implemented OP!"),
        (Opcode::BPL_rel, _) => todo!("Not implemented OP!"),
        (Opcode::BVC_rel, _) => todo!("Not implemented OP!"),
        (Opcode::BVS_rel, _) => todo!("Not implemented OP!"),

        // Bit test instruction.
        (Opcode::BIT_zpg, _) => todo!("Not implemented OP!"),
        (Opcode::BIT_abs, _) => todo!("Not implemented OP!"),

        // Break instruction.
        (Opcode::BRK_imp, _) => todo!("Not implemented OP!"),

        // Clear status flag instructions.
        (Opcode::CLC_imp, _) => todo!("Not implemented OP!"),
        (Opcode::CLD_imp, _) => todo!("Not implemented OP!"),
        (Opcode::CLI_imp, _) => todo!("Not implemented OP!"),
        (Opcode::CLV_imp, _) => todo!("Not implemented OP!"),

        // Compare instructions.
        (Opcode::CMP_imm, _) => todo!("Not implemented OP!"),
        (Opcode::CMP_zpg, _) => todo!("Not implemented OP!"),
        (Opcode::CMP_zpX, _) => todo!("Not implemented OP!"),
        (Opcode::CMP_abs, _) => todo!("Not implemented OP!"),
        (Opcode::CMP_abX, _) => todo!("Not implemented OP!"),
        (Opcode::CMP_abY, _) => todo!("Not implemented OP!"),
        (Opcode::CMP_inX, _) => todo!("Not implemented OP!"),
        (Opcode::CMP_inY, _) => todo!("Not implemented OP!"),

        (Opcode::CPX_imm, _) => todo!("Not implemented OP!"),
        (Opcode::CPX_zpg, _) => todo!("Not implemented OP!"),
        (Opcode::CPX_abs, _) => todo!("Not implemented OP!"),

        (Opcode::CPY_imm, _) => todo!("Not implemented OP!"),
        (Opcode::CPY_zpg, _) => todo!("Not implemented OP!"),
        (Opcode::CPY_abs, _) => todo!("Not implemented OP!"),

        // Decrement instructions.
        (Opcode::DEC_zpg, _) => todo!("Not implemented OP!"),
        (Opcode::DEC_zpX, _) => todo!("Not implemented OP!"),
        (Opcode::DEC_abs, _) => todo!("Not implemented OP!"),
        (Opcode::DEC_abX, _) => todo!("Not implemented OP!"),

        (Opcode::DEX_imp, _) => todo!("Not implemented OP!"),
        (Opcode::DEY_imp, _) => todo!("Not implemented OP!"),

        // Logical XOR instructions.
        (Opcode::EOR_imm, _) => todo!("Not implemented OP!"),
        (Opcode::EOR_zpg, _) => todo!("Not implemented OP!"),
        (Opcode::EOR_zpX, _) => todo!("Not implemented OP!"),
        (Opcode::EOR_abs, _) => todo!("Not implemented OP!"),
        (Opcode::EOR_abX, _) => todo!("Not implemented OP!"),
        (Opcode::EOR_abY, _) => todo!("Not implemented OP!"),
        (Opcode::EOR_inX, _) => todo!("Not implemented OP!"),
        (Opcode::EOR_inY, _) => todo!("Not implemented OP!"),

        // Increment instructions.
        (Opcode::INC_zpg, _) => todo!("Not implemented OP!"),
        (Opcode::INC_zpX, _) => todo!("Not implemented OP!"),
        (Opcode::INC_abs, _) => todo!("Not implemented OP!"),
        (Opcode::INC_abX, _) => todo!("Not implemented OP!"),

        (Opcode::INX_imp, _) => todo!("Not implemented OP!"),
        (Opcode::INY_imp, _) => todo!("Not implemented OP!"),

        // Jump instructions.
        (Opcode::JMP_abs, _) => todo!("Not implemented OP!"),
        (Opcode::JMP_ind, _) => todo!("Not implemented OP!"),

        (Opcode::JSR_abs, _) => todo!("Not implemented OP!"),

        // Load register A instructions.
        (Opcode::LDA_imm, _) => todo!("Not implemented OP!"),
        (Opcode::LDA_zpg, _) => todo!("Not implemented OP!"),
        (Opcode::LDA_zpX, _) => todo!("Not implemented OP!"),
        (Opcode::LDA_abs, _) => todo!("Not implemented OP!"),
        (Opcode::LDA_abX, _) => todo!("Not implemented OP!"),
        (Opcode::LDA_abY, _) => todo!("Not implemented OP!"),
        (Opcode::LDA_inX, _) => todo!("Not implemented OP!"),
        (Opcode::LDA_inY, _) => todo!("Not implemented OP!"),

        // Load register X instructions.
        (Opcode::LDX_imm, _) => todo!("Not implemented OP!"),
        (Opcode::LDX_zpg, _) => todo!("Not implemented OP!"),
        (Opcode::LDX_zpY, _) => todo!("Not implemented OP!"),
        (Opcode::LDX_abs, _) => todo!("Not implemented OP!"),
        (Opcode::LDX_abY, _) => todo!("Not implemented OP!"),

        // Load register Y instructions.
        (Opcode::LDY_imm, _) => todo!("Not implemented OP!"),
        (Opcode::LDY_zpg, _) => todo!("Not implemented OP!"),
        (Opcode::LDY_zpX, _) => todo!("Not implemented OP!"),
        (Opcode::LDY_abs, _) => todo!("Not implemented OP!"),
        (Opcode::LDY_abX, _) => todo!("Not implemented OP!"),

        // Logical shift right instructions.
        (Opcode::LSR_acc, _) => todo!("Not implemented OP!"),
        (Opcode::LSR_zpg, _) => todo!("Not implemented OP!"),
        (Opcode::LSR_zpX, _) => todo!("Not implemented OP!"),
        (Opcode::LSR_abs, _) => todo!("Not implemented OP!"),
        (Opcode::LSR_abX, _) => todo!("Not implemented OP!"),

        // No operation instruction.
        (Opcode::NOP_imp, _) => todo!("Not implemented OP!"),

        // Logical OR instructions.
        (Opcode::ORA_imm, _) => todo!("Not implemented OP!"),
        (Opcode::ORA_zpg, _) => todo!("Not implemented OP!"),
        (Opcode::ORA_zpX, _) => todo!("Not implemented OP!"),
        (Opcode::ORA_abs, _) => todo!("Not implemented OP!"),
        (Opcode::ORA_abX, _) => todo!("Not implemented OP!"),
        (Opcode::ORA_abY, _) => todo!("Not implemented OP!"),
        (Opcode::ORA_inX, _) => todo!("Not implemented OP!"),
        (Opcode::ORA_inY, _) => todo!("Not implemented OP!"),

        // Push register instructions.
        (Opcode::PHA_imp, _) => todo!("Not implemented OP!"),
        (Opcode::PHP_imp, _) => todo!("Not implemented OP!"),

        // Pull register instructions.
        (Opcode::PLA_imp, _) => todo!("Not implemented OP!"),
        (Opcode::PLP_imp, _) => todo!("Not implemented OP!"),

        // Rotate left instructions.
        (Opcode::ROL_acc, _) => todo!("Not implemented OP!"),
        (Opcode::ROL_zpg, _) => todo!("Not implemented OP!"),
        (Opcode::ROL_zpX, _) => todo!("Not implemented OP!"),
        (Opcode::ROL_abs, _) => todo!("Not implemented OP!"),
        (Opcode::ROL_abX, _) => todo!("Not implemented OP!"),

        // Rotate right instructions.
        (Opcode::ROR_acc, _) => todo!("Not implemented OP!"),
        (Opcode::ROR_zpg, _) => todo!("Not implemented OP!"),
        (Opcode::ROR_zpX, _) => todo!("Not implemented OP!"),
        (Opcode::ROR_abs, _) => todo!("Not implemented OP!"),
        (Opcode::ROR_abX, _) => todo!("Not implemented OP!"),

        // Return instructions.
        (Opcode::RTI_imp, _) => todo!("Not implemented OP!"),
        (Opcode::RTS_imp, _) => todo!("Not implemented OP!"),

        // Subtract with carry instructions.
        (Opcode::SBC_imm, _) => todo!("Not implemented OP!"),
        (Opcode::SBC_zpg, _) => todo!("Not implemented OP!"),
        (Opcode::SBC_zpX, _) => todo!("Not implemented OP!"),
        (Opcode::SBC_abs, _) => todo!("Not implemented OP!"),
        (Opcode::SBC_abX, _) => todo!("Not implemented OP!"),
        (Opcode::SBC_abY, _) => todo!("Not implemented OP!"),
        (Opcode::SBC_inX, _) => todo!("Not implemented OP!"),
        (Opcode::SBC_inY, _) => todo!("Not implemented OP!"),

        // Set status flag instructions.
        (Opcode::SEC_imp, _) => todo!("Not implemented OP!"),
        (Opcode::SED_imp, _) => todo!("Not implemented OP!"),
        (Opcode::SEI_imp, _) => todo!("Not implemented OP!"),

        // Store register A instructions.
        (Opcode::STA_zpg, _) => todo!("Not implemented OP!"),
        (Opcode::STA_zpX, _) => todo!("Not implemented OP!"),
        (Opcode::STA_abs, _) => todo!("Not implemented OP!"),
        (Opcode::STA_abX, _) => todo!("Not implemented OP!"),
        (Opcode::STA_abY, _) => todo!("Not implemented OP!"),
        (Opcode::STA_inX, _) => todo!("Not implemented OP!"),
        (Opcode::STA_inY, _) => todo!("Not implemented OP!"),

        // Store register X instructions.
        (Opcode::STX_zpg, _) => todo!("Not implemented OP!"),
        (Opcode::STX_zpY, _) => todo!("Not implemented OP!"),
        (Opcode::STX_abs, _) => todo!("Not implemented OP!"),

        // Store register Y instructions.
        (Opcode::STY_zpg, _) => todo!("Not implemented OP!"),
        (Opcode::STY_zpX, _) => todo!("Not implemented OP!"),
        (Opcode::STY_abs, _) => todo!("Not implemented OP!"),

        // Transfer instructions.
        (Opcode::TAX_imp, _) => todo!("Not implemented OP!"),
        (Opcode::TAY_imp, _) => todo!("Not implemented OP!"),
        (Opcode::TSX_imp, _) => todo!("Not implemented OP!"),
        (Opcode::TXA_imp, _) => todo!("Not implemented OP!"),
        (Opcode::TXS_imp, _) => todo!("Not implemented OP!"),
        (Opcode::TYA_imp, _) => todo!("Not implemented OP!"),

        (_, _) => panic!("Unknown instruction!"),
    }
}
