/*
    Made by Daxanius

    A simple low level gameboy emulator

    https://gbdev.io/
*/

use clap::Parser;
use piston_window::*;

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

    // Cartridge should not be mutable, as it is ROM
    let cartridge = core::cartridge::Cartridge::from_file(&file);

    let mut board = core::board::Board { paused: false, ticks: 0, cart: None };
    let mut cpu = core::cpu::CPU::init();

    board.insert_cart(cartridge);

    // Capybara will use piston for rendering.
    // TODO: make a context which does basically the same thing as this
    let mut window: PistonWindow = WindowSettings::new("Capybara", [640, 480]).exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
        // Stepping trough the CPU cycles
        // We give the cpu the board context
        // on which it can execute instructions
        if !cpu.step(&mut board) {
            print!("CPU halted\n");
            return;
        }

        window.draw_2d(&event, |c, g, device| {
            clear([1.0; 4], g);
            rectangle(
                [1.0, 0.0, 0.0, 1.0], // red
                        [0.0, 0.0, 10.0, 10.0],
                        c.transform,    
                        g
            );
        });
    }
}