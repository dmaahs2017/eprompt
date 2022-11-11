use std::io::{stdout, Result, Write};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    queue,
    style::*,
    terminal,
};

/// Prompt the user with various options and return an interator over selected indicies, and references to the
/// chosen options.
/// ```no_run
/// use eprompt::*;
/// let chosen: Vec<usize> = multi_select("Choose an option:", &["User chooses 1", "Not Chosen", "Chosen"]).unwrap().map(|x| x.0).collect();
/// assert_eq!(chosen, vec![0, 1]);
/// ```
pub fn multi_select<'a, T: std::fmt::Display>(
    message: &str,
    opts: &'a [T],
) -> Result<impl Iterator<Item = (usize, &'a T)>> {
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
                    return Ok(opts.iter().enumerate().filter(move |o| selected[o.0]));
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
