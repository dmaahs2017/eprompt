pub use crossterm::Result;
use std::io::{stdin, stdout, BufRead, Write};

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
pub fn input<T: std::str::FromStr>(prompt: &str) -> Result<T> {
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

/// Offers the user a multiple selections to choose and returns a list of the chosed values.
/// this is basically a checkbox select.
/// ```no_run
/// use eprompt::*;
/// let chosen: Vec<&i32> = multi_select("Choose an option:", &[1, 2, 3]).unwrap();
/// ```
pub fn multi_select<'a, T: std::fmt::Display>(message: &str, opts: &'a [T]) -> Result<Vec<&'a T>> {
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
                        .filter(|o| selected[o.0])
                        .map(|o| o.1)
                        .collect());
                }
                SelectEvent::Noop => (),
            }
        }
        queue!(stdout, cursor::MoveUp(opts.len() as u16))?;
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
                    return Ok(&opts[selected]);
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
