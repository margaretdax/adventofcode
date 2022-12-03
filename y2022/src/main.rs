use std::io::{self, BufRead};

#[derive(PartialEq, Eq, Clone)]
enum RPC { Rock, Paper, Scissors }

fn main() {
    let stdin = io::stdin();

    let mut score = 0;
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let (left, right) = split_sack(line);
        let overlap = left.iter().filter(|x| right.contains(x)).next().unwrap();
        let overlap_score = priority(overlap);
        score += overlap_score;
    }

    println!("{}", score);
}

fn split_sack(line: String) -> (Vec<u8>, Vec<u8>) {
    let index = line.len() / 2;
    let (left, right) = line.split_at(index);
    if left.len() != right.len() {
        panic!("halfing is borked");
    }

    (left.as_bytes().to_vec(), right.as_bytes().to_vec())
}

fn priority(item: &u8) -> i32 {
    if *item >= b'a' &&  *item <= b'z' {
        (item - b'a' + 1).into()
    } else if *item >= b'A' && *item <= b'Z' {
        (item - b'A' + 27).into()
    } else {
        panic!("invalid item")
    }
}

fn trans(a: RPC, b: RPC) -> (RPC, RPC) {
    let c = a.clone();
    let d = match b {
        RPC::Rock => // Lose
            match a {
                RPC::Rock => RPC::Scissors,
                RPC::Paper => RPC::Rock,
                RPC::Scissors => RPC::Paper
            },
        RPC::Paper => a, // Draw
        RPC::Scissors => // Win
            match a {
                RPC::Rock => RPC::Paper,
                RPC::Paper => RPC::Scissors,
                RPC::Scissors => RPC::Rock
            }
    };

    (c, d)
}

fn score_line(a: RPC, b: RPC) -> i32 {
    let play = match b {
        RPC::Rock => 1,
        RPC::Paper => 2,
        RPC:: Scissors => 3,
    };

    let win = winner(a, b);

    play + (win + 1) * 3
}

fn parse_line(line: &String) -> (RPC, RPC) {
    let mut line_iter =  line.split_whitespace();
    let a = match line_iter.next() {
        Some("A") => RPC::Rock,
        Some("B") => RPC::Paper,
        Some("C") => RPC::Scissors,
        _ => panic!("invalid input")
    };

    let b = match line_iter.next() {
        Some("X") => RPC::Rock,
        Some("Y") => RPC::Paper,
        Some("Z") => RPC::Scissors,
        _ => panic!("invalid input")
    };

    (a, b)
}

fn winner(a: RPC, b: RPC) -> i32 {
    if a == b {
        return 0;
    }

    if (a == RPC::Rock && b == RPC::Scissors) || (a == RPC::Scissors && b == RPC::Paper) || (a == RPC::Paper && b == RPC::Rock) {
        return -1;
    }

    1
}
