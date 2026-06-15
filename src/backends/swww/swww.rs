use crate::backends::swww::mode;

use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode},
    execute,
    style::Print,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};
use std::process::Command;

pub fn run() -> io::Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    let mut selected: usize = 0;

    let entries = ["name", "mode", "apply"];

    let mut path = String::new();
    let mut mode = String::new();
    let mut command = String::new();

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
                        execute!(stdout, Print(format!("Enter the path: ")))?;
                        io::stdout().flush()?;
                        disable_raw_mode()?;
                        path.clear();
                        io::stdin().read_line(&mut path)?;
                        enable_raw_mode()?;
                    } else if selected == 1 {
                        disable_raw_mode()?;
                        mode = mode::run()?.to_string();
                        enable_raw_mode()?;
                    } else {
                        if mode.trim().is_empty() {
                            command.clear();
                            command = String::from("swww img ") + path.trim();
                        } else {
                            command.clear();
                            command =
                                String::from("swww img ") + path.trim() + " --resize " + mode.trim();
                        }
                        execute!(stdout, Print(format!("Command: {}\r\n\r\n", command)))?;

                        disable_raw_mode()?;
                        execute!(stdout, Print(format!("Apply? (y/n)")))?;
                        io::stdout().flush()?;
                        let mut apply = String::new();
                        io::stdin().read_line(&mut apply)?;
                        if apply.trim() == "y" {
                            Command::new("pkill")
                                .arg("swaybg")
                                .status()?;

                            Command::new("sh")
                                .arg("-c")
                                .arg(&command)
                                .spawn()?;
                        }
                        enable_raw_mode()?;
                        break;
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
