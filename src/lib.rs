use std::fs;

use nom::{bytes::complete::{tag, take}, error::{Error, ErrorKind}, IResult};

struct RdbError;

#[derive(Debug)]
struct Rdb {
    version_number: String,
}

pub fn read_file(filename: String) -> Result<(), RdbError> {
    let redis = "REDIS".as_bytes();
    println!("{redis:x?}");
    let bytes: &[u8] = &fs::read(filename).expect("read failed");
    println!("bytes: {bytes:X?}");
    let s = String::from_utf8_lossy(&bytes);
    println!("s: {s:?}");
    
    let (remaining, parsed) = parse_redis_magic_string(bytes).expect("Parsing failed");
    println!("{remaining:#?} +++++ {parsed:#?}");
    let (remaining, parsed) = parse_version_number(remaining).expect("Parsing failed");
    println!("{remaining:#?} +++++ {parsed:#?}");
    let parsed = parse(bytes);
    println!("Parsed: {parsed:#?}");
    Ok(())
}

fn parse(input: &[u8]) -> IResult<&[u8], Rdb> {
    let (remaining, _) = parse_redis_magic_string(input)?;
    let (remaining, version_number) = parse_version_number(remaining)?;


    Ok((remaining, Rdb {
        version_number
    }))
}

fn parse_version_number(input: &[u8]) -> IResult<&[u8], String> {
    let (remaining, parsed) = take(4usize)(input)?;
    let version_number = String::from_utf8(parsed.to_vec()).map_err(|_| nom::Err::Failure(Error::new(input, ErrorKind::Char)))?;

    return Ok((remaining, version_number))
}

fn parse_redis_magic_string(input: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("REDIS")(input)
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
