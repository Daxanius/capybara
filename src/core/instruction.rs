/*
    Made by Daxanius

    See https://gbdev.io/pandocs/CPU_Instruction_Set.html for more
    information on the instruction set
*/

#[derive(Debug)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub addr_mode: AddressingMode,
    pub reg1: Option<Register>,
    pub reg2: Option<Register>,
    pub condition: Option<Condition>,
    pub param: Option<u8>,
}

impl Instruction {
    // See https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html for
    // more information on the instructions
    // I have a feeling that these could be binary mapped, I could be wrong though
    // The result could be an instruction pointer, not an entire instruction,
    // making memory usage less prominent
    pub fn from_opcode(opcode: &u8) -> Option<Instruction> {
        match opcode {
            0x00 => Some(Instruction { instruction_type: InstructionType::NOP, addr_mode: AddressingMode::IMP, reg1: None, reg2: None, condition: None, param: None }),
            0x05 => Some(Instruction { instruction_type: InstructionType::DEC, addr_mode: AddressingMode::R, reg1: Some(Register::B), reg2: None, condition: None, param: None }),
            0x0E => Some(Instruction { instruction_type: InstructionType::LD, addr_mode: AddressingMode::R_D8, reg1: Some(Register::C), reg2: None, condition: None, param: None}),
            0xAF => Some(Instruction { instruction_type: InstructionType::XOR, addr_mode: AddressingMode::R, reg1: Some(Register::A), reg2: None, condition: None, param: None}),
            0xC3 => Some(Instruction { instruction_type: InstructionType::JP, addr_mode: AddressingMode::D16, reg1: None, reg2: None, condition: None, param: None }),
            0xF3 => Some(Instruction { instruction_type: InstructionType::DI, addr_mode: AddressingMode::IMP, reg1: None, reg2: None, condition: None, param: None}),
            _ => None
        }
    }
}

// NOTE TO SELF:
// An addressing mode basically describes
// how to read the next n bytes in the stream
// (n also being described by the addressing mode)
// This to handle opcodes which require different types
// or amounts (bytes) of input for each instruction
// Fun fact: this is a gameboy specific thing :-)
#[derive(Debug)]
pub enum AddressingMode {
    /// No input bytes
    IMP,

    /// A register and a 16 bit number
    R_D16,

    /// The second register
    R_R,

    /// Load a register into a register memory region (MR)
    MR_R,

    /// A register
    R,

    /// A register and an 8 bit number
    R_D8,

    /// Read from a register memory region (MR)
    R_MR,

    /// Load address of the HL register and increment
    R_HLI,

    /// Load address of the HL register and decrement
    R_HLD,

    /// Move a value into the HL address and increment it
    HLI_R,

    /// Move a value into the HL address and decrement it
    HLD_R,

    /// Move an A8 into a register
    R_A8,

    /// Move from a register into an A8
    A8_R,

    /// Load stackpointer into HL incremented by R8
    HL_SPR,

    /// A 16 bit number
    D16,

    /// Move from a register into D8
    D8,

    /// Moving a register into a 16 bit address
    D16_R,

    /// Loading a D8 into a memory address
    MR_D8,

    /// Load reg1 into memory address
    MR,

    /// Move A16 to a register
    A16_R,

    /// Move A16 into a register
    R_A16
}

// Registers, see https://gbdev.io/pandocs/CPU_Registers_and_Flags.html for more information
#[derive(Debug)]
pub enum Register {
    NONE,
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,

    /// Accumulator & Flags
    AF,

    BC,
    DE,
    HL,

    /// Stack pointer
    SP,

    /// Program counter
    PC
}

// Instruction type, see https://gbdev.io/pandocs/CPU_Instruction_Set.html for more information
#[derive(Debug)]
pub enum InstructionType {
    /// No instruction
    NONE,

    /// No operation
    NOP,

    /// Load
    LD,

    /// Increment
    INC,

    /// Decrement
    DEC,

    /// Rotate A left
    RLCA,

    /// Add
    ADD,

    /// Rotate A right
    RRCA,

    /// Low power standby mode (VERY low power)
    STOP,

    /// Rotate A left trough carry
    RLA,

    /// Relative jump
    JR,

    /// Rotate A trough carry
    RRA,

    /// decimal adjust A
    DAA,

    /// A = A xor FF
    CPL,

    /// cy=1
    SCF,

    /// cy=cy xor 1
    CCF,

    /// Halt until interrupt occurs (low power)
    HALT,

    ADC,

    /// Subtract
    SUB,
    SBC,

    /// And
    AND,

    /// Xor
    XOR,

    /// Or
    OR,

    /// Compare
    CP,

    /// Pop from stack
    POP,

    /// Jump
    JP,

    /// Push to stack
    PUSH,

    /// Return
    RET,
    CB,

    /// Call to
    CALL,

    /// Return and enable interrupts (IME=1)
    RETI,
    LDH,
    JPHL,

    /// Disable interrupts, IME = 0
    DI,

    /// Enable interrupts, IME=1
    EI,

    /// Call to
    RST,
    ERR,

    //CB instructions...
    /// Rotate left
    RLC,

    /// Rotate right
    RRC,

    /// Rotate left trough carry
    RL,

    /// Rotate right trough carry
    RR,

    /// Shift left arithmetic (b0=0)
    SLA,

    /// Shift right arithmetic (b7=b7)
    SRA,

    /// Exchange low/hi-nibble
    SWAP,

    /// Shift right logical (b7=0)
    SRL,

    /// Test bit
    BIT,

    /// Reset bit
    RES,

    /// Set bit
    SET
}

// Condition
#[derive(Debug)]
pub enum Condition {
    /// No condition
    NONE, 

    /// Not equal to zero
    NZ,

    /// Equal to zero
    Z,

    /// Not C flag
    NC,

    /// C flag
    C
}