/*
    Made by Daxanius

    See https://gbdev.io/pandocs/CPU_Instruction_Set.html for more
    information on the instruction set
*/

pub struct Instruction {
    pub instruction_type: In,
    pub addr_mode: AddrMode,
    pub reg1: Option<Reg>,
    pub reg2: Option<Reg>,
    pub condition: Option<Cond>,
    pub param: Option<u8>,
}

// See https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html for
// more information on the instructions
// I have a feeling that these could be binary mapped, I could be wrong though
// The result could be an instruction pointer, not an entire instruciton,
// making memory usage less prominent
pub fn instruction_by_opcode(opcode: &u8) -> Option<Instruction> {
    match opcode {
        0x00 => Some(Instruction { instruction_type: In::NOP, addr_mode: AddrMode::IMP, reg1: None, reg2: None, condition: None, param: None }),
        0x05 => Some(Instruction { instruction_type: In::DEC, addr_mode: AddrMode::R, reg1: Some(Reg::B), reg2: None, condition: None, param: None }),
        0x0E => Some(Instruction { instruction_type: In::LD, addr_mode: AddrMode::R_D8, reg1: Some(Reg::C), reg2: None, condition: None, param: None}),
        0xAF => Some(Instruction { instruction_type: In::XOR, addr_mode: AddrMode::R, reg1: Some(Reg::A), reg2: None, condition: None, param: None}),
        0xC3 => Some(Instruction { instruction_type: In::JP, addr_mode: AddrMode::D16, reg1: None, reg2: None, condition: None, param: None }),
        _ => None
    }
}

// NOTE TO SELF:
// An addressing mode basically describes
// how to read the next n bytes in the stream
// (n also being described by the addressing mode)
// This to handle opcodes which require different types
// or amounts (bytes) of input for each instruction
#[derive(Debug)]
pub enum AddrMode {
    IMP,
    R_D16,
    R_R,
    MR_R,
    R,
    R_D8,
    R_MR,
    R_HLI,
    R_HLD,
    HLI_R,
    HLD_R,
    R_A8,
    A8_R,
    HL_SPR,
    D16,
    D8,
    D16_R,
    MR_D8,
    MR,
    A16_R,
    R_A16
}

#[derive(Debug)]
pub enum Reg {
    NONE,
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
    SP,
    PC
}

// Instruction type
#[derive(Debug)]
pub enum In {
    NONE,
    NOP,
    LD,
    INC,
    DEC,
    RLCA,
    ADD,
    RRCA,
    STOP,
    RLA,
    JR,
    RRA,
    DAA,
    CPL,
    SCF,
    CCF,
    HALT,
    ADC,
    SUB,
    SBC,
    AND,
    XOR,
    OR,
    CP,
    POP,
    JP,
    PUSH,
    RET,
    CB,
    CALL,
    RETI,
    LDH,
    JPHL,
    DI,
    EI,
    RST,
    ERR,
    //CB instructions...
    RLC, 
    RRC,
    RL, 
    RR,
    SLA, 
    SRA,
    SWAP, 
    SRL,
    BIT, 
    RES, 
    SET
}

#[derive(Debug)]
pub enum Cond {
    NONE, 
    NZ, 
    Z, 
    NC, 
    C
}