use std::io;

mod menu;
mod backends;

fn main() -> io::Result<()> {
    menu::run()?;

    Ok(())
}
