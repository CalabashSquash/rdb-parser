use std::fs;

use nom::{
    bytes::complete::{tag, take},
    error::{ContextError, Error, ErrorKind, ParseError},
    IResult,
};
use variable_length::{parse_length_encoded_string, LengthEncoded};

mod error;
mod opcodes;
mod variable_length;

use crate::error::CustomError;
use crate::opcodes::OpCodes;

// Using the following as a specification:
// https://rdb.fnordig.de/file_format.html#auxiliary-fields

#[derive(Debug)]
struct Rdb {
    version_number: String,
}

#[derive(Debug)]
struct Auxiliary {
    redis_ver: Option<String>,
    redis_bits: Option<u32>,
    ctime: Option<u32>,
    used_mem: Option<u32>,
    aof_base: Option<u32>,
}

pub fn read_file(filename: String) -> Result<(), CustomError> {
    let redis = "REDIS".as_bytes();
    println!("{redis:x?}");
    let bytes: &[u8] = &fs::read(filename).expect("read failed");
    println!("bytes: {bytes:X?}");
    let s = String::from_utf8_lossy(&bytes);
    println!("s: {s:?}");

    // let (remaining, parsed) = parse_redis_magic_string(bytes).expect("Parsing failed");
    // println!("{remaining:#?} +++++ {parsed:#?}");
    // let (remaining, parsed) = parse_version_number(remaining).expect("Parsing failed");
    // println!("{remaining:#?} +++++ {parsed:#?}");
    let (remaining, parsed) = parse(bytes).expect("Parsing failed");
    println!("Parsed: {parsed:#?}");
    Ok(())
}

fn parse(input: &[u8]) -> IResult<&[u8], Rdb, CustomError> {
    let (remaining, _) = parse_redis_magic_string(input)?;
    let (remaining, version_number) = parse_version_number(remaining)?;
    let (remaining, (aux)) = parse_opcodes(remaining)?;

    Ok((remaining, Rdb { version_number }))
}

fn parse_opcodes(input: &[u8]) -> IResult<&[u8], (Auxiliary), CustomError> {
    let mut aux = Auxiliary {
        redis_ver: None,
        redis_bits: None,
        ctime: None,
        used_mem: None,
        aof_base: None
    };
    let mut remaining = input;

    let mut temp = 123;

    for i in 0..6 {
        println!("");
        println!("=========");
        println!("remaining at start of loop: {remaining:X?}");
        let (new_remaining, next_opcode) = take(1usize)(remaining)?;
        remaining = new_remaining;

        println!("Next opcode: {next_opcode:X?}");
        if next_opcode.len() > 1 {
            panic!("TODO");
        }
        match next_opcode[0].try_into() {
            Ok(OpCodes::AUX) => {
                println!("PRE Remaining: {:X?}, aux: {:?}", remaining, aux);
                let (new_remaining, new_aux) = parse_aux(remaining, aux)?;
                remaining = new_remaining;
                aux = new_aux;
                temp = 456;
                println!("Remaining: {:X?}, aux: {:?}", remaining, aux);
            }
            Ok(OpCodes::EOF) => {
                todo!("EOF")
            }
            Ok(OpCodes::EXPIRETIME) => {
                todo!("EXPIRETIME")
            }
            Ok(OpCodes::EXPIRETIMEMS) => {
                todo!("EXPIRETIMEMS")
            }
            Ok(OpCodes::RESIZEDB) => {
                todo!("RESIZEDB")
            }
            Ok(OpCodes::SELECTDB) => {
                todo!("SELECTDB")
            }
            _ => {
                todo!("Opcode not handled");
            }
        };
        println!("Next aux: {aux:#?}");
    };
    Ok((remaining, aux))
}

fn parse_redis_magic_string(input: &[u8]) -> IResult<&[u8], &[u8], CustomError> {
    tag("REDIS")(input)
}

