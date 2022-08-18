/*
    Made by Daxanius

    A simple low level gameboy emulator

    https://gbdev.io/
*/

use clap::Parser;

mod core;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CapybaraArgs {
    /// ROM file to load as cartridge
    #[clap(short, long, value_parser)]
    file: String,
}

fn main() {
    let args = CapybaraArgs::parse();
    let file = args.file;

    let mut ctx = core::board::Board { running: false, paused: false, ticks: 0, cart: None };

    ctx.insert_cart(&file);
    ctx.start();
}