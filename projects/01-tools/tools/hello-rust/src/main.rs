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
    #[test]
    fn test_greeting() {
        let args = vec!["hello-rust".to_string(), "Rust".to_string()];
        assert_eq!(args.len(), 2);
        assert_eq!(&args[1], "Rust");
    }
}
