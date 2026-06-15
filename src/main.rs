use std::io;

mod menu;
mod mode;
mod swaybg;
mod swww;

fn main() -> io::Result<()> {
    menu::run()?;

    Ok(())
}
