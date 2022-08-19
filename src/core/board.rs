/*
    Made by Daxanius

    Runs the main part of the emulator 
*/

use super::util;
use super::cartridge::Cartridge;
use super::cpu::CPU;
use super::cartridge;
use super::ppu;

/*
    BUS ADDR TABLE

    0x0000 - 0x3FFF : ROM Bank 0
    0x4000 - 0x7FFF : ROM Bank 1 - Switchable
    0x8000 - 0x97FF : CHR RAM
    0x9800 - 0x9BFF : BG Map 1
    0x9C00 - 0x9FFF : BG Map 2
    0xA000 - 0xBFFF : Cartridge RAM
    0xC000 - 0xCFFF : RAM Bank 0
    0xD000 - 0xDFFF : RAM Bank 1-7 - switchable - Color only
    0xE000 - 0xFDFF : Reserved - Echo RAM
    0xFE00 - 0xFE9F : Object Attribute Memory
    0xFEA0 - 0xFEFF : Reserved - Unusable
    0xFF00 - 0xFF7F : I/O Registers
    0xFF80 - 0xFFFE : Zero Page
*/


// The main emulator context, I'm new to Rust and
// it's programming structure, so bare with me here
// I feel like I'm just applying an OOP style of programming to this x.x
pub struct Board {
    // Metadata
    pub paused: bool,
    pub ticks: u64,

    // Components
    pub cart: Option<cartridge::Cartridge>,
}

impl Board {
    pub fn bus_read(&self, address: &usize) -> u8 {
        if address < &0x8000 {
            match &self.cart {
                Some(cart) => return cart.read(address),
                None => panic!("Failed to read address {:02X} from bus: no cartridge or ROM inserted!\n", address)
            }
        }

        unimplemented!();
    }
    
    pub fn bus_write(address: &u16, value: &u8) {
        unimplemented!();
    }

    pub fn insert_cart(&mut self, cartridge: Cartridge) {
        self.cart = Some(cartridge);

        match &self.cart {
            Some(cart) => {
                print!("Cartridge loaded:\n");
                print!("\tTitle    : {}\n", cart.header.title);
                print!("\tVersion  : {}\n", cart.header.version);
                print!("\tType     : {} ({})\n", cart.header.cart_type, cart.cart_type_name());
                print!("\tROM Size : {} KB\n", cart.header.rom_size);
                print!("\tRAM Size : {}\n", cart.header.ram_size);
                print!("\tLIC Code : {} ({})\n", cart.header.lic_code, cart.cart_lic_name());

                // Run a checksum as stated in 
                // https://gbdev.io/pandocs/The_Cartridge_Header.html
                let mut x: u16 = 0;
                for i in 0x0134..0x014C {
                    x = x.wrapping_sub((self.bus_read(&i) as u16).wrapping_sub(1));
                }

                // There is no ternary operator (bool ? "true" : false) in Rust
                let val;
                if (x & 0xFF) != 0 { val = "PASSED"; } else { val = "FAILED"; };

                print!("\tChecksum : {:01X} ({})\n", cart.header.checksum, val);
            }

            None => println!("Failed to load cartrigde"),
        }
    }
}