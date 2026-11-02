use advent_of_code_2024::solve;
fn check(d: u8, s: &str, a: &str, b: &str) {
    assert_eq!(solve(d, 1, s).unwrap(), a, "day {d} part 1");
    assert_eq!(solve(d, 2, s).unwrap(), b, "day {d} part 2");
}
#[test]
fn day01() {
    check(1, "3   4\n4   3\n2   5\n1   3\n3   9\n3   3", "11", "31");
}
#[test]
fn day02() {
    check(
        2,
        "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9",
        "2",
        "4",
    );
}
#[test]
fn day03() {
    assert_eq!(
        solve(
            3,
            1,
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
        )
        .unwrap(),
        "161"
    );
    assert_eq!(
        solve(
            3,
            2,
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
        )
        .unwrap(),
        "48"
    );
}
#[test]
fn day04() {
    check(4,"MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX","18","9");
}
#[test]
fn day05() {
    check(5,"47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47","143","123");
}
#[test]
fn day06() {
    check(6,"....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...","41","6");
}
#[test]
fn day07() {
    check(7,"190: 10 19\n3267: 81 40 27\n83: 17 5\n156: 15 6\n7290: 6 8 6 15\n161011: 16 10 13\n192: 17 8 14\n21037: 9 7 18 13\n292: 11 6 16 20","3749","11387");
}
#[test]
fn day08() {
    check(8,"............\n........0...\n.....0......\n.......0....\n....0.......\n......A.....\n............\n............\n........A...\n.........A..\n............\n............","14","34");
}
#[test]
fn day09() {
    check(9, "2333133121414131402", "1928", "2858");
}
#[test]
fn day10() {
    check(
        10,
        "89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732",
        "36",
        "81",
    );
}
#[test]
fn day11() {
    check(11, "125 17", "55312", "65601038650482");
}
#[test]
fn day12() {
    check(12, "AAAA\nBBCD\nBBCC\nEEEC", "140", "80");
    check(12, "OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO", "772", "436");
}
#[test]
fn day13() {
    check(13,"Button A: X+94, Y+34\nButton B: X+22, Y+67\nPrize: X=8400, Y=5400\n\nButton A: X+26, Y+66\nButton B: X+67, Y+21\nPrize: X=12748, Y=12176\n\nButton A: X+17, Y+86\nButton B: X+84, Y+37\nPrize: X=7870, Y=6450\n\nButton A: X+69, Y+23\nButton B: X+27, Y+71\nPrize: X=18641, Y=10279","480","875318608908");
}
#[test]
fn day14() {
    assert_eq!(solve(14,1,"p=0,4 v=3,-3\np=6,3 v=-1,-3\np=10,3 v=-1,2\np=2,0 v=2,-1\np=0,0 v=1,3\np=3,0 v=-2,-2\np=7,6 v=-1,-3\np=3,0 v=-1,-2\np=9,3 v=2,3\np=7,3 v=-1,2\np=2,4 v=2,-3\np=9,5 v=-3,-3").unwrap(),"12");
    assert_eq!(solve(14, 2, "p=0,0 v=1,0\np=4,0 v=-1,0").unwrap(), "2");
}
#[test]
fn day15() {
    assert_eq!(solve(15,1,"########\n#..O.O.#\n##@.O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########\n\n<^^>>>vv<v>>v<<").unwrap(),"2028");
    assert_eq!(
        solve(
            15,
            2,
            "#######\n#...#.#\n#.....#\n#..OO@#\n#..O..#\n#.....#\n#######\n\n<vv<<^^<<^^"
        )
        .unwrap(),
        "618"
    );
}
#[test]
fn day16() {
    check(16,"###############\n#.......#....E#\n#.#.###.#.###.#\n#.....#.#...#.#\n#.###.#####.#.#\n#.#.#.......#.#\n#.#.#####.###.#\n#...........#.#\n###.#.#####.#.#\n#...#.....#.#.#\n#.#.#.###.#.#.#\n#.....#...#.#.#\n#.###.#.#.#.#.#\n#S..#.....#...#\n###############","7036","45");
}
#[test]
fn day17() {
    assert_eq!(
        solve(
            17,
            1,
            "Register A: 729\nRegister B: 0\nRegister C: 0\n\nProgram: 0,1,5,4,3,0"
        )
        .unwrap(),
        "4,6,3,5,6,3,5,2,1,0"
    );
    assert_eq!(
        solve(
            17,
            2,
            "Register A: 2024\nRegister B: 0\nRegister C: 0\n\nProgram: 0,3,5,4,3,0"
        )
        .unwrap(),
        "117440"
    );
}
#[test]
fn day18() {
    check(18,"5,4\n4,2\n4,5\n3,0\n2,1\n6,3\n2,4\n1,5\n0,6\n3,3\n2,6\n5,1\n1,2\n5,5\n2,5\n6,5\n1,4\n0,4\n6,4\n1,1\n6,1\n1,0\n0,5\n1,6\n2,0","22","6,1");
}
#[test]
fn day19() {
    check(
        19,
        "r, wr, b, g, bwu, rb, gb, br\n\nbrwrr\nbggr\ngbbr\nrrbgbr\nubwu\nbwurrg\nbrgr\nbbrgwb",
        "6",
        "16",
    );
}
#[test]
fn day20() {
    let s = "#####\n#S.E#\n#####";
    check(20, s, "0", "0");
}
#[test]
fn day20_long_cheats() {
    let s = format!(
        "{}\n#S{}#\n{}.#\n#E{}#\n{}",
        "#".repeat(104),
        ".".repeat(101),
        "#".repeat(102),
        ".".repeat(101),
        "#".repeat(104)
    );
    assert_eq!(solve(20, 1, &s).unwrap(), "52");
    let track: Vec<(i32, i32)> = (1..=102)
        .map(|x| (x, 1))
        .chain([(102, 2)])
        .chain((1..=102).rev().map(|x| (x, 3)))
        .collect();
    let mut count = 0;
    for i in 0..track.len() {
        for j in i + 1..track.len() {
            let len = (track[i].0 - track[j].0).abs() + (track[i].1 - track[j].1).abs();
            if len <= 20 && j as i32 - i as i32 - len >= 100 {
                count += 1;
            }
        }
    }
    assert_eq!(solve(20, 2, &s).unwrap(), count.to_string());
}
#[test]
fn day21() {
    check(
        21,
        "029A\n980A\n179A\n456A\n379A",
        "126384",
        "154115708116294",
    );
}
#[test]
fn day22() {
    assert_eq!(solve(22, 1, "1\n10\n100\n2024").unwrap(), "37327623");
    assert_eq!(solve(22, 2, "1\n2\n3\n2024").unwrap(), "23");
}
#[test]
fn day23() {
    check(23,"kh-tc\nqp-kh\nde-cg\nka-co\nyn-aq\nqp-ub\ncg-tb\nvc-aq\ntb-ka\nwh-tc\nyn-cg\nkh-ub\nta-co\nde-co\ntc-td\ntb-wq\nwh-td\nta-ka\ntd-qp\naq-cg\nwq-ub\nub-vc\nde-ta\nwq-aq\nwq-vc\nwh-yn\nka-de\nkh-ta\nco-tc\nwh-qp\ntb-vc\ntd-yn","7","co,de,ka,ta");
}
#[test]
fn day24() {
    assert_eq!(solve(24,1,"x00: 1\nx01: 1\nx02: 1\ny00: 0\ny01: 1\ny02: 0\n\nx00 AND y00 -> z00\nx01 XOR y01 -> z01\nx02 OR y02 -> z02").unwrap(),"4");
}
#[test]
fn day24_repair() {
    let mut gates = vec![
        ("x00 XOR y00".to_string(), "z00".to_string()),
        ("x00 AND y00".into(), "c01".into()),
    ];
    for i in 1..5 {
        gates.extend([
            (format!("x{i:02} XOR y{i:02}"), format!("p{i:02}")),
            (format!("x{i:02} AND y{i:02}"), format!("g{i:02}")),
            (format!("p{i:02} XOR c{i:02}"), format!("z{i:02}")),
            (format!("p{i:02} AND c{i:02}"), format!("t{i:02}")),
            (
                format!("g{i:02} OR t{i:02}"),
                if i == 4 {
                    "z05".into()
                } else {
                    format!("c{:02}", i + 1)
                },
            ),
        ]);
    }
    let swaps = [
        ("z00", "g01"),
        ("z01", "t02"),
        ("p03", "g04"),
        ("z04", "t04"),
    ];
    let mut s = String::new();
    for i in 0..5 {
        s.push_str(&format!("x{i:02}: 0\ny{i:02}: 0\n"));
    }
    s.push('\n');
    for (expr, mut out) in gates {
        for (a, b) in swaps {
            if out == a {
                out = b.into();
                break;
            }
            if out == b {
                out = a.into();
                break;
            }
        }
        s.push_str(&format!("{expr} -> {out}\n"));
    }
    assert_eq!(solve(24, 2, &s).unwrap(), "g01,g04,p03,t02,t04,z00,z01,z04");
}
#[test]
fn day25() {
    assert_eq!(solve(25,1,"#####\n.####\n.####\n.####\n.#.#.\n.#...\n.....\n\n#####\n##.##\n.#.##\n...##\n...#.\n...#.\n.....\n\n.....\n#....\n#....\n#...#\n#.#.#\n#.###\n#####\n\n.....\n.....\n#.#..\n###..\n###.#\n###.#\n#####\n\n.....\n.....\n.....\n#....\n#.#..\n#.#.#\n#####").unwrap(),"3");
    assert!(solve(25, 2, "").is_err());
}
#[test]
fn invalid_selection() {
    assert!(solve(0, 1, "").is_err());
    assert!(solve(1, 3, "").is_err());
}
#[test]
fn crlf() {
    check(1, "3 4\r\n4 3\r\n", "0", "7");
}
