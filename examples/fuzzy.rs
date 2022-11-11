use eprompt::*;
use std::io::Result;

fn main() -> Result<()> {
    let a = fuzzy_prompt(&[
        "Choose me!",
        "Number 2 is best!",
        "Fuzzy wuzzy?",
        "Woah how u do dis?",
    ])?;
    dbg!(a);
    //fuzzy(&[])?;
    Ok(())
}
