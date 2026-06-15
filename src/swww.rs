use crate::mode;

use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode},
    execute,
    style::Print,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};

pub fn run() -> io::Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    let mut selected: usize = 0;

    let entries = ["mode", "name"];

    let mut path = String::new();

    loop {
        execute!(stdout, MoveTo(0, 0), Clear(ClearType::All))?;

        for (i, name) in entries.iter().enumerate() {
            if i == selected {
                execute!(stdout, Print(format!("> {}\r\n", name)))?;
            } else {
                execute!(stdout, Print(format!("  {}\r\n", name)))?;
            }
        }

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('j') | KeyCode::Down => {
                    if selected + 1 < entries.len() {
                        selected += 1;
                    }
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
                    if selected == 0 {
                        disable_raw_mode()?;
                        mode::run()?;
                        enable_raw_mode()?;
                    } else {
                        execute!(stdout, Print(format!("Enter the path: ")))?;
                        io::stdout().flush()?;
                        disable_raw_mode()?;
                        io::stdin().read_line(&mut path)?;
                        enable_raw_mode()?;
                    }
                }
                KeyCode::Char('h') | KeyCode::Left | KeyCode::Char('q') => {
                    break;
                }

                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}
