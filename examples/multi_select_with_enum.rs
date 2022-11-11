use eprompt::*;
use std::fmt::Display;

#[derive(Debug)]
enum Choices {
    Opt1(i32),
    Opt2(&'static str),
    Opt3,
}

impl Display for Choices {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            Self::Opt1(x) => write!(fmt, "{} of Option 1", x),
            Self::Opt2(x) => write!(fmt, "Option 2 is {}", x),
            Self::Opt3 => write!(fmt, "Mystery option 3"),
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let x = multi_select(
        "Make a selection",
        &[
            Choices::Opt1(69),
            Choices::Opt2("A Brand new pony"),
            Choices::Opt3,
        ],
    )?
    .map(|x| x.1)
    .collect::<Vec<_>>();
    println!("Lets do it! {:?}", x);

    let x = select("How much fun is this library out of 5?", &[1, 2, 3, 4, 5])?.1;
    println!("Excelent: {:?}", x);

    let x: i32 = input("Enter your age")?;
    println!("Wow you're already {}!", x);
    Ok(())
}
