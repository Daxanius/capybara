/*
    Made by Daxanius

    Executes instructions mapped by function pointers in an array
*/

use super::cpu::{self, CPU};

// Currently an invalid instruction
pub fn proc_none(cpu: CPU) {
    panic!("Invalid instruction!");
}

// These are function pointers that point
// to a coresponding function for the instruction
pub static processors: [fn(CPU);1] = [
    proc_none,
];