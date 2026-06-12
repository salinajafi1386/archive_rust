use std::io;
use std::io::{Read, Write};

pub fn xor_crypt(data: &mut [u8], password: &str) -> Vec<u8> {
    let password = password.as_bytes();
    let mut result = Vec::new();
    for (i, &byte) in data.iter().enumerate() {
        result.push(byte ^ password[i % password.len()]);
    }
    result
}

pub fn xor_stream<R: Read, W: Write>(
    reader: R,
    writer: W,
    password: String,
) -> Result<(), io::Error> {
    todo!()
}
