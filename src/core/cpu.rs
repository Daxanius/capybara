/*
    Made by Daxanius

    Simulates the CPU by executing gameboy instructions
*/

use super::instruction::*;
use super::board::Board;
use super::util;

// Simple CPU registers
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    af: usize,
    bc: usize,
    de: usize,
    hl: usize,
    sp: usize,
    pc: usize,
}

pub struct CPU {
    regs: Registers,
    fetched_data: u16,
    mem_dest: u16,
    dest_is_mem: bool,
    cur_opcode: u8,

    cur_inst: Instruction,

    halted: bool,
    stepping: bool,

    // Are interrupts enabled?
    int_enaled: bool,
}

impl CPU {
    // This init could be improved upon
    // The way we handle CPU and bus communication is rather bad
    pub fn init() -> CPU {
        print!("Initialising CPU\n");

        let in_nop;
        match Instruction::from_opcode(&0) {
            Some(instruction) => in_nop = instruction,
            None => panic!("Instruction not found!"),
        }

        // Initialise with empty registers
        // IRL these values would be somewhat 
        // random at start, because technical stuff.
        // Maybe that would be interesting to implement
        // artificially?
        let regs = Registers { 
            a: 0x01, 
            b: 0, 
            c: 0, 
            d: 0,
            e: 0, 
            f: 0, 
            h: 0, 
            l: 0, 
            af: 0, 
            bc: 0, 
            de: 0, 
            hl: 0, 
            sp: 0,

            // The main program entry point starts at 255 (0x100)
            pc: 0x100 
        };

        // Create and return the CPU
        CPU { 
            regs: regs, 
            fetched_data: 0, 
            mem_dest: 0, 
            dest_is_mem: false, 
            cur_opcode: 0, 
            cur_inst: in_nop, 
            halted: false, 
            stepping: true,
            int_enaled: true,
        }
    }

    pub fn flag_z(&self) -> bool {
        util::bit!(self.regs.f, 7)
    }

    pub fn flag_c(&self) -> bool {
        util::bit!(self.regs.f, 4)
    }

    /// Fetch the next instruction and increment the program counter
    fn fetch_instruction(&mut self, board: &Board) {
        self.cur_opcode = board.bus_read(&self.regs.pc);
        self.regs.pc += 1;
        
        // Fetch the instruction at the opcode
        match Instruction::from_opcode(&self.cur_opcode) {
            Some(instruction) => self.cur_inst = instruction,
            None => panic!("Instruction not found for opcode '0x{:01X}'!", self.cur_opcode),
        }
    }

    // Read from a register
    // TODO: fix these pointer references
    pub fn read_reg(&self, register: &Register) -> u16 {
        match register {
            Register::A => self.regs.a as u16,
            Register::F => self.regs.f as u16,
            Register::B => self.regs.b as u16,
            Register::C => self.regs.c as u16,
            Register::D => self.regs.d as u16,
            Register::E => self.regs.e as u16,
            Register::H => self.regs.h as u16,
            Register::L => self.regs.l as u16,

            // These are supposed to be pointers...
            Register::AF => util::reverse!(self.regs.a) as u16,
            Register::BC => util::reverse!(self.regs.b) as u16,
            Register::DE => util::reverse!(self.regs.d) as u16,
            Register::HL => util::reverse!(self.regs.h) as u16,

            Register::PC => self.regs.pc as u16,
            Register::SP => self.regs.sp as u16,

            _ => 0
        }
    }

    // Set a value of a register
    // Why do we take the extra step to do this?
    pub fn set_reg(&mut self, register: &Register, value: &u16) {
        match register {
            Register::A => self.regs.a = (value & 0xFF) as u8,
            Register::F => self.regs.f = (value & 0xFF) as u8,
            Register::B => self.regs.b = (value & 0xFF) as u8,
            Register::C => self.regs.c = (value & 0xFF) as u8,
            Register::D => self.regs.d = (value & 0xFF) as u8,
            Register::E => self.regs.e = (value & 0xFF) as u8,
            Register::H => self.regs.h = (value & 0xFF) as u8,
            Register::L => self.regs.l = (value & 0xFF) as u8,

            _ => ()
        }
    }

    /// Returns true if the current instruction condition passes
    pub fn condition(&self) -> bool {
        match &self.cur_inst.condition {
            // Perhaps this should panic?
            None => true,
            Some (condition) => {
                match condition {
                    Condition::NONE => true,
                    Condition::C => self.flag_c(),
                    Condition::NC => !self.flag_c(),
                    Condition::Z => self.flag_z(),
                    Condition::NZ => !self.flag_z(),
                }
            }
        }
    }

