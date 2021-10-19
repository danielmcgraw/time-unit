use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct UnrecognizedUnitError {}

impl Error for UnrecognizedUnitError {}

impl Display for UnrecognizedUnitError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "UnrecognizedUnitError")
    }
}
