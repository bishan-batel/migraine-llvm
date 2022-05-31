use std::error::Error;

use crate::codegen::CodeGen;

#[macro_use]
extern crate pest_derive;
extern crate pest;

mod semantic;
mod codegen;

fn main() -> Result<(), Box<dyn Error>>{
    CodeGen::parse()?;

    Ok(())
}
