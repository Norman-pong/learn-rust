use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("用法: {} <name>", args[0]);
        process::exit(1);
    }

    let name = &args[1];
    println!("Hello, {}!", name);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn test_greeting() {
        let output = Command::new(env!("CARGO_BIN_EXE_hello-rust"))
            .arg("Rust")
            .output()
            .unwrap();
        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Hello, Rust!"));
    }
}
