use std::error::Error;
use crate::semantic;

type MainFunction = fn() -> i32;
pub struct CodeGen {}

impl CodeGen {
    pub fn new(input: &str) -> Self {
        
        Self {}
    }

    pub fn parse(&self) -> Result<(), Box<dyn Error>> {
        let res = semantic::parse(include_str!("../test.migraine"))?;
        Ok(())
    }
}
