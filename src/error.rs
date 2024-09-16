use std::{error::Error, fmt::Display};

#[derive(Debug)]
enum PhaseError {
    GenerationTape,
    OutofBounds
}

#[derive(Debug)]
pub struct GenerationError{
    msg: String,
    phase: PhaseError
}

impl GenerationError{
    pub fn new(msg: String, phase: PhaseError) -> Self{ Self{msg, phase} }
    pub fn out_of_bounds<S: AsRef<str>>(msg: S) -> Self{ Self::new(msg.as_ref().to_string(), PhaseError::OutofBounds) }
    pub fn generation_tape<S: AsRef<str>>(msg: S) -> Self{ Self::new(msg.as_ref().to_string(), PhaseError::GenerationTape) }
}

impl Display for GenerationError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} : {}", self.phase, self.msg)
    }
}

impl Error for GenerationError{}


