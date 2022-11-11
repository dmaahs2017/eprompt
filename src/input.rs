use std::{
    io::{stdin, stdout, BufRead, Result, Write},
    str::FromStr,
};

use crossterm::{cursor, execute, queue, style::*, terminal};

/// Prompts the user for input and parses it to the specified type.
/// ```no_run
/// use eprompt::*;
/// let x: i32 = input("How old are you? ").unwrap();
/// ```
pub fn input<T: FromStr>(prompt: &str) -> Result<T> {
    let stdin = stdin();
    let mut stdout = stdout();
    let mut buffer = String::new();
    loop {
        print!("{}: ", prompt);
        queue!(stdout, SetForegroundColor(Color::Magenta))?;
        stdout.flush()?;
        buffer.clear();
        stdin.lock().read_line(&mut buffer)?;
        if let Ok(x) = buffer.trim().parse() {
            execute!(stdout, ResetColor)?;
            return Ok(x);
        }
        queue!(
            stdout,
            cursor::MoveUp(1),
            terminal::Clear(terminal::ClearType::CurrentLine),
            ResetColor,
        )?;
    }
}
