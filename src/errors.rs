use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum LMCErrors {
    #[error("No instructions were given to LMC")]
    NoInstructionsGiven,
    #[error("Too many instructions given! Not enough memory.")]
    TooManyInstructionsGiven,
    #[error("The program halted")]
    Halt,
    #[error("Please enter an integer")]
    InvalidInput(#[from] std::num::ParseIntError),
    #[error("Error reading from standard input")]
    IOError(#[from] std::io::Error),
    #[error("Tried to access memory out of bounds.")]
    MemoryOutOfBounds,
}

pub const TEXT_PREVIEW_LENGTH: usize = 10;

#[derive(Error, Debug)]
pub enum SasmErrors {
    #[error("The file given does not exist")]
    FileDoesNotExist(#[from] io::Error),
    #[error("An unfamiliar token was encountered when lexing! Next characters: '{0}' ({1} more)")]
    LexemeNotRecognised(String, u64),
    #[error("The instruction lexed was not recognised: found unknown token '{0}'")]
    InstructionNotRecognised(String),
    #[error("No argument was passed to an instruction")]
    NoArgumentPassedToOp,
    #[error("An argument was passed to an instruction that did not require one")]
    UnexpectedArgPassedToOp,
    #[error("Encountered an end of file while parsing")]
    UnexpectedEOF,
    #[error("No argument, newline, or comment followed an instruction")]
    NoArgNewlineOrComment,
    #[error("The instruction code read was not recognised; got {0}, limit is {1}")]
    InstructionCodeNotRecognised(i64, u64),
    #[error("The label used ({0}) was not defined")]
    UseOfUndeclaredLabel(String),
}
