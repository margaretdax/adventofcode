use utils::*;

pub struct Solver;

impl Solution for Solver {
    fn get_number(self: &Self) -> i32 { 1 }

    fn solve(self: &Self, _lines: Vec<String>) -> Option<String> {
        let mut solution = "".to_owned();
        let mut answer;

        let mut elves = totals(&_lines);
        answer = elves.iter().cloned().max().unwrap();
        solution.push_str(format!("Part I: {}\n", answer).as_str());

        elves.sort();
        answer = elves.iter().rev().take(3).sum::<i32>();
        solution.push_str(format!("Part II: {}\n", answer).as_str());

        Some(solution)
    }
}

fn totals(lines: &Vec<String>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0];

    for line in lines.iter() {
        if line.is_empty() {
            result.push(0);
            continue;
        }
        let current = result.len() - 1;
        result[current] += line.parse::<i32>().unwrap();
    }

    result
}