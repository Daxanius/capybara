/*
    Made by Daxanius

    Some common functions and features
*/

use std::{thread, time, fmt::Write};

// Not a great implementation for a delay, but it works
pub fn delay(ms: u64) {
    thread::sleep(time::Duration::from_millis(ms));
}

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