    // Like zoinks Scoob! What is this implementation?
    pub fn set_flags(&mut self, z: i8, n: i8, h: i8, c: i8) {
        if z != -1 {
            util::bit_set!(self.regs.f, 7, z!=0);
        }

        if n != -1 {
            util::bit_set!(self.regs.f, 6, n!=0);
        }

        if h != -1 {
            util::bit_set!(self.regs.f, 5, h!=0);
        }

        if c != -1 {
            util::bit_set!(self.regs.c, 4, c!=0);
        }
    }

    fn execute(&mut self) {
        // Note to self: the program counter here does not contain
        // the location of the current instruction, as it has already incremented past the instruction
        // and it's data, thus, this just displays the current position of the program counter
        // after fetching the instruction
        print!("Executing instruction: {:?}   PC: {:04x}\n", self.cur_inst.instruction_type, self.regs.pc);

        match &self.cur_inst.instruction_type {
            InstructionType::NONE => {
                panic!("Invalid instruction!");
            },

            InstructionType::NOP => {
                
            }

            InstructionType::LD => {

            },

            InstructionType::DI => {
                // Very difficult, no?
                self.int_enaled = false;
            },

            InstructionType::JP => {
                if self.condition() {
                    self.regs.pc = self.fetched_data as usize;
                    emu_cycles(1);
                }
            },

            InstructionType::XOR => {
                self.regs.a = self.regs.a ^ (self.fetched_data as u8 & 0xFF);
                self.set_flags((self.regs.a == 0) as i8, 0, 0, 0);
            },

            _ => panic!("Instruction not implemented: {:?}!", &self.cur_inst),
        }
    }
    
    pub fn step(&mut self, board: &mut Board) -> bool {
        // This could and should probably be moved to main
        if board.paused {
            util::wait!(10);
            return true;
        }

        if self.halted {
            return false;
        }

        self.fetch_instruction(board);
        self.fetch_data(board);
        self.execute();
        board.ticks += 1;

        return true;
    }
}

/// Emulate clock cycles
pub fn emu_cycles(n: u8) {
        
}

