/*
    Made by Daxanius

    Simulates the CPU by executing gameboy instructions
*/

use super::instruction::{Instruction, AddressingMode, Register, InstructionType};
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
            a: 0, 
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
        }
    }

    pub fn flag_z(&self) -> bool {
        util::bit!(self.regs.f, 7)
    }

    pub fn flag_c(&self) -> bool {
        util::bit!(self.regs.f, 4)
    }

    /// Fetch the next instruction and increment the program counter
    pub fn fetch_instruction(&mut self, board: &Board) {
        self.cur_opcode = board.bus_read(&self.regs.pc);
        self.regs.pc += 1;
        
        // Fetch the instruction at the opcode
        match Instruction::from_opcode(&self.cur_opcode) {
            Some(instruction) => self.cur_inst = instruction,
            None => panic!("Instruction not found for opcode '0x{:01X}'!", self.cur_opcode),
        }
    }

    // Reading data according to the addressing mode
    // telling us how we need to read the next n bytes
    // see ./instructions.rs for more information
    pub fn fetch_data(&mut self, board: &Board) {
        self.mem_dest = 0;
        self.dest_is_mem = false;

        match &self.cur_inst.addr_mode {
            // Nothing needs to be read for IMP (implied)
            AddressingMode::IMP => return,

            // Address mode Register
            AddressingMode::R => {
                match &self.cur_inst.reg1 {
                    Some(x) => read_reg(&x),
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

            mode => panic!("Unknown addressing mode '({:?})'!", mode),
        }
    }

    pub fn execute(&self) {
        print!("Not executing yet...\n");

        match &self.cur_inst.instruction_type {
            InstructionType::NONE => {
                panic!("Invalid instruction!");
            },

            InstructionType::LD => {

            },

            InstructionType::JP => {

            },

            _ => panic!("Invalid instruction: {:?}!", &self.cur_inst),
        }
    }
    
    pub fn step(&mut self, board: &Board) -> bool {
        if !self.halted {
            self.fetch_instruction(board);
            self.fetch_data(board);
            self.execute();
        }

        return true;
    }
}

pub fn emu_cycles(n: u8) {
        
}

pub fn read_reg(n: &Register) {
        
}