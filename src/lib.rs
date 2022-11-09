//! Here is a cool example with enums!
//!
//!```no_run
//! use eprompt::*;
//! use std::fmt::Display;
//! 
//! #[derive(Debug)]
//! enum Choices {
//!     Opt1(i32),
//!     Opt2(&'static str),
//!     Opt3,
//! }
//! 
//! impl Display for Choices {
//!     fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
//!         match *self {
//!             Self::Opt1(x) => write!(fmt, "{} of Option 1", x),
//!             Self::Opt2(x) => write!(fmt, "Option 2 is {}", x),
//!             Self::Opt3 => write!(fmt, "Mystery option 3"),
//!         }
//!     }
//! }
//! 
//! fn main() -> Result<(), std::io::Error> {
//!     for choice in multi_select("Make a selection", 
//!         &[Choices::Opt1(69), Choices::Opt2("A Brand new pony"), Choices::Opt3]
//!     )?.map(|x| x.1) {
//!         match choice {
//!             Choices::Opt1(x) => todo!("Do something with option 1"),
//!             Choices::Opt2(x) => todo!("Do something with option 2"),
//!             Choices::Opt3 => todo!("Do something with option 3"),
//!         }
//!     }
//!     
//!     Ok(())
//! }
//! ```
use crossterm::Result;
use std::io::{stdin, stdout, BufRead, Write};
use std::str::FromStr;

use crossterm::{
    cursor, event,
    event::{Event, KeyCode},
    execute, queue,
    style::*,
    terminal,
};

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

/// Prompt the user with various options and return an interator over selected indicies, and references to the
/// chosen options.
/// ```no_run
/// use eprompt::*;
/// let chosen: Vec<usize> = multi_select("Choose an option:", &["User chooses 1", "Not Chosen", "Chosen"]).unwrap().map(|x| x.0).collect();
/// assert_eq!(chosen, vec![0, 1]);
/// ```
pub fn multi_select<'a, T: std::fmt::Display>(message: &str, opts: &'a [T]) -> Result<impl Iterator<Item=(usize, &'a T)>> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    let mut selected = vec![false; opts.len()];
    let mut current = 0;

    queue!(stdout, Print(format!("{}\n\r", message)))?;
    loop {
        for i in 0..opts.len() {
            queue!(stdout, Print('\t'))?;
            if i == current {
                queue!(
                    stdout,
                    SetAttribute(Attribute::Bold),
                    SetAttribute(Attribute::Underlined),
                    SetForegroundColor(Color::Magenta),
                    Print('>'),
                )?;
            } else {
                queue!(stdout, Print(' '))?;
            }
            queue!(stdout, Print(" ["))?;
            if selected[i] {
                queue!(stdout, Print('x'))?;
            } else {
                queue!(stdout, Print(' '))?;
            }
            queue!(
                stdout,
                Print(format!("] {}\n\r", opts[i])),
                SetAttribute(Attribute::Reset)
            )?;
        }
        queue!(stdout, terminal::Clear(terminal::ClearType::CurrentLine))?;
        stdout.flush()?;

        if let Event::Key(k) = event::read()? {
            match k.code.into() {
                SelectEvent::Up => current = current.saturating_sub(1),
                SelectEvent::Down => current = (current + 1).min(opts.len() - 1),
                SelectEvent::Select => selected[current] = !selected[current],
                SelectEvent::Enter => {
                    terminal::disable_raw_mode()?;
                    return Ok(opts
                        .iter()
                        .enumerate()
                        .filter(move |o| selected[o.0])
                    );
                }
                SelectEvent::Noop => (),
            }
        }
        queue!(stdout, cursor::MoveUp(opts.len() as u16))?;
    }
}

/// Gets single user choice from `opts` and returns the selected index and reference to the
/// selected option
/// ```no_run
/// use eprompt::*;
/// let chosen: (usize, &i32) = select("Choose an option:", &[1, 2, 3]).unwrap();
/// ```
pub fn select<'a, T: std::fmt::Display>(prompt: &str, opts: &'a [T]) -> Result<(usize, &'a T)> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();

    let mut selected = 0;

    queue!(stdout, Print(format!("{}\n\r", prompt)))?;
    loop {
        for (i, opt) in opts.iter().enumerate() {
            if i == selected {
                queue!(
                    stdout,
                    SetAttribute(Attribute::Bold),
                    SetAttribute(Attribute::Underlined),
                    SetForegroundColor(Color::Magenta),
                    Print(format!("\t> {}\n\r", opt)),
                    SetAttribute(Attribute::Reset),
                )?;
            } else {
                queue!(stdout, Print(format!("\t  {}\n\r", opts[i])))?;
            }
        }
        queue!(stdout, terminal::Clear(terminal::ClearType::CurrentLine))?;
        stdout.flush()?;

        if let Event::Key(k) = event::read()? {
            match k.code.into() {
                SelectEvent::Up => selected = selected.saturating_sub(1),
                SelectEvent::Down => selected = (selected + 1).min(opts.len() - 1),
                SelectEvent::Select | SelectEvent::Enter => {
                    terminal::disable_raw_mode()?;
                    return Ok((selected, &opts[selected]));
                }
                SelectEvent::Noop => (),
            }
        }
        queue!(stdout, cursor::MoveUp(opts.len() as u16))?;
    }
}

enum SelectEvent {
    Up,
    Down,
    Select,
    Enter,
    Noop,
}

impl From<KeyCode> for SelectEvent {
    fn from(value: KeyCode) -> Self {
        match value {
            KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => SelectEvent::Up,
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => SelectEvent::Down,
            KeyCode::Enter => SelectEvent::Enter,
            KeyCode::Char(' ') => SelectEvent::Select,
            _ => SelectEvent::Noop,
        }
    }
}
