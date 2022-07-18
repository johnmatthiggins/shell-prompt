use ansi_term::Colour;
use git2::Repository;
use git2::Error;
use std::env;

enum GitStatus {
    Clean(String),
    Dirty(String),
}

fn main() -> std::io::Result<()> {
    let cwd = env::current_dir()?;
    let path = cwd
        .to_str();

    if let Some(s) = path {
        match git_status(&s.to_string()) {
            Ok(b) =>
                println!("[{}@{} {}]({}) $\n> ",
                    "jhiggins",
                    "PAT",
                    s,
                    Colour::Green.paint(b)),
            Err(_) => println!("{} $", s),
        }
    }

    Ok(())
}

fn git_status(cwd: &String) -> Result<String, Error> {
    let repo = Repository::open(cwd);

    repo.and_then(|r| r
        .head()
        .and_then(|h| h
            .shorthand()
            .map(|x| x.to_string())
            .ok_or(Error::from_str("Could not find name."))))
}
