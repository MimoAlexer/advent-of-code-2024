use std::{
    env, fs,
    io::{self, Read},
};
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() < 2 || args.len() > 3 {
        return Err("usage: cargo run --release -- DAY PART [INPUT_FILE] (otherwise stdin)".into());
    }
    let day = args[0].parse()?;
    let part = args[1].parse()?;
    let input = if let Some(path) = args.get(2) {
        fs::read_to_string(path)?
    } else {
        let mut s = String::new();
        io::stdin().read_to_string(&mut s)?;
        s
    };
    println!("{}", advent_of_code_2024::solve(day, part, &input)?);
    Ok(())
}
