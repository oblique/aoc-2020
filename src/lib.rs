use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn file_lines<P>(path: P) -> Result<impl Iterator<Item = String>>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let f = File::open(path)
        .with_context(|| format!("Failed to open '{}'", path.display()))?;

    Ok(BufReader::new(f)
        .lines()
        .take_while(|res| res.is_ok())
        .map(|res| res.unwrap()))
}
