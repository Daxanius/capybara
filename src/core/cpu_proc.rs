/*
    Made by Daxanius

    Executes instructions mapped by function pointers in an array
*/

use super::cpu::CPU;

// Currently an invalid instruction
pub fn proc_none(cpu: CPU) {
    panic!("Invalid instruction!");
}

pub fn proc_ld(cpu: CPU) {
    // TODO...
}

pub fn proc_jp(cpu: CPU) {

}

// These are function pointers that point
// to a coresponding function for the instruction
// will probably use a hashmap for this
pub static PROCESSORS: [fn(CPU);3] = [
    proc_none,
    proc_ld,
    proc_jp,
];