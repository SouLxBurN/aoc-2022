fn main() {
    let input = std::fs::read_to_string("./src/bin/day2.input").unwrap();
    part1(&input);
    part2(&input);
}

// A for Rock, B for Paper, and C for Scissors
// X for Rock, Y for Paper, and Z for Scissors
fn part1(input: &str) {
    let total_points = input.split("\n").filter_map(|round| {
        let t = round.split_once(" ");
        if t.is_some() {
            t
        } else {
            None
        }
    }).map(|round_tuple| {
        match round_tuple {
            ("A", "X") => 1 + 3,
            ("A", "Y") => 2 + 6,
            ("A", "Z") => 3 + 0,
            ("B", "X") => 1 + 0,
            ("B", "Y") => 2 + 3,
            ("B", "Z") => 3 + 6,
            ("C", "X") => 1 + 6,
            ("C", "Y") => 2 + 0,
            ("C", "Z") => 3 + 3,
            _ => 0,
        }
    }).sum::<u32>();
    println!("{:?}", total_points);
}

// A for Rock, B for Paper, and C for Scissors
// X to Love, Y to Draw, and Z to Win
fn part2(input: &str) {
    let total_points = input.split("\n").filter_map(|round| {
        let t = round.split_once(" ");
        if t.is_some() {
            t
        } else {
            None
        }
    }).map(|round_tuple| {
        match round_tuple {
            ("A", "X") => 3 + 0,
            ("A", "Y") => 1 + 3,
            ("A", "Z") => 2 + 6,
            ("B", "X") => 1 + 0,
            ("B", "Y") => 2 + 3,
            ("B", "Z") => 3 + 6,
            ("C", "X") => 2 + 0,
            ("C", "Y") => 3 + 3,
            ("C", "Z") => 1 + 6,
            _ => 0,
        }
    }).sum::<u32>();
    println!("{}", total_points);
}
