use std::io;

mod menu;
mod swww;
mod swaybg;

fn main() -> io::Result<()> {
    menu::run()?;

    Ok(())
}
