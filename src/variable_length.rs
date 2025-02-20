
use nom::{number::Endianness, IResult};

use crate::error::CustomError;

pub enum LengthEncoded {
    Number(u32),
    String(String)
}

pub fn parse_length_encoded_string(input: &[u8], endianness: Endianness) -> IResult<&[u8], LengthEncoded, CustomError> {
    let first_half_nibble = input[0] >> 6;
    let length: u32;
    let start_byte;
    println!("input[0]: {:X?}", input[0]);
    println!("First half nibble: {:X?}", first_half_nibble);
    println!("input: {:X?}", input);
    match first_half_nibble {
        0 => {
            // Next 6 bits represent the length
            length = (input[0] & 0b00111111) as u32;
            start_byte = 1;
        },
        1 => {
            // `(Next 6 bits) | next byte` represents the length
            length = ((input[0] & 0b00111111) as u32) << 8 + (input[1] as u32);
            start_byte = 2;
        },
        2 => {
            // Discard next 6 bits. The next 4 bytes represents the length
            match endianness {
                Endianness::Little => {
                    length = ((input[1] as u32) << 24) | ((input[2] as u32) << 16) | ((input[3] as u32) << 8) | (input[4] as u32);
                },
                Endianness::Big => {
                    length = ((input[4] as u32) << 24) | ((input[3] as u32) << 16) | ((input[2] as u32) << 8) | (input[1] as u32);
                },
                Endianness::Native => {
                    return Err(
                        nom::Err::Failure(CustomError::new(input, "endianness can not be Native", nom::error::ErrorKind::AlphaNumeric))
                    )
                }

            }
            start_byte = 5;
        },
        3 => {
            println!("Special format!");
            // Special format
            let format_code = input[0] & 0b00111111;
            match format_code {
                0 => {
                    // 8bit number
                    length = 1;
                    start_byte = 1;
                    return Ok((&input[(start_byte + length) as usize..], LengthEncoded::Number(input[1] as u32)));
                },
                1 => {
                    // 16 bit number
                    length = 2;
                    start_byte = 1;
                    let num = (input[1] as u32) << 8 + input[2] as u32;
                    return Ok((&input[(start_byte + length) as usize..], LengthEncoded::Number(num)));
                },
                2 => {
                    // 32 bit number
                    length = 4;
                    start_byte = 1;
                    let num;
                    match endianness {
                        Endianness::Little => {
                            num = ((input[1] as u32) << 24) | ((input[2] as u32) << 16) | ((input[3] as u32) << 8) | (input[4] as u32);
                        },
                        Endianness::Big => {
                            num = ((input[4] as u32) << 24) | ((input[3] as u32) << 16) | ((input[2] as u32) << 8) | (input[1] as u32);
                        },
                        Endianness::Native => {
                            return Err(
                                nom::Err::Failure(CustomError::new(input, "endianness can not be Native", nom::error::ErrorKind::AlphaNumeric))
                            )
                        }
                    }
                    return Ok((&input[(start_byte + length) as usize..], LengthEncoded::Number(num)));
                }
                3 => {
                    // Compressed string
                    todo!("Compressed string")
                },
                _ => {
                    todo!("Error handling")
                }
            }
        }
        _ => {
            todo!("Error handling")
        }
    }


    let v = &input[start_byte as usize..(start_byte + length) as usize];
    let remaining = &input[(start_byte + length) as usize..];
    let s = String::from_utf8(v.to_vec()).map_err(|_| nom::Err::Failure(CustomError::new(input, "from_utf8 error while parsing length-encoded string", nom::error::ErrorKind::Tag)))?;

    return Ok((remaining, LengthEncoded::String(s)));
}