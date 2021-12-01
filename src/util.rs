use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
pub enum Error<S> {
    FromStr(S),
    Io(std::io::Error),
}

pub fn parse_file<P: AsRef<Path>, S: FromStr>(
    path: P,
) -> Result<Vec<S>, Error<<S as FromStr>::Err>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .map_err(Error::Io)?;
    lines
        .into_iter()
        .map(|l| l.parse().map_err(Error::FromStr))
        .collect()
}
