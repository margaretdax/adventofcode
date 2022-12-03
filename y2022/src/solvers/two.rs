pub struct DayTwo;

#[derive(PartialEq, Eq, Clone)]
enum RPS { Rock, Paper, Scissors }

impl utils::Solution for DayTwo {
    fn solve(self: Box<Self>, lines: Vec<String>) -> String {
        let mut solution = "".to_owned();
        let mut score = 0;
        for line in lines.iter() {
            let (a, b) = parse_line(line);
            score += score_line(a, b);
        }
        solution.push_str(format!("Part I: {}\n", score).as_str());
        
        score = 0;
        for line in lines.iter() {
            let (a, b) = parse_line(line);
            let (c, d) = trans(a, b);
            score += score_line(c, d);
        }
        solution.push_str(format!("Part II: {}\n", score).as_str());

        solution
    }
}

fn trans(a: RPS, b: RPS) -> (RPS, RPS) {
    let c = a.clone();
    let d = match b {
        RPS::Rock => // Lose
            match a {
                RPS::Rock => RPS::Scissors,
                RPS::Paper => RPS::Rock,
                RPS::Scissors => RPS::Paper
            },
        RPS::Paper => a, // Draw
        RPS::Scissors => // Win
            match a {
                RPS::Rock => RPS::Paper,
                RPS::Paper => RPS::Scissors,
                RPS::Scissors => RPS::Rock
            }
    };

    (c, d)
}

fn score_line(a: RPS, b: RPS) -> i32 {
    let play = match b {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS:: Scissors => 3,
    };

    let win = winner(a, b);

    play + (win + 1) * 3
}

fn parse_line(line: &String) -> (RPS, RPS) {
    let mut line_iter =  line.split_whitespace();
    let a = match line_iter.next() {
        Some("A") => RPS::Rock,
        Some("B") => RPS::Paper,
        Some("C") => RPS::Scissors,
        _ => panic!("invalid input")
    };

    let b = match line_iter.next() {
        Some("X") => RPS::Rock,
        Some("Y") => RPS::Paper,
        Some("Z") => RPS::Scissors,
        _ => panic!("invalid input")
    };

    (a, b)
}

fn winner(a: RPS, b: RPS) -> i32 {
    if a == b {
        return 0;
    }

    if (a == RPS::Rock && b == RPS::Scissors) || (a == RPS::Scissors && b == RPS::Paper) || (a == RPS::Paper && b == RPS::Rock) {
        return -1;
    }

    1
}
