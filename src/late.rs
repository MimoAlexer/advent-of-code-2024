use super::*;
pub fn solve(d: u8, p: u8, s: &str) -> Answer {
    Ok(match d {
        14 => d14(s, p),
        15 => d15(s, p),
        16 => d16(s, p),
        17 => d17(s, p)?,
        18 => d18(s, p),
        19 => d19(s, p),
        20 => d20(s, p),
        21 => d21(s, p),
        22 => d22(s, p),
        23 => d23(s, p),
        24 => d24(s, p)?,
        25 => d25(s),
        _ => unreachable!(),
    })
}
fn d14(s: &str, p: u8) -> String {
    let robots: Vec<_> = s.lines().map(nums).collect();
    let (w, h) = if robots.iter().all(|v| v[0] < 11 && v[1] < 7) {
        (11, 7)
    } else {
        (101, 103)
    };
    let positions = |t: i64| {
        robots
            .iter()
            .map(|v| {
                (
                    (v[0] + t * v[2]).rem_euclid(w),
                    (v[1] + t * v[3]).rem_euclid(h),
                )
            })
            .collect::<Vec<_>>()
    };
    if p == 1 {
        let mut q = [0u64; 4];
        for (x, y) in positions(100) {
            if x != w / 2 && y != h / 2 {
                q[(x > w / 2) as usize + 2 * (y > h / 2) as usize] += 1;
            }
        }
        return q.iter().product::<u64>().to_string();
    }
    // Search the full period. Variance uses every robot and is less sensitive
    // to a few distant robots than an axis-aligned bounding box.
    (0..w * h)
        .min_by_key(|&t| {
            let v = positions(t);
            let n = v.len() as i128;
            let sx: i128 = v.iter().map(|&(x, _)| x as i128).sum();
            let sy: i128 = v.iter().map(|&(_, y)| y as i128).sum();
            let squared: i128 = v.iter().map(|&(x, y)| (x * x + y * y) as i128).sum();
            n * squared - sx * sx - sy * sy
        })
        .unwrap()
        .to_string()
}
fn d15(s: &str, p: u8) -> String {
    let (a, moves) = s.split_once("\n\n").unwrap();
    let mut walls = HashSet::new();
    let mut boxes = HashSet::new();
    let mut robot = (0, 0);
    let width = if p == 1 { 1 } else { 2 };
    for ((x, y), c) in grid(a) {
        let q = (x * width, y);
        match c {
            b'#' => {
                walls.insert(q);
                if p == 2 {
                    walls.insert(add(q, (1, 0)));
                }
            }
            b'O' => {
                boxes.insert(q);
            }
            b'@' => robot = q,
            _ => {}
        }
    }
    let find = |q: P, b: &HashSet<P>| {
        if b.contains(&q) {
            Some(q)
        } else if p == 2 && b.contains(&add(q, (-1, 0))) {
            Some(add(q, (-1, 0)))
        } else {
            None
        }
    };
    for c in moves.bytes() {
        let d = match c {
            b'>' => DIR[0],
            b'v' => DIR[1],
            b'<' => DIR[2],
            b'^' => DIR[3],
            _ => continue,
        };
        let next = add(robot, d);
        if walls.contains(&next) {
            continue;
        }
        let mut moving = HashSet::new();
        let mut todo = vec![];
        if let Some(b) = find(next, &boxes) {
            moving.insert(b);
            todo.push(b);
        }
        let mut ok = true;
        while let Some(b) = todo.pop() {
            for dx in 0..width {
                let dest = add(add(b, (dx, 0)), d);
                if walls.contains(&dest) {
                    ok = false;
                }
                if let Some(other) = find(dest, &boxes) {
                    if other != b && moving.insert(other) {
                        todo.push(other);
                    }
                }
            }
        }
        if ok {
            for b in &moving {
                boxes.remove(b);
            }
            for b in moving {
                boxes.insert(add(b, d));
            }
            robot = next;
        }
    }
    boxes
        .iter()
        .map(|&(x, y)| x as i64 + 100 * y as i64)
        .sum::<i64>()
        .to_string()
}
fn d16(s: &str, p: u8) -> String {
    let g = grid(s);
    let start = *g.iter().find(|(_, c)| **c == b'S').unwrap().0;
    let end = *g.iter().find(|(_, c)| **c == b'E').unwrap().0;
    let mut dist = HashMap::from([((start, 0usize), 0u64)]);
    let mut heap = BinaryHeap::from([Reverse((0u64, start, 0usize))]);
    let mut parents: HashMap<(P, usize), Vec<(P, usize)>> = HashMap::new();
    while let Some(Reverse((cost, q, d))) = heap.pop() {
        if dist[&(q, d)] != cost {
            continue;
        }
        for (n, nd, w) in [
            (add(q, DIR[d]), d, 1),
            (q, (d + 1) % 4, 1000),
            (q, (d + 3) % 4, 1000),
        ] {
            if !g.contains_key(&n) || g[&n] == b'#' {
                continue;
            }
            let new = cost + w;
            let old = *dist.get(&(n, nd)).unwrap_or(&u64::MAX);
            if new < old {
                dist.insert((n, nd), new);
                parents.insert((n, nd), vec![(q, d)]);
                heap.push(Reverse((new, n, nd)));
            } else if new == old {
                parents.entry((n, nd)).or_default().push((q, d));
            }
        }
    }
    let best = (0..4).filter_map(|d| dist.get(&(end, d))).min().unwrap();
    if p == 1 {
        return best.to_string();
    }
    let mut todo: Vec<_> = (0..4)
        .filter(|&d| dist.get(&(end, d)) == Some(best))
        .map(|d| (end, d))
        .collect();
    let mut seen = HashSet::new();
    while let Some(a) = todo.pop() {
        if seen.insert(a) {
            if let Some(v) = parents.get(&a) {
                todo.extend(v);
            }
        }
    }
    seen.iter()
        .map(|a| a.0)
        .collect::<HashSet<_>>()
        .len()
        .to_string()
}
fn vm(program: &[u64], mut r: [u64; 3], limit: usize) -> Result<Vec<u64>, String> {
    fn divide(value: u64, exponent: u64) -> u64 {
        if exponent >= 64 {
            0
        } else {
            value >> exponent
        }
    }
    let (mut ip, mut steps) = (0, 0);
    let mut out = vec![];
    while ip + 1 < program.len() {
        steps += 1;
        if steps > 1_000_000 {
            return Err("VM instruction limit reached".into());
        }
        let op = program[ip];
        let raw = program[ip + 1];
        let combo = || match raw {
            0..=3 => Ok(raw),
            4..=6 => Ok(r[(raw - 4) as usize]),
            _ => Err("invalid combo operand".to_string()),
        };
        match op {
            0 => r[0] = divide(r[0], combo()?),
            1 => r[1] ^= raw,
            2 => r[1] = combo()? % 8,
            3 => {
                if r[0] != 0 {
                    ip = raw as usize;
                    continue;
                }
            }
            4 => r[1] ^= r[2],
            5 => {
                out.push(combo()? % 8);
                if out.len() > limit {
                    return Ok(out);
                }
            }
            6 => r[1] = divide(r[0], combo()?),
            7 => r[2] = divide(r[0], combo()?),
            _ => return Err("invalid opcode".into()),
        }
        ip += 2;
    }
    Ok(out)
}
fn d17(s: &str, p: u8) -> Answer {
    let (a, b) = s.split_once("\n\n").unwrap();
    let n = nums(a);
    let r = [n[0] as u64, n[1] as u64, n[2] as u64];
    let code: Vec<u64> = nums(b).into_iter().map(|n| n as u64).collect();
    if p == 1 {
        return Ok(vm(&code, r, usize::MAX)?
            .iter()
            .map(u64::to_string)
            .collect::<Vec<_>>()
            .join(","));
    } // Reverse the octal loop, verifying each suffix with the actual VM.
    let mut candidates = vec![0u64];
    for i in (0..code.len()).rev() {
        let mut next = vec![];
        for a in candidates {
            for digit in 0..8 {
                if let Some(a) = a.checked_mul(8).and_then(|a| a.checked_add(digit)) {
                    if vm(&code, [a, r[1], r[2]], code.len())? == code[i..] {
                        next.push(a);
                    }
                }
            }
        }
        candidates = next;
    }
    candidates
        .into_iter()
        .min()
        .map(|n| n.to_string())
        .ok_or("no quine found for the puzzle's octal-loop program".into())
}
fn shortest(block: &HashSet<P>, size: i32) -> Option<usize> {
    if block.contains(&(0, 0)) {
        return None;
    }
    let mut q = VecDeque::from([((0, 0), 0)]);
    let mut seen = HashSet::from([(0, 0)]);
    while let Some((a, d)) = q.pop_front() {
        if a == (size, size) {
            return Some(d);
        }
        for dir in DIR {
            let b = add(a, dir);
            if b.0 >= 0
                && b.1 >= 0
                && b.0 <= size
                && b.1 <= size
                && !block.contains(&b)
                && seen.insert(b)
            {
                q.push_back((b, d + 1));
            }
        }
    }
    None
}
fn d18(s: &str, p: u8) -> String {
    let v: Vec<P> = s
        .lines()
        .map(|l| {
            let n = nums(l);
            (n[0] as i32, n[1] as i32)
        })
        .collect();
    let (size, count) = if v.iter().all(|&(x, y)| x <= 6 && y <= 6) {
        (6, 12)
    } else {
        (70, 1024)
    };
    if p == 1 {
        return shortest(&v.iter().take(count).copied().collect(), size)
            .expect("no path")
            .to_string();
    }
    let (mut lo, mut hi) = (0, v.len());
    while lo < hi {
        let mid = (lo + hi) / 2;
        if shortest(&v[..mid].iter().copied().collect(), size).is_some() {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    let (x, y) = v[lo - 1];
    format!("{x},{y}")
}
fn d19(s: &str, p: u8) -> String {
    let (a, b) = s.split_once("\n\n").unwrap();
    let towels: Vec<_> = a.split(", ").collect();
    b.lines()
        .map(|l| {
            let mut dp = vec![0u128; l.len() + 1];
            dp[0] = 1;
            for i in 0..l.len() {
                if !l.is_char_boundary(i) {
                    continue;
                }
                for t in &towels {
                    if l[i..].starts_with(t) {
                        dp[i + t.len()] += dp[i];
                    }
                }
            }
            if p == 1 {
                (dp[l.len()] > 0) as u128
            } else {
                dp[l.len()]
            }
        })
        .sum::<u128>()
        .to_string()
}
fn distances(g: &HashMap<P, u8>, start: P) -> HashMap<P, i32> {
    let mut d = HashMap::from([(start, 0)]);
    let mut q = VecDeque::from([start]);
    while let Some(a) = q.pop_front() {
        for dir in DIR {
            let b = add(a, dir);
            if g.get(&b).is_some_and(|c| *c != b'#') && !d.contains_key(&b) {
                d.insert(b, d[&a] + 1);
                q.push_back(b);
            }
        }
    }
    d
}
fn d20(s: &str, p: u8) -> String {
    let g = grid(s);
    let start = *g.iter().find(|(_, c)| **c == b'S').unwrap().0;
    let end = *g.iter().find(|(_, c)| **c == b'E').unwrap().0;
    let a = distances(&g, start);
    let b = distances(&g, end);
    let radius: i32 = if p == 1 { 2 } else { 20 };
    let mut n = 0;
    for (&q, &cost) in &a {
        for dx in -radius..=radius {
            for dy in -(radius - dx.abs())..=radius - dx.abs() {
                let len = dx.abs() + dy.abs();
                if let Some(rest) = b.get(&add(q, (dx, dy))) {
                    if a[&end] - (cost + rest + len) >= 100 {
                        n += 1;
                    }
                }
            }
        }
    }
    n.to_string()
}
fn keypad(numeric: bool) -> HashMap<u8, P> {
    let rows = if numeric {
        vec!["789", "456", "123", " 0A"]
    } else {
        vec![" ^A", "<v>"]
    };
    rows.iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.bytes()
                .enumerate()
                .filter(|(_, c)| *c != b' ')
                .map(move |(x, c)| (c, (x as i32, y as i32)))
        })
        .collect()
}
fn paths(pad: &HashMap<u8, P>, from: u8, to: u8) -> Vec<Vec<u8>> {
    fn go(q: P, t: P, pad: &HashMap<u8, P>, path: &mut Vec<u8>, out: &mut Vec<Vec<u8>>) {
        if q == t {
            let mut p = path.clone();
            p.push(b'A');
            out.push(p);
            return;
        }
        for (d, c) in DIR.into_iter().zip(b">v<^".iter().copied()) {
            let n = add(q, d);
            if (n.0 - t.0).abs() + (n.1 - t.1).abs() < (q.0 - t.0).abs() + (q.1 - t.1).abs()
                && pad.values().any(|&v| v == n)
            {
                path.push(c);
                go(n, t, pad, path, out);
                path.pop();
            }
        }
    }
    let mut out = vec![];
    go(pad[&from], pad[&to], pad, &mut vec![], &mut out);
    out
}
fn keycost(
    from: u8,
    to: u8,
    depth: u8,
    numeric: bool,
    memo: &mut HashMap<(u8, u8, u8, bool), u64>,
) -> u64 {
    let key = (from, to, depth, numeric);
    if let Some(&v) = memo.get(&key) {
        return v;
    }
    let best = paths(&keypad(numeric), from, to)
        .iter()
        .map(|path| {
            if depth == 0 {
                path.len() as u64
            } else {
                let mut prev = b'A';
                path.iter()
                    .map(|&c| {
                        let cost = keycost(prev, c, depth - 1, false, memo);
                        prev = c;
                        cost
                    })
                    .sum()
            }
        })
        .min()
        .unwrap();
    memo.insert(key, best);
    best
}
fn d21(s: &str, p: u8) -> String {
    let mut memo = HashMap::new();
    s.lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut prev = b'A';
            let len: u64 = l
                .bytes()
                .map(|c| {
                    let n = keycost(prev, c, if p == 1 { 2 } else { 25 }, true, &mut memo);
                    prev = c;
                    n
                })
                .sum();
            len * l.trim_end_matches('A').parse::<u64>().unwrap()
        })
        .sum::<u64>()
        .to_string()
}
fn secret(mut n: u64) -> u64 {
    n = (n ^ (n << 6)) & 0xffffff;
    n = (n ^ (n >> 5)) & 0xffffff;
    (n ^ (n << 11)) & 0xffffff
}
fn d22(s: &str, p: u8) -> String {
    let mut total = 0;
    let mut bananas: HashMap<[i8; 4], u64> = HashMap::new();
    for l in s.lines() {
        let mut n: u64 = l.parse().unwrap();
        let mut prev = (n % 10) as i8;
        let mut diff = VecDeque::new();
        let mut seen = HashSet::new();
        for _ in 0..2000 {
            n = secret(n);
            let price = (n % 10) as i8;
            diff.push_back(price - prev);
            prev = price;
            if diff.len() > 4 {
                diff.pop_front();
            }
            if diff.len() == 4 {
                let key = [diff[0], diff[1], diff[2], diff[3]];
                if seen.insert(key) {
                    *bananas.entry(key).or_default() += price as u64;
                }
            }
        }
        total += n;
    }
    if p == 1 {
        total
    } else {
        *bananas.values().max().unwrap_or(&0)
    }
    .to_string()
}
fn d23(s: &str, p: u8) -> String {
    let mut g: HashMap<&str, HashSet<&str>> = HashMap::new();
    for l in s.lines() {
        let (a, b) = l.split_once('-').unwrap();
        g.entry(a).or_default().insert(b);
        g.entry(b).or_default().insert(a);
    }
    let mut nodes: Vec<_> = g.keys().copied().collect();
    nodes.sort();
    if p == 1 {
        let mut n = 0;
        for i in 0..nodes.len() {
            for j in i + 1..nodes.len() {
                for k in j + 1..nodes.len() {
                    let (a, b, c) = (nodes[i], nodes[j], nodes[k]);
                    if [a, b, c].iter().any(|a| a.starts_with('t'))
                        && g[a].contains(b)
                        && g[a].contains(c)
                        && g[b].contains(c)
                    {
                        n += 1;
                    }
                }
            }
        }
        return n.to_string();
    }
    fn clique<'a>(
        r: &mut Vec<&'a str>,
        mut candidates: Vec<&'a str>,
        g: &HashMap<&'a str, HashSet<&'a str>>,
        best: &mut Vec<&'a str>,
    ) {
        while let Some(v) = candidates.pop() {
            if r.len() + candidates.len() + 1 <= best.len() {
                return;
            }
            r.push(v);
            let next = candidates
                .iter()
                .copied()
                .filter(|n| g[v].contains(n))
                .collect();
            clique(r, next, g, best);
            if r.len() > best.len() {
                *best = r.clone();
            }
            r.pop();
        }
    }
    let mut best = vec![];
    clique(&mut vec![], nodes, &g, &mut best);
    best.sort();
    best.join(",")
}
#[derive(Clone, Debug)]
struct Gate {
    a: String,
    b: String,
    op: String,
    out: String,
}
fn gate_index(g: &[Gate], op: &str, a: &str, b: &str) -> Option<usize> {
    g.iter()
        .position(|x| x.op == op && ((x.a == a && x.b == b) || (x.a == b && x.b == a)))
}
// The first broken ripple-adder identity supplies candidate output swaps.
fn repair_options(g: &[Gate], bits: usize) -> Result<(), Vec<(String, String)>> {
    fn need(g: &[Gate], op: &str, a: &str, b: &str) -> Result<String, Vec<(String, String)>> {
        if let Some(i) = gate_index(g, op, a, b) {
            return Ok(g[i].out.clone());
        }
        let outputs: HashSet<_> = g.iter().map(|g| g.out.as_str()).collect();
        let mut swaps = vec![];
        for x in g.iter().filter(|x| x.op == op) {
            for (c, d) in [(&x.a, &x.b), (&x.b, &x.a)] {
                if c == a && outputs.contains(d.as_str()) && outputs.contains(b) {
                    swaps.push((d.clone(), b.to_string()));
                }
                if c == b && outputs.contains(d.as_str()) && outputs.contains(a) {
                    swaps.push((d.clone(), a.to_string()));
                }
            }
        }
        // If both operands were renamed, neither currently matches. Try
        // restoring one operand; the next identity check restores the other.
        if swaps.is_empty() {
            for x in g.iter().filter(|x| x.op == op) {
                for c in [&x.a, &x.b] {
                    for d in [a, b] {
                        if c != d && outputs.contains(c.as_str()) && outputs.contains(d) {
                            swaps.push((c.clone(), d.to_string()));
                        }
                    }
                }
            }
        }
        Err(swaps)
    }
    macro_rules! need {
        ($op:expr,$a:expr,$b:expr) => {
            match need(g, $op, $a, $b) {
                Ok(v) => v,
                Err(v) => return Err(v),
            }
        };
    }
    let mut carry = String::new();
    for i in 0..bits {
        let x = format!("x{i:02}");
        let y = format!("y{i:02}");
        let z = format!("z{i:02}");
        let prop = need!("XOR", &x, &y);
        let gen = need!("AND", &x, &y);
        if i == 0 {
            if prop != z {
                return Err(vec![(prop, z)]);
            }
            carry = gen;
            continue;
        }
        let sum = need!("XOR", &prop, &carry);
        if sum != z {
            return Err(vec![(sum, z)]);
        }
        let both = need!("AND", &prop, &carry);
        carry = need!("OR", &gen, &both);
    }
    let z = format!("z{bits:02}");
    if carry != z {
        Err(vec![(carry, z)])
    } else {
        Ok(())
    }
}
fn d24(s: &str, p: u8) -> Answer {
    let (a, b) = s.split_once("\n\n").ok_or("missing gates")?;
    let mut values: HashMap<String, bool> = a
        .lines()
        .map(|l| {
            let (k, v) = l.split_once(": ").unwrap();
            (k.into(), v == "1")
        })
        .collect();
    let g: Vec<Gate> = b
        .lines()
        .map(|l| {
            let w: Vec<_> = l.split_whitespace().collect();
            Gate {
                a: w[0].into(),
                b: w[2].into(),
                op: w[1].into(),
                out: w[4].into(),
            }
        })
        .collect();
    if p == 1 {
        loop {
            let mut changed = false;
            for x in &g {
                if values.contains_key(&x.out) {
                    continue;
                }
                if let (Some(&a), Some(&b)) = (values.get(&x.a), values.get(&x.b)) {
                    values.insert(
                        x.out.clone(),
                        match x.op.as_str() {
                            "AND" => a & b,
                            "OR" => a | b,
                            "XOR" => a ^ b,
                            _ => return Err("invalid gate".into()),
                        },
                    );
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        let mut n = 0u128;
        for x in &g {
            if let Some(i) = x.out.strip_prefix('z') {
                let v = values.get(&x.out).ok_or("cyclic or unresolved circuit")?;
                if *v {
                    n |= 1u128 << i.parse::<u32>().unwrap();
                }
            }
        }
        return Ok(n.to_string());
    }
    fn search(g: Vec<Gate>, bits: usize, used: Vec<String>) -> Option<Vec<String>> {
        let options = match repair_options(&g, bits) {
            Ok(()) => return if used.len() == 8 { Some(used) } else { None },
            Err(options) => options,
        };
        if used.len() >= 8 {
            return None;
        }
        for (a, b) in options {
            if a == b || used.contains(&a) || used.contains(&b) {
                continue;
            }
            if !g.iter().any(|x| x.out == a) || !g.iter().any(|x| x.out == b) {
                continue;
            }
            let mut next = g.clone();
            for x in &mut next {
                if x.out == a {
                    x.out = b.clone();
                } else if x.out == b {
                    x.out = a.clone();
                }
            }
            let mut u = used.clone();
            u.extend([a, b]);
            if let Some(v) = search(next, bits, u) {
                return Some(v);
            }
        }
        None
    }
    let bits = values.keys().filter(|k| k.starts_with('x')).count();
    let mut result = search(g, bits, vec![])
        .ok_or("no four disjoint output swaps restore a ripple-carry adder")?;
    result.sort();
    Ok(result.join(","))
}
fn d25(s: &str) -> String {
    let mut locks = vec![];
    let mut keys = vec![];
    for b in s.trim().split("\n\n") {
        let lines: Vec<_> = b.lines().collect();
        let h: Vec<_> = (0..lines[0].len())
            .map(|x| lines.iter().filter(|l| l.as_bytes()[x] == b'#').count() - 1)
            .collect();
        if lines[0].bytes().all(|c| c == b'#') {
            locks.push((h, lines.len() - 2));
        } else {
            keys.push(h);
        }
    }
    locks
        .iter()
        .map(|(l, limit)| {
            keys.iter()
                .filter(|k| l.iter().zip(k.iter()).all(|(a, b)| a + b <= *limit))
                .count()
        })
        .sum::<usize>()
        .to_string()
}
