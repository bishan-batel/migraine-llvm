use std::error::Error;

use pest::{Parser, iterators::Pairs};

#[derive(Parser)]
#[grammar = "migraine.pest"]
pub struct MigraineParse;

pub fn parse(file: &str) -> Result<Pairs<Rule>, Box<dyn Error>> {
    let pairs = MigraineParse::parse(Rule::module, file)?;
    Ok(pairs)
}
