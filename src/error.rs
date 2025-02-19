use nom::error::{ContextError, ParseError};

#[derive(Debug)]
pub struct CustomError {
    message: String,
}

impl CustomError {
    pub fn new(input: &[u8], ctx: &'static str, kind: nom::error::ErrorKind) -> Self {
        CustomError::add_context(input, ctx, CustomError::from_error_kind(input, kind))
    }
}

impl ContextError<&[u8]> for CustomError {
    fn add_context(_input: &[u8], _ctx: &'static str, other: Self) -> Self {
        let message = format!("{}\"{}\":\t{:?}\n", other.message, _ctx, _input);
        println!("{}", message);
        CustomError { message }
    }
}

impl ParseError<&[u8]> for CustomError {
    fn from_error_kind(input: &[u8], kind: nom::error::ErrorKind) -> Self {
        let message = format!("{:?} \"{:?}\"", input, kind);
        println!("{}", message);
        CustomError { message }
    }
    fn append(input: &[u8], kind: nom::error::ErrorKind, other: Self) -> Self {
        let message = format!("{:?}{:?} \"{:?}\"", input, other.message, kind);
        println!("{}", message);
        CustomError { message }
    }
}
