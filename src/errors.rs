use std::error::Error;
use std::fmt;

#[derive(Debug, RustcEncodable)]
pub struct LinterWarning {
    pub message: &'static str,
    pub start: usize,
    pub end: usize
}

impl fmt::Display for LinterWarning {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LinterWarning { message, ref start, ref end } => {
                f.write_str(
                    format!("Warning: {} ({}, {})", message, start, end).as_str()
                )
            },
        }
    }
}

impl Error for LinterWarning {
    #[allow(unused_variables)]
    fn description(&self) -> &str {
        match *self {
            LinterWarning { message, ref start, ref end } => { message },
        }
    }
}
