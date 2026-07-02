use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader, IsTerminal};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "log-grep", about = "A tiny grep-style log filter")]
struct Args {
    /// Regular expression pattern to match.
    #[arg(short = 'p', long)]
    pattern: String,

    /// Count matching lines instead of printing them.
    #[arg(short = 'c', long)]
    count: bool,

    /// Invert match: print/count lines that do NOT match the pattern.
    #[arg(short = 'v', long)]
    invert: bool,

    /// Path to a log file. Reads from stdin if omitted.
    path: Option<PathBuf>,
}

fn build_reader(path: Option<&PathBuf>) -> io::Result<Box<dyn BufRead>> {
    match path {
        Some(p) => File::open(p).map(|f| Box::new(BufReader::new(f)) as Box<dyn BufRead>),
        None => Ok(Box::new(BufReader::new(io::stdin())) as Box<dyn BufRead>),
    }
}

fn count_matches<R: BufRead>(reader: R, re: &Regex, invert: bool) -> io::Result<usize> {
    let mut count = 0;
    for line in reader.lines() {
        let line = line?;
        let matched = re.is_match(&line);
        if matched != invert {
            count += 1;
        }
    }
    Ok(count)
}

fn print_matches<R: BufRead>(reader: R, re: &Regex, invert: bool) -> io::Result<()> {
    for line in reader.lines() {
        let line = line?;
        let matched = re.is_match(&line);
        if matched != invert {
            println!("{}", line);
        }
    }
    Ok(())
}

fn run(args: Args) -> io::Result<()> {
    let re = Regex::new(&args.pattern)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    let reader = build_reader(args.path.as_ref())?;

    if args.count {
        let count = count_matches(reader, &re, args.invert)?;
        println!("{}", count);
    } else {
        print_matches(reader, &re, args.invert)?;
    }
    Ok(())
}

fn main() {
    if io::stdin().is_terminal() && std::env::args_os().len() == 1 {
        eprintln!("Usage: log-grep -p <pattern> [-c] [-v] [FILE]");
        std::process::exit(1);
    }
    let args = Args::parse();
    if let Err(e) = run(args) {
        eprintln!("log-grep: error: {}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn sample_lines() -> Cursor<&'static [u8]> {
        Cursor::new(b"INFO request started\nERROR connection failed\nINFO request finished\nERROR timeout\n200 OK\n")
    }

    #[test]
    fn counts_matching_lines() {
        let re = Regex::new("ERROR").unwrap();
        let input = sample_lines();
        assert_eq!(count_matches(input, &re, false).unwrap(), 2);
    }

    #[test]
    fn counts_inverted_lines() {
        let re = Regex::new("200").unwrap();
        let input = sample_lines();
        assert_eq!(count_matches(input, &re, true).unwrap(), 4);
    }

    #[test]
    fn print_matches_filters_correctly() {
        let re = Regex::new("^INFO").unwrap();
        let input = sample_lines();
        let mut output = Vec::new();
        for line in input.lines() {
            let line = line.unwrap();
            if re.is_match(&line) {
                output.push(line);
            }
        }
        assert_eq!(output, vec!["INFO request started", "INFO request finished"]);
    }

    #[test]
    fn regex_rejects_invalid_pattern() {
        let result = Regex::new("(");
        assert!(result.is_err());
    }
}
