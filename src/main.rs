use eprompt::*;

fn main() -> Result<()> {
    //let x = multi_select("Select multiple options", &[1, 2, 3])?;
    //println!("Chosen: {:?}", x);

    //let x = select("Select an option", &[4, 5, 6])?;
    //println!("Chosen: {:?}", x);

    let x: i32 = input("Enter your age")?;
    println!("Your age is: {}", x);
    Ok(())
}
