use std::{
    env,
    fs::{self, File},
    io::{self, BufRead, BufReader},
    num::ParseIntError,
    path::Path,
};

// use underscore (_) to silence warnings about unused variables/values
#[derive(Debug)]
enum SumError {
    Io,
    Parse {
        _line: usize,
        _source: ParseIntError,
    },
    InvalidInput,
}

impl From<io::Error> for SumError {
    fn from(_e: io::Error) -> Self {
        SumError::Io
    }
}

fn main() -> Result<(), SumError> {
    println!("{:?}", parse_number("8"));
    println!("{:?}", parse_number("dp"));

    let total = sum_integers("numbers.txt")?;
    println!("Total: {}", total);

    // cargo run -- numbers.txt
    // <app-name:0> -- <args:1->n>
    let path = env::args().nth(1).ok_or_else(|| SumError::InvalidInput)?;
    let content = fs::read_to_string(&path)?;
    let chars = content.chars().count();

    println!("{}", chars);

    Ok(())
}

fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.trim().parse::<i32>()
}

fn sum_integers<P: AsRef<Path>>(path: P) -> Result<i64, SumError> {
    // AsRef<Path. means this function accepts anything that can be borrowed as &Path
    // (e.g. &str, String, PathBuf, &Path)
    let path = path.as_ref();
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut sum = 0i64;
    for (idx, line) in reader.lines().enumerate() {
        // no: numero
        let line_no = idx + 1;
        let text = line?;
        for tok in text.split_whitespace() {
            let num = tok.parse::<i64>().map_err(|e| SumError::Parse {
                _line: line_no,
                _source: e,
            })?;
            sum += num;
        }
    }
    Ok(sum)
}
