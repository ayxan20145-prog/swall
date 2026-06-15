use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode},
    execute,
    style::Print,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::io;

pub fn run() -> io::Result<String> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    let mut selected: usize = 0;

    let entries = ["fill", "fit", "stretch", "center", "tile"];

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
                    return Ok(entries[selected].to_string());
                }
                KeyCode::Char('h') | KeyCode::Left | KeyCode::Char('q') => {
                    break;
                }

                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    Ok(String::new())
}
