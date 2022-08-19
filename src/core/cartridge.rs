/*
    Made by Daxanius

    Simulates read and write functions for
    game cartridges (also referred to as ROMs)
*/

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use super::util;

// See https://gbdev.io/pandocs/The_Cartridge_Header.html for more information
// Not all header fields have been implemented (yet)
pub struct RomHeader {
    pub entry: [u8; 4],
    pub logo: [u8; 0x30],

    pub title: String,
    pub new_lic_code: u16,
    pub sgb_flag: u8,
    pub cart_type: u8,
    pub rom_size: u8,
    pub ram_size: u8,
    pub dest_code: u8,
    pub lic_code: u8,
    pub version: u8,
    pub checksum: u8,
    pub global_checksum: u16
}

pub struct Cartridge {
    pub filename: String,
    pub rom_size: u64,
    rom_data: Vec<u8>,
    pub header: RomHeader,
}

impl Cartridge {
    pub fn cart_lic_name(&self) -> &'static str {
        if self.header.new_lic_code <= 0xA4 {
            return lic_code(&self.header.lic_code);
        }
    
        return "UNKNOWN";
    }

    pub fn cart_type_name(&self) -> &'static str {
        if self.header.cart_type <= 0x22 {
            return ROM_TYPES[self.header.cart_type as usize];
        }
    
        return "UNKNOWN";
    }

    pub fn read(&self, address: &usize) -> u8 {
        // For now ROM only

        return self.rom_data[*address];
    }

    pub fn write(address: u16, value: u8) {
        // For now ROM only

        unimplemented!();
    }

    // Loads a cartridge from a ROM file
    pub fn load_rom(file: &String) -> Cartridge {
        // Create path to the ROM
        let path = Path::new(file);
        let display = path.display();

        // Open the ROM in read-only mode, returns `io::Result<File>`
        let mut filestream = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        // Read the file contents into a stream of bytes
        let mut data: Vec<u8> = Vec::new();
        match filestream.read_to_end(&mut data) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => print!("Succesfully read raw data from \"{}\"\n", file),
        }

        // Manual casting was required...
        // See https://gbdev.io/pandocs/The_Cartridge_Header.html for more information
        let head = RomHeader {
            entry: util::unslice_4(&data[0x100..0x104]),
            logo: util::unslice_48(&data[0x104..0x134]), // Yep, it's the legendary Nintendo logo
            title: util::bytes_to_title(util::unslice_16(&data[0x134..0x144])),
            new_lic_code: util::as_u16_be(&util::unslice_2(&data[0x144..0x146])),
            sgb_flag: data[0x146],
            cart_type: data[0x147],
            rom_size: 32 << data[0x148],
            ram_size: data[0x140],
            dest_code: data[0x14A],
            lic_code: data[0x14B],
            version: data[0x14C],
            checksum: data[0x14D],
            global_checksum: util::as_u16_be(&util::unslice_2(&data[0x14E..0x150])),
        };

        // Return the cartridge
        return Cartridge {
            filename: file.clone(),
            rom_size: filestream.metadata().unwrap().len(), // TODO: replace this by filestream.stream_len() when this feature releases
            rom_data: data,
            header: head,
        }
    }
}

const ROM_TYPES: [&'static str; 35] = [
    "ROM ONLY",
    "MBC1",
    "MBC1+RAM",
    "MBC1+RAM+BATTERY",
    "0x04 ???",
    "MBC2",
    "MBC2+BATTERY",
    "0x07 ???",
    "ROM+RAM 1",
    "ROM+RAM+BATTERY 1",
    "0x0A ???",
    "MMM01",
    "MMM01+RAM",
    "MMM01+RAM+BATTERY",
    "0x0E ???",
    "MBC3+TIMER+BATTERY",
    "MBC3+TIMER+RAM+BATTERY 2",
    "MBC3",
    "MBC3+RAM 2",
    "MBC3+RAM+BATTERY 2",
    "0x14 ???",
    "0x15 ???",
    "0x16 ???",
    "0x17 ???",
    "0x18 ???",
    "MBC5",
    "MBC5+RAM",
    "MBC5+RAM+BATTERY",
    "MBC5+RUMBLE",
    "MBC5+RUMBLE+RAM",
    "MBC5+RUMBLE+RAM+BATTERY",
    "0x1F ???",
    "MBC6",
    "0x21 ???",
    "MBC7+SENSOR+RUMBLE+RAM+BATTERY",
];

// A quick and simple way of quickly getting a matching LIC code
pub fn lic_code(code: &u8) -> &'static str {
    match code {
        0x00 => "None",
        0x01 => "Nintendo R&D1",
        0x08 => "Capcom",
        0x13 => "Electronic Arts",
        0x18 => "Hudson Soft",
        0x19 => "b-ai",
        0x20 => "kss",
        0x22 => "pow",
        0x24 => "PCM Complete",
        0x25 => "san-x",
        0x28 => "Kemco Japan",
        0x29 => "seta",
        0x30 => "Viacom",
        0x31 => "Nintendo",
        0x32 => "Bandai",
        0x33 => "Ocean/Acclaim",
        0x34 => "Konami",
        0x35 => "Hector",
        0x37 => "Taito",
        0x38 => "Hudson",
        0x39 => "Banpresto",
        0x41 => "Ubi Soft",
        0x42 => "Atlus",
        0x44 => "Malibu",
        0x46 => "angel",
        0x47 => "Bullet-Proof",
        0x49 => "irem",
        0x50 => "Absolute",
        0x51 => "Acclaim",
        0x52 => "Activision",
        0x53 => "American sammy",
        0x54 => "Konami",
        0x55 => "Hi tech entertainment",
        0x56 => "LJN",
        0x57 => "Matchbox",
        0x58 => "Mattel",
        0x59 => "Milton Bradley",
        0x60 => "Titus",
        0x61 => "Virgin",
        0x64 => "LucasArts",
        0x67 => "Ocean",
        0x69 => "Electronic Arts",
        0x70 => "Infogrames",
        0x71 => "Interplay",
        0x72 => "Broderbund",
        0x73 => "sculptured",
        0x75 => "sci",
        0x78 => "THQ",
        0x79 => "Accolade",
        0x80 => "misawa",
        0x83 => "lozc",
        0x86 => "Tokuma Shoten Intermedia",
        0x87 => "Tsukuda Original",
        0x91 => "Chunsoft",
        0x92 => "Video system",
        0x93 => "Ocean/Acclaim",
        0x95 => "Varie",
        0x96 => "Yonezawa/s’pal",
        0x97 => "Kaneko",
        0x99 => "Pack in soft",
        0xA4 => "Konami (Yu-Gi-Oh!)",
        _ => "OTHER",
    }
}