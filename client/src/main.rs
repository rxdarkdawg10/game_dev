use crate::errors::{Error, Result};

mod errors;

fn main() -> Result<()> {
    let sdl2_context = sdl2::init()?;

    Ok(())
}
