/*
    Made by Daxanius

    A simple low level gameboy emulator

    https://gbdev.io/
*/

mod core;

fn main() {
    let mut ctx = core::board::Board { running: false, paused: false, ticks: 0, cart: None };

    ctx.insert_cart("D:\\ROMs\\cpu_instrs.gb");
    ctx.start();
}