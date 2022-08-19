/*
    Made by Daxanius

    All core submodules are defined here
    this file includes some common utility stuff
*/

// Submodules

pub mod cartridge;
pub mod cpu;
pub mod timer;
pub mod ppu;
pub mod board;
pub mod instruction;
pub mod cpu_proc;

pub mod util {
    use std::fmt::Write;

    // Macros
    /// Bit macro to get the nth bit
    macro_rules! bit {
        ($a:expr, $n:expr) => {
            ($a & (1 << $n)) != 0
        };
    }

    /// Wait n milliseconds before continueing thread execution
    macro_rules! wait {
        ($ms:expr) => {
            std::thread::sleep(std::time::Duration::from_millis($ms));
        };
    }

    // This is a trick applied to make macros available to other
    // modules
    pub(crate) use {
        bit,
        wait
    };

    // Functions

    // Specifically implemented for the title
    pub fn bytes_to_title(bytes: [u8; 16]) -> String {
        let mut title: String = String::from("");

        for byte in bytes {
            let result = title.write_char(byte as char);

            match result {
                Ok(_) => continue,
                Err(error) => panic!("Problem writing char: {:?}", error),
            };
        }

        title
    }

    /* 
        SLICING 

        Converts a slice to an array of given length
        TODO: this should be macro'd
    */

    pub fn unslice_2(value: &[u8]) -> [u8; 2] {
        value.try_into().expect("Slice with incorrect length")
    }

    pub fn unslice_4(value: &[u8]) -> [u8; 4] {
        value.try_into().expect("Slice with incorrect length")
    }

    pub fn unslice_16 (value: &[u8]) -> [u8; 16] {
        value.try_into().expect("Slice with incorrect length")
    }

    pub fn unslice_48 (value: &[u8]) -> [u8; 0x30] {
        value.try_into().expect("Slice with incorrect length")
    }

    /*
        CONVERTING BYTE ARRAYS

        Converts byte arrays to it's coresponding value
    */

    pub fn as_u16_be(array: &[u8; 2]) -> u16 {
        array[0] as u16 | ((array[1] as u16) << 8)
    }
}