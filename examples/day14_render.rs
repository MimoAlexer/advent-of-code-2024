//! Render the visual puzzle's candidate timestamp in a terminal.
use std::{collections::HashSet, env, fs};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() || args.len() > 2 {
        return Err(
            "usage: cargo run --release --example day14_render -- INPUT_FILE [TIME]".into(),
        );
    }
    let input = fs::read_to_string(&args[0])?;
    let time: i64 = match args.get(1) {
        Some(t) => t.parse()?,
        None => advent_of_code_2024::solve(14, 2, &input)?.parse()?,
    };
    let robots: Vec<Vec<i64>> = input
        .lines()
        .map(|l| {
            l.split(|c: char| !c.is_ascii_digit() && c != '-')
                .filter(|s| !s.is_empty() && *s != "-")
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();
    let (w, h) = if robots.iter().all(|r| r[0] < 11 && r[1] < 7) {
        (11, 7)
    } else {
        (101, 103)
    };
    let points: HashSet<_> = robots
        .iter()
        .map(|r| {
            (
                (r[0] + time * r[2]).rem_euclid(w),
                (r[1] + time * r[3]).rem_euclid(h),
            )
        })
        .collect();
    println!("Time: {time}");
    for y in 0..h {
        println!(
            "{}",
            (0..w)
                .map(|x| if points.contains(&(x, y)) { '#' } else { ' ' })
                .collect::<String>()
        );
    }
    Ok(())
}
