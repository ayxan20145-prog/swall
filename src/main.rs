use colored::Colorize;
use crossterm::{
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
    execute,
    cursor::MoveTo,
    style::Print,
    event::{self, Event, KeyCode},
};
use std::io::{self};

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    let mut selected: usize = 0;

    MoveTo(0, 0);

    loop {
        execute!(stdout, MoveTo(0, 0), Clear(ClearType::All))?;

        if selected == 0 {
            let name = "swww";
            let other_name = "swaybg";
            execute!(stdout, Print(format!("> {}\r\n", name.blue())))?;
            execute!(stdout, Print(format!("  {}\r\n", other_name.blue())))?;
        } else {
            let name = "swaybg";
            let other_name = "swww";

            execute!(stdout, Print(format!("  {}\r\n", other_name.blue())))?;
            execute!(stdout, Print(format!("> {}\r\n", name.blue())))?;
        }

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('j') | KeyCode::Down => {
                    if selected < 1 {
                        selected += 1;
                    }
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                KeyCode::Char('q') => {
                    break;
                }

                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}
