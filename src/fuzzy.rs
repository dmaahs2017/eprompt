use std::io::{stdout, Result, Write};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    queue,
    style::*,
    terminal::{self, ClearType},
};

fn filter_input<'a>(input: &[&'a str], filter_str: &str) -> Vec<usize> {
    input
        .iter()
        .enumerate()
        .filter_map(|(i, s)| {
            if s.to_lowercase().contains(filter_str) {
                Some(i)
            } else {
                None
            }
        })
        .collect()
}

/// An interactive prompt. Lets the user filter the options by typing. Select with
/// ctrl-{j,k}/{up/down} and choose one with enter.
pub fn fuzzy_prompt<'a>(input: &[&'a str]) -> Result<&'a str> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    queue!(stdout, terminal::EnterAlternateScreen)?;

    let mut filter_str = String::new();
    let mut filter_changed = false;
    let mut selected = 0;
    let mut filtered_input_indices = filter_input(input, &filter_str);
    let mut output_index = 0;

    loop {
        let term_size = terminal::size()?;
        if filter_changed {
            filtered_input_indices = filter_input(input, &filter_str);
            selected = 0;
            output_index = 0;
            filter_changed = false;
        }

        queue!(
            stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, term_size.0)
        )?;
        for (i, input_index) in filtered_input_indices.iter().enumerate() {
            if i == selected {
                output_index = *input_index;
                queue!(
                    stdout,
                    SetAttribute(Attribute::Bold),
                    SetAttribute(Attribute::Underlined),
                    SetForegroundColor(Color::Magenta),
                    Print(format!("{}\r\n", input[*input_index])),
                    SetAttribute(Attribute::Reset),
                )?;
            } else {
                queue!(stdout, Print(format!("{}\r\n", input[*input_index])))?;
            }
        }
        queue!(
            stdout,
            //terminal::Clear(ClearType::CurrentLine),
            Print(format!("> {}", &filter_str)),
        )?;
        stdout.flush()?;

        if let Event::Key(k) = event::read()? {
            match k.into() {
                FuzzyEvent::Up => {
                    selected = selected.saturating_sub(1);
                }
                FuzzyEvent::Down => {
                    selected = (selected + 1).min(filtered_input_indices.len() - 1);
                }
                FuzzyEvent::FilterChar(c) => {
                    filter_str.push(c);
                    filter_changed = true;
                }
                FuzzyEvent::Backspace => {
                    filter_str.pop();
                    filter_changed = true;
                }
                FuzzyEvent::Enter => break,
                FuzzyEvent::Noop => (),
            }
        }
    }

    queue!(stdout, terminal::LeaveAlternateScreen)?;
    stdout.flush()?;
    terminal::disable_raw_mode()?;

    Ok(input[output_index])
}

enum FuzzyEvent {
    Up,
    Down,
    FilterChar(char),
    Backspace,
    Enter,
    Noop,
}

impl From<KeyEvent> for FuzzyEvent {
    fn from(e: KeyEvent) -> Self {
        if e.modifiers.contains(KeyModifiers::CONTROL) {
            match e.code {
                KeyCode::Char('k') => Self::Up,
                KeyCode::Char('j') => Self::Down,
                _ => Self::Noop,
            }
        } else {
            match e.code {
                KeyCode::Enter => Self::Enter,
                KeyCode::Char(c) => Self::FilterChar(c),
                KeyCode::Backspace => Self::Backspace,
                KeyCode::Up => Self::Up,
                KeyCode::Down => Self::Down,
                _ => Self::Noop,
            }
        }
    }
}
