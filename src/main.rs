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

    let mut ctx = core::board::Board { running: false, paused: false, ticks: 0, cart: None };

    ctx.insert_cart(&file);
    ctx.start();

    // Capybara will use piston for rendering.
    let mut window: PistonWindow = WindowSettings::new("Capybara", [640, 480]).exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
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