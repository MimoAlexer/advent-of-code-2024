use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
mod late;
pub type Answer = Result<String, String>;
pub fn solve(day: u8, part: u8, input: &str) -> Answer {
    if !(1..=25).contains(&day) || !(1..=2).contains(&part) {
        return Err("day must be 1..25 and part 1..2".into());
    }
    if day == 25 && part == 2 {
        return Err(
            "Day 25 has no computational part 2; earn the other stars on adventofcode.com.".into(),
        );
    }
    let s = input.replace("\r\n", "\n");
    Ok(match day {
        1 => d01(&s, part),
        2 => d02(&s, part),
        3 => d03(&s, part),
        4 => d04(&s, part),
        5 => d05(&s, part),
        6 => d06(&s, part),
        7 => d07(&s, part),
        8 => d08(&s, part),
        9 => d09(&s, part),
        10 => d10(&s, part),
        11 => d11(&s, part),
        12 => d12(&s, part),
        13 => d13(&s, part),
        _ => return late::solve(day, part, &s),
    })
}
pub(crate) fn nums(s: &str) -> Vec<i64> {
    s.split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter(|x| !x.is_empty() && *x != "-")
        .map(|x| x.parse().unwrap())
        .collect()
}
pub(crate) type P = (i32, i32);
pub(crate) const DIR: [P; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
pub(crate) fn add(a: P, b: P) -> P {
    (a.0 + b.0, a.1 + b.1)
}
pub(crate) fn grid(s: &str) -> HashMap<P, u8> {
    s.lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.bytes()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c))
        })
        .collect()
}
fn d01(s: &str, p: u8) -> String {
    let mut a = vec![];
    let mut b = vec![];
    for l in s.lines().filter(|l| !l.is_empty()) {
        let n = nums(l);
        a.push(n[0]);
        b.push(n[1]);
    }
    a.sort();
    b.sort();
    if p == 1 {
        a.iter().zip(b).map(|(x, y)| (x - y).abs()).sum::<i64>()
    } else {
        a.iter()
            .map(|x| x * b.iter().filter(|y| *y == x).count() as i64)
            .sum()
    }
    .to_string()
}
fn safe(v: &[i64]) -> bool {
    [1, -1]
        .iter()
        .any(|d| v.windows(2).all(|w| (1..=3).contains(&((w[1] - w[0]) * d))))
}
fn d02(s: &str, p: u8) -> String {
    s.lines()
        .filter(|l| {
            let v = nums(l);
            safe(&v)
                || (p == 2
                    && (0..v.len()).any(|i| {
                        safe(
                            &v.iter()
                                .enumerate()
                                .filter(|(j, _)| *j != i)
                                .map(|(_, x)| *x)
                                .collect::<Vec<_>>(),
                        )
                    }))
        })
        .count()
        .to_string()
}
fn d03(s: &str, p: u8) -> String {
    let b = s.as_bytes();
    let (mut i, mut total, mut on) = (0, 0u64, true);
    while i < b.len() {
        if b[i..].starts_with(b"do()") {
            on = true;
        }
        if b[i..].starts_with(b"don't()") {
            on = false;
        }
        if b[i..].starts_with(b"mul(") {
            let mut j = i + 4;
            let mut v = [0u64; 2];
            let mut ok = true;
            for (k, value) in v.iter_mut().enumerate() {
                let start = j;
                while j < b.len() && b[j].is_ascii_digit() && j - start < 3 {
                    *value = *value * 10 + (b[j] - b'0') as u64;
                    j += 1;
                }
                if j == start || j >= b.len() || b[j] != if k == 0 { b',' } else { b')' } {
                    ok = false;
                    break;
                }
                j += 1;
            }
            if ok && (p == 1 || on) {
                total += v[0] * v[1];
            }
        }
        i += 1;
    }
    total.to_string()
}
fn d04(s: &str, p: u8) -> String {
    let g = grid(s);
    let mut n = 0;
    for (&q, &c) in &g {
        if p == 1 && c == b'X' {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if (dx != 0 || dy != 0)
                        && (0..4).all(|i| {
                            g.get(&(q.0 + dx * i, q.1 + dy * i)) == Some(&b"XMAS"[i as usize])
                        })
                    {
                        n += 1;
                    }
                }
            }
        } else if p == 2 && c == b'A' {
            if [(-1, -1), (1, -1)].iter().all(|&(x, y)| {
                let a = g.get(&add(q, (x, y)));
                let b = g.get(&add(q, (-x, -y)));
                (a == Some(&b'M') && b == Some(&b'S')) || (a == Some(&b'S') && b == Some(&b'M'))
            }) {
                n += 1;
            }
        }
    }
    n.to_string()
}
fn d05(s: &str, p: u8) -> String {
    let (a, b) = s.split_once("\n\n").unwrap();
    let rules: Vec<_> = a
        .lines()
        .map(|l| {
            let n = nums(l);
            (n[0], n[1])
        })
        .collect();
    let mut sum = 0;
    for l in b.lines() {
        let v = nums(l);
        let valid = rules.iter().all(|(a, b)| {
            match (v.iter().position(|x| x == a), v.iter().position(|x| x == b)) {
                (Some(i), Some(j)) => i < j,
                _ => true,
            }
        });
        if valid && p == 1 {
            sum += v[v.len() / 2];
        }
        if !valid && p == 2 {
            let mut out = vec![];
            let mut left = v.clone();
            while !left.is_empty() {
                let i = left
                    .iter()
                    .position(|b| !rules.iter().any(|(a, z)| z == b && left.contains(a)))
                    .expect("cyclic rules");
                out.push(left.remove(i));
            }
            sum += out[out.len() / 2];
        }
    }
    sum.to_string()
}
fn patrol(g: &HashMap<P, u8>, start: P, block: Option<P>) -> (bool, HashSet<P>) {
    let (mut q, mut d) = (start, 3);
    let mut states = HashSet::new();
    let mut seen = HashSet::new();
    while g.contains_key(&q) {
        if !states.insert((q, d)) {
            return (true, seen);
        }
        seen.insert(q);
        let next = add(q, DIR[d]);
        if g.get(&next) == Some(&b'#') || Some(next) == block {
            d = (d + 1) % 4;
        } else {
            q = next;
        }
    }
    (false, seen)
}
fn d06(s: &str, p: u8) -> String {
    let g = grid(s);
    let start = *g.iter().find(|(_, c)| **c == b'^').unwrap().0;
    let (_, seen) = patrol(&g, start, None);
    if p == 1 {
        seen.len()
    } else {
        seen.into_iter()
            .filter(|&q| q != start && patrol(&g, start, Some(q)).0)
            .count()
    }
    .to_string()
}
fn d07(s: &str, p: u8) -> String {
    fn go(v: &[i64], i: usize, a: i128, t: i128, concat: bool) -> bool {
        if i == v.len() {
            return a == t;
        }
        // Positive operations cannot bring an overshoot back down unless a
        // later multiplication by zero is possible.
        if a > t && !v[i..].contains(&0) {
            return false;
        }
        let b = v[i] as i128;
        go(v, i + 1, a + b, t, concat)
            || go(v, i + 1, a * b, t, concat)
            || (concat
                && go(
                    v,
                    i + 1,
                    a * 10i128.pow(v[i].to_string().len() as u32) + b,
                    t,
                    concat,
                ))
    }
    s.lines()
        .map(|l| {
            let v = nums(l);
            if go(&v[1..], 1, v[1] as i128, v[0] as i128, p == 2) {
                v[0]
            } else {
                0
            }
        })
        .sum::<i64>()
        .to_string()
}
fn d08(s: &str, p: u8) -> String {
    let g = grid(s);
    let mut groups: HashMap<u8, Vec<P>> = HashMap::new();
    for (&q, &c) in &g {
        if c != b'.' {
            groups.entry(c).or_default().push(q);
        }
    }
    let mut out = HashSet::new();
    for v in groups.values() {
        for &a in v {
            for &b in v {
                if a == b {
                    continue;
                }
                let d = (a.0 - b.0, a.1 - b.1);
                let mut q = if p == 1 { add(a, d) } else { a };
                while g.contains_key(&q) {
                    out.insert(q);
                    if p == 1 {
                        break;
                    }
                    q = add(q, d);
                }
            }
        }
    }
    out.len().to_string()
}
fn d09(s: &str, p: u8) -> String {
    let mut disk = vec![];
    let mut files = vec![];
    for (i, c) in s.trim().bytes().enumerate() {
        let n = (c - b'0') as usize;
        if i % 2 == 0 {
            files.push((disk.len(), n));
        }
        disk.extend(std::iter::repeat_n(
            if i % 2 == 0 { Some(i / 2) } else { None },
            n,
        ));
    }
    if p == 1 {
        let (mut l, mut r) = (0, disk.len().saturating_sub(1));
        while l < r {
            if disk[l].is_some() {
                l += 1;
            } else if disk[r].is_none() {
                r -= 1;
            } else {
                disk[l] = disk[r].take();
                l += 1;
                r -= 1;
            }
        }
    } else {
        for (id, &(start, n)) in files.iter().enumerate().rev() {
            if n == 0 {
                continue;
            }
            let mut i = 0;
            while i + n <= start {
                if disk[i..i + n].iter().all(Option::is_none) {
                    for j in 0..n {
                        disk[i + j] = Some(id);
                        disk[start + j] = None;
                    }
                    break;
                }
                i += 1;
            }
        }
    }
    disk.iter()
        .enumerate()
        .map(|(i, v)| i as u64 * v.unwrap_or(0) as u64)
        .sum::<u64>()
        .to_string()
}
fn d10(s: &str, p: u8) -> String {
    fn walk(g: &HashMap<P, u8>, q: P, ends: &mut HashSet<P>) -> u64 {
        if g[&q] == b'9' {
            ends.insert(q);
            return 1;
        }
        DIR.iter()
            .map(|&d| add(q, d))
            .filter(|n| g.get(n) == Some(&(g[&q] + 1)))
            .map(|n| walk(g, n, ends))
            .sum()
    }
    let g = grid(s);
    g.iter()
        .filter(|(_, v)| **v == b'0')
        .map(|(&q, _)| {
            let mut ends = HashSet::new();
            let n = walk(&g, q, &mut ends);
            if p == 1 {
                ends.len() as u64
            } else {
                n
            }
        })
        .sum::<u64>()
        .to_string()
}
fn d11(s: &str, p: u8) -> String {
    let mut counts: HashMap<u128, u128> = HashMap::new();
    for n in s.split_whitespace() {
        *counts.entry(n.parse().unwrap()).or_default() += 1;
    }
    for _ in 0..if p == 1 { 25 } else { 75 } {
        let mut next = HashMap::new();
        for (n, c) in counts {
            if n == 0 {
                *next.entry(1).or_default() += c;
            } else {
                let digits = n.ilog10() + 1;
                if digits % 2 == 0 {
                    let k = 10u128.pow(digits / 2);
                    *next.entry(n / k).or_default() += c;
                    *next.entry(n % k).or_default() += c;
                } else {
                    *next.entry(n * 2024).or_default() += c;
                }
            }
        }
        counts = next;
    }
    counts.values().sum::<u128>().to_string()
}
fn d12(s: &str, p: u8) -> String {
    let g = grid(s);
    let mut seen = HashSet::new();
    let mut total = 0;
    for (&q, &c) in &g {
        if seen.contains(&q) {
            continue;
        }
        let mut region = HashSet::new();
        let mut todo = vec![q];
        seen.insert(q);
        while let Some(a) = todo.pop() {
            region.insert(a);
            for d in DIR {
                let b = add(a, d);
                if g.get(&b) == Some(&c) && seen.insert(b) {
                    todo.push(b);
                }
            }
        }
        let mut perimeter = 0;
        let mut corners = 0;
        for &a in &region {
            for i in 0..4 {
                let b = add(a, DIR[i]);
                let d = add(a, DIR[(i + 1) % 4]);
                if !region.contains(&b) {
                    perimeter += 1;
                }
                if (!region.contains(&b) && !region.contains(&d))
                    || (region.contains(&b)
                        && region.contains(&d)
                        && !region.contains(&add(b, DIR[(i + 1) % 4])))
                {
                    corners += 1;
                }
            }
        }
        total += region.len() * if p == 1 { perimeter } else { corners };
    }
    total.to_string()
}
fn d13(s: &str, p: u8) -> String {
    s.split("\n\n")
        .map(|b| {
            let n = nums(b);
            if n.len() != 6 {
                return 0;
            }
            let (ax, ay, bx, by) = (n[0] as i128, n[1] as i128, n[2] as i128, n[3] as i128);
            let shift = if p == 1 { 0 } else { 10_000_000_000_000i128 };
            let (x, y) = (n[4] as i128 + shift, n[5] as i128 + shift);
            let det = ax * by - ay * bx;
            if det == 0 {
                return 0;
            }
            let (a, b) = (x * by - y * bx, ax * y - ay * x);
            if a % det != 0 || b % det != 0 {
                return 0;
            }
            let (a, b) = (a / det, b / det);
            if a < 0 || b < 0 || (p == 1 && (a > 100 || b > 100)) {
                0
            } else {
                3 * a + b
            }
        })
        .sum::<i128>()
        .to_string()
}
