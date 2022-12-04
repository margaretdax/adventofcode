use std::io::{self, BufRead};
mod solvers;

fn main() {
    let mut args = std::env::args();
    args.next();
    let solver = solvers::name_to_solver(args.next());
    
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap()).collect();
    match solver {
        Some(s) => {
            let solution = s.solve(lines);
            match solution {
                Some(solution_str) => println!("{}", solution_str),
                None => println!("No solution for Solver {}-{}", s.get_year(), s.get_number())
            }
        }
        None => println!()
    }
}