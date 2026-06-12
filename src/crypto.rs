use std::io;
use std::io::{Read, Write};

// pub fn xor_crypt(data: &mut [u8], password: &str) -> Vec<u8> {
//     let password = password.as_bytes();
//     let mut result = Vec::new();
//     for (i, &byte) in data.iter().enumerate() {
//         result.push(byte ^ password[i % password.len()]);
//     }
//     result
// }

pub fn xor_stream<R: Read, W: Write>(
    mut reader: R,
    mut writer: W,
    password: &str,
) -> Result<(), io::Error> {
    let pass_bytes = password.as_bytes();
    let pass_len = pass_bytes.len();

    if pass_len == 0 {
        std::io::copy(&mut reader, &mut writer)?;
        return Ok(());
    }

    let mut buffer = [0u8; 8192];
    let mut key_index = 0;

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        for i in 0..bytes_read {
            buffer[i] ^= pass_bytes[key_index % pass_len];
            key_index += 1;
        }

        writer.write_all(&buffer[..bytes_read])?;
    }

    Ok(())
}
