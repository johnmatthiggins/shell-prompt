use ansi_term::Colour;
use std::env;

fn main() -> std::io::Result<()> {
    let cwd = env::current_dir()?;
    let path = cwd
        .to_str()
        .map(|p| Colour::Red.paint(p));

    if let Some(s) = path {
        println!("{}", s);
    }

    Ok(())
}