// Big code block, what a pain
// TODO: find a way to implement this more elegantly
impl CPU {
    // Reading data according to the addressing mode
    // telling us how we need to read the next n bytes
    // see ./instructions.rs for more information
    fn fetch_data(&mut self, board: &Board) {
        self.mem_dest = 0;
        self.dest_is_mem = false;

        match &self.cur_inst.addr_mode {
            // Nothing needs to be read for IMP (implied)
            AddressingMode::IMP => return,

            // Address mode Register
            AddressingMode::R => {
                match &self.cur_inst.reg1 {
                    Some(x) => self.fetched_data = self.read_reg(&x),
                    None => panic!("No register present!"),
                }
            }

            AddressingMode::R_R => {
                match &self.cur_inst.reg2 {
                    Some(x) => self.fetched_data = self.read_reg(&x),
                    None => panic!("No register present!"),
                }
            }

            // 8 bit from rom
            AddressingMode::R_D8 => {
                board.bus_read(&self.regs.pc);
                emu_cycles(1);
                self.regs.pc += 1;
            }

            // 16 bit from rom
            AddressingMode::D16 => {
                // Making a 16 bit value by getting a low and a high value and ORing them together with 
                // hi shifted over by 8
                let lo: u16 = board.bus_read(&self.regs.pc) as u16;
                emu_cycles(1); 

                let hi: u16 = board.bus_read(&(self.regs.pc +1)) as u16;
                emu_cycles(1);

                self.fetched_data = lo | (hi << 8);
                self.regs.pc += 2;
            }

            // The same as D16, might remove later
            AddressingMode::R_D16 => {
                let lo: u16 = board.bus_read(&self.regs.pc) as u16;
                emu_cycles(1); 

                let hi: u16 = board.bus_read(&(self.regs.pc +1)) as u16;
                emu_cycles(1);

                self.fetched_data = lo | (hi << 8);
                self.regs.pc += 2;
            }

            AddressingMode::MR_R => {
                match &self.cur_inst.reg2 {
                    Some(x) => self.fetched_data = self.read_reg(&x),
                    None => panic!("No register present!"),
                }

                match &self.cur_inst.reg1 {
                    Some(x) => self.mem_dest = self.read_reg(&x),
                    None => panic!("No register present!"),
                }

                self.dest_is_mem = true;

                if matches!(self.cur_inst.reg1.as_ref().unwrap(), Register::C) {
                    self.mem_dest |= 0xFF00;
                }
            }

            AddressingMode::R_MR => {
                let addr: usize;
                match &self.cur_inst.reg2 {
                    Some(x) => addr = self.read_reg(&x) as usize,
                    None => panic!("No register present!"),
                }

                if matches!(self.cur_inst.reg1.as_ref().unwrap(), Register::C) {
                    self.mem_dest |= 0xFF00;
                }

                self.fetched_data = board.bus_read(&addr) as u16;
                emu_cycles(1);
            }

            AddressingMode::R_HLI => {
                match &self.cur_inst.reg2 {
                    Some(x) => self.fetched_data = board.bus_read(&(self.read_reg(&x) as usize)) as u16,
                    None => panic!("No register present!"),
                }

                emu_cycles(1);
                self.set_reg(&Register::HL, &(self.read_reg(&Register::HL) +1));
            }

            AddressingMode::R_HLD => {
                match &self.cur_inst.reg2 {
                    Some(x) => self.fetched_data = board.bus_read(&(self.read_reg(&x) as usize)) as u16,
                    None => panic!("No register present!"),
                }

                emu_cycles(1);
                self.set_reg(&Register::HL, &(self.read_reg(&Register::HL) -1));
            }

            AddressingMode::HLI_R => {
                match &self.cur_inst.reg2 {
                    Some(x) => self.fetched_data = self.read_reg(&x),
                    None => panic!("No register present!"),
                }

                match &self.cur_inst.reg1 {
                    Some(x) => self.mem_dest = self.read_reg(&x),
                    None => panic!("No register present!"),
                }

                self.dest_is_mem = true;

                self.set_reg(&Register::HL, &(self.read_reg(&Register::HL) +1));
            }

            AddressingMode::HLD_R => {
                match &self.cur_inst.reg2 {
                    Some(x) => self.fetched_data = self.read_reg(&x),
                    None => panic!("No register present!"),
                }

                match &self.cur_inst.reg1 {
                    Some(x) => self.mem_dest = self.read_reg(&x),
                    None => panic!("No register present!"),
                }

                self.dest_is_mem = true;

                self.set_reg(&Register::HL, &(self.read_reg(&Register::HL) -1));
            }

            AddressingMode::A8_R => {
                self.mem_dest = board.bus_read(&self.regs.pc) as u16 | 0xFF00;
                self.dest_is_mem = true;
                emu_cycles(1);
                self.regs.pc += 1;
            }

            // The latter 3 are practically the same,
            // perhaps they could be turned into
            // 1 addressing mode?
            AddressingMode::R_A8 => {
                self.fetched_data = board.bus_read(&self.regs.pc) as u16;
                emu_cycles(1);
                self.regs.pc += 1;
            }

            AddressingMode::HL_SPR => {
                self.fetched_data = board.bus_read(&self.regs.pc) as u16;
                emu_cycles(1);
                self.regs.pc += 1;
            }

            AddressingMode::D8 => {
                self.fetched_data = board.bus_read(&self.regs.pc) as u16;
                emu_cycles(1);
                self.regs.pc += 1;
            }

            AddressingMode::D16_R => {
                let lo: u16 = board.bus_read(&self.regs.pc) as u16;
                emu_cycles(1); 

                let hi: u16 = board.bus_read(&(self.regs.pc +1)) as u16;
                emu_cycles(1);

                self.mem_dest = lo | (hi << 8);
                self.dest_is_mem = true;
                self.regs.pc += 2;

                match &self.cur_inst.reg2 {
                    Some(x) => self.fetched_data = self.read_reg(&x),
                    None => panic!("No register present!"),
                }
            }

            AddressingMode::MR_D8 => {
                self.fetched_data = board.bus_read(&self.regs.pc) as u16;
                emu_cycles(1);
                self.regs.pc += 1;

                match &self.cur_inst.reg1 {
                    Some(x) => self.mem_dest = self.read_reg(&x),
                    None => panic!("No register present!"),
                }

                self.dest_is_mem = true;
            }

            AddressingMode::MR => {
                match &self.cur_inst.reg1 {
                    Some(x) => self.mem_dest = self.read_reg(&x),
                    None => panic!("No register present!"),
                }

                self.dest_is_mem = true;

                match &self.cur_inst.reg1 {
                    Some(x) => self.fetched_data = board.bus_read(&(self.read_reg(&x) as usize)) as u16,
                    None => panic!("No register present!"),
                }

                emu_cycles(1);
            }

            // Same as D16_R
            AddressingMode::A16_R => {
                let lo: u16 = board.bus_read(&self.regs.pc) as u16;
                emu_cycles(1); 

                let hi: u16 = board.bus_read(&(self.regs.pc +1)) as u16;
                emu_cycles(1);

                self.mem_dest = lo | (hi << 8);
                self.dest_is_mem = true;
                self.regs.pc += 2;

                match &self.cur_inst.reg2 {
                    Some(x) => self.fetched_data = self.read_reg(&x),
                    None => panic!("No register present!"),
                }
            }

            AddressingMode::R_A16 => {
                let lo: u16 = board.bus_read(&self.regs.pc) as u16;
                emu_cycles(1); 

                let hi: u16 = board.bus_read(&(self.regs.pc +1)) as u16;
                emu_cycles(1);

                let addr: u16 = lo | (hi << 8);
                self.regs.pc += 2;
                self.fetched_data = board.bus_read(&(addr as usize)) as u16;
                emu_cycles(1);
            }

            mode => panic!("Unknown addressing mode '({:?})'!", mode),
        }
    }
}