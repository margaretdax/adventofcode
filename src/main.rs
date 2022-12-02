use std::io::{self, BufRead};

#[derive(PartialEq, Eq)]
enum RPC { Rock, Paper, Scissors }

fn main() {
    let stdin = io::stdin();

    let mut score = 0;
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let (a, b) = parse_line(&line);
        score += score_line(a, b);
    }

    println!("{}", score);
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
