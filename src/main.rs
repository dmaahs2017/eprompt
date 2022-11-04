use eprompt::*;

fn main() -> Result<()> {
    let x = multi_select(
        "What would you like to do today?",
        &["Eat a cake", "Go to work", "Go on a hike"],
    )?;
    println!("Lets do it! {:?}", x);

    let x = select("How much fun is this library out of 5?", &[1, 2, 3, 4, 5])?;
    println!("Excelent: {:?}", x);

    let x: i32 = input("Enter your age")?;
    println!("Wow you're already {}!", x);
    Ok(())
}
