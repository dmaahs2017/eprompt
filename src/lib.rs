use std::io::{stdout, stdin, Write, BufRead};
pub use crossterm::Result;

use crossterm::{
    execute,
    event::{ Event, KeyCode },
    terminal,
    cursor,
    event,
};


/// Prompts the user for input and parses it to the specified type.
/// ```no_run
/// use eprompt::*;
/// let x: i32 = input("How old are you? ").unwrap();
/// ```
pub fn input<T: std::str::FromStr>(prompt: &str) -> Result<T> {
    let stdin = stdin();
    let mut stdout = stdout();
    let mut buffer = String::new();
    loop {
        print!("{}: ", prompt);
        stdout.flush()?;
        buffer.clear();
        stdin.lock().read_line(&mut buffer)?;
        if let Ok(x) = buffer.trim().parse() {
            return Ok(x);
        }
        execute!(stdout, cursor::MoveUp(1), terminal::Clear(terminal::ClearType::CurrentLine))?;
    }
}


/// Offers the user a multiple selections to choose and returns a list of the chosed values.
/// this is basically a checkbox select.
/// ```no_run
/// use eprompt::*;
/// let chosen: Vec<&i32> = multi_select("Choose an option:", &[1, 2, 3]).unwrap();
/// ```
pub fn multi_select<'a, T: std::fmt::Display>(prompt: &str, opts: &'a [T]) -> Result<Vec<&'a T>> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    let mut selected = vec![false; opts.len()];
    let mut current = 0;

    write!(stdout, "{}\n\r", prompt)?;
    loop {
        for i in 0..opts.len() {
            if selected[i] {
                if i == current {
                    write!(stdout, "\t> [x] {}\n\r", opts[i])?;
                } else {
                    write!(stdout, "\t  [x] {}\n\r", opts[i])?;
                }
            } else  {
                if i == current {
                    write!(stdout, "\t> [ ] {}\n\r", opts[i])?;
                } else {
                    write!(stdout, "\t  [ ] {}\n\r", opts[i])?;
                }
            }
        }
        execute!(stdout, terminal::Clear(terminal::ClearType::CurrentLine))?;

        match event::read()? {
            Event::Key(k) => {
                match k.code.into() {
                    SelectEvent::Up => current = current.saturating_sub(1),
                    SelectEvent::Down => current = (current + 1).min(opts.len() - 1),
                    SelectEvent::Select => selected[current] = !selected[current],
                    SelectEvent::Enter => { 
                        terminal::disable_raw_mode()?;
                        return Ok(opts.into_iter().enumerate().filter_map(|( index, o )| {
                            if selected[index] { Some(o) }
                            else { None }
                        }).collect());
                    },
                    SelectEvent::Noop => (),

                }
            },
            _ => (),
        }
        execute!(stdout, cursor::MoveUp(opts.len() as u16))?;
    }
}

/// Offers the user a multiple options and the user may select one.
/// this is basically a radio select.
/// ```no_run
/// use eprompt::*;
/// let chosen: &i32 = select("Choose an option:", &[1, 2, 3]).unwrap();
/// ```
pub fn select<'a, T: std::fmt::Display>(prompt: &str, opts: &'a [T]) -> Result<&'a T> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();

    let mut selected = 0;

    write!(stdout, "{}\n\r", prompt)?;
    loop {
        for i in 0..opts.len() {
            if i == selected {
                write!(stdout, "\t> {}\n\r", opts[i])?;
            } else {
                write!(stdout, "\t  {}\n\r", opts[i])?;
            }
        }
        execute!(stdout, terminal::Clear(terminal::ClearType::CurrentLine))?;

        match event::read()? {
            Event::Key(k) => {
                match k.code.into() {
                    SelectEvent::Up => selected = selected.saturating_sub(1),
                    SelectEvent::Down => selected = (selected + 1).min(opts.len() - 1),
                    SelectEvent::Select | SelectEvent::Enter => { 
                        terminal::disable_raw_mode()?;
                        return Ok(&opts[selected]);
                    },
                    SelectEvent::Noop => (),

                }
            },
            _ => (),
        }
        execute!(stdout, cursor::MoveUp(opts.len() as u16))?;
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
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') =>  SelectEvent::Down,
            KeyCode::Enter => SelectEvent::Enter,
            KeyCode::Char(' ') => SelectEvent::Select,
            _ => SelectEvent::Noop,
        }
    }
}
