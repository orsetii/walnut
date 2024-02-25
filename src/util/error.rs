use core::error::Error;

use crate::mem::allocator::AllocationError;



#[derive(Debug)]
pub struct WalnutError {
    details: &'static str,
}

impl WalnutError {
    pub fn new(msg: &'static str) -> WalnutError {
        WalnutError { details: msg }
    }
}

impl core::fmt::Display for WalnutError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for WalnutError {
    fn description(&self) -> &str {
        self.details
    }
}


impl From<AllocationError> for WalnutError {
    fn from(value: AllocationError) -> Self {
        // this is likely violating the geneva convention
        // TODO Make this cleaner!
        Self::new(format_args!("Allocation Error: {}", value.description()).as_str().unwrap())
    }
}