fn parse_version_number(input: &[u8]) -> IResult<&[u8], String, CustomError> {
    let (remaining, parsed) = take(4usize)(input)?;
    let version_number = String::from_utf8(parsed.to_vec())
        .map_err(|_| nom::Err::Failure(CustomError::new(
            input,
            "Failed to convert the version number to String",
            ErrorKind::TakeUntil
        )))?;

    return Ok((remaining, version_number));
}

fn parse_aux(input: &[u8], aux: Auxiliary) -> IResult<&[u8], Auxiliary, CustomError> {
    let (remaining, aux_key) = parse_length_encoded_string(input)?;
    let aux_key = match aux_key {
        LengthEncoded::Number(_) => {
            return Err(
                nom::Err::Failure(CustomError::new(
                    input,
                    "auxiliary key should never be number",
                    ErrorKind::TakeTill1,
                ))
            )
        },
        LengthEncoded::String(s) => s
    };
    let (remaining, aux_value) = parse_length_encoded_string(remaining)?;
    println!("aux_key: {aux_key}");

    match aux_key.as_str() {
        "redis-ver" => {
            let aux_value = match aux_value {
                LengthEncoded::Number(_) => {
                    return Err(
                        nom::Err::Failure(CustomError::new(
                            input,
                            "redis-ver should never be number",
                            ErrorKind::TakeTill1,
                        ))
                    )
                },
                LengthEncoded::String(s) => s
            };
            return Ok((
                remaining,
                Auxiliary {
                    redis_ver: Some(aux_value),
                    ..aux
                },
            ))
        }
        "redis-bits" => {
            let aux_value = match aux_value {
                LengthEncoded::Number(n) => n,
                LengthEncoded::String(_) => {
                    return Err(
                        nom::Err::Failure(CustomError::new(
                            input,
                            "redis-bits should never be String",
                            ErrorKind::TakeTill1,
                        ))
                    )
                }
            };
            return Ok((
                remaining,
                Auxiliary {
                    redis_bits: Some(aux_value),
                    ..aux
                },
            ));
        }
        "ctime" => {
            let aux_value = match aux_value {
                LengthEncoded::Number(n) => n,
                LengthEncoded::String(_) => {
                    return Err(
                        nom::Err::Failure(CustomError::new(
                            input,
                            "ctime should never be String",
                            ErrorKind::TakeTill1,
                        ))
                    )
                }
            };
            return Ok((
                remaining,
                Auxiliary {
                    ctime: Some(aux_value),
                    ..aux
                },
            ));
        }
        "used-mem" => {
            let aux_value = match aux_value {
                LengthEncoded::Number(n) => n,
                LengthEncoded::String(_) => {
                    return Err(
                        nom::Err::Failure(CustomError::new(
                            input,
                            "ctime should never be String",
                            ErrorKind::TakeTill1,
                        ))
                    )
                }
            };
            return Ok((
                remaining,
                Auxiliary {
                    used_mem: Some(aux_value),
                    ..aux
                },
            ));
        },
        "aof-base" => {
            let aux_value = match aux_value {
                LengthEncoded::Number(n) => n,
                LengthEncoded::String(_) => {
                    return Err(
                        nom::Err::Failure(CustomError::new(
                            input,
                            "ctime should never be String",
                            ErrorKind::TakeTill1,
                        ))
                    )
                }
            };
            return Ok((
                remaining,
                Auxiliary {
                    aof_base: Some(aux_value),
                    ..aux
                },
            ));
        }
        _ => {
            return Err(nom::Err::Failure(CustomError::new(
                input,
                "Invalid or unsupported auxiliary key",
                ErrorKind::TakeUntil,
            )))
        }
    }

    todo!("parse_aux")
    // Ok((
    //     input,
    //     Auxiliary {
    //         redis_ver: None,
    //         redis_bits: None,
    //         ctime: None,
    //         used_mem: None,
    //     },
    // ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        read_file(String::from("test.rdb"));
        // assert_eq!(result, 4);
    }
}
