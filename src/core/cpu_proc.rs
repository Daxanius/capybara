/*
    Made by Daxanius

    Executes instructions mapped by function pointers in an array
*/

use super::cpu::{self, CPU};

// Currently an invalid instruction
pub fn proc_none(cpu: CPU) {
    panic!("Invalid instruction!");
}

pub fn proc_ld(cpu: CPU) {
    // TODO...
}

// These are function pointers that point
// to a coresponding function for the instruction
pub static PROCESSORS: [fn(CPU);2] = [
    proc_none,
    proc_ld,
];