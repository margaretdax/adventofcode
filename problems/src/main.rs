use std::io::{self, BufRead};
mod solvers;

fn main() {
    let mut args = std::env::args();
    args.next();
    let solver = solvers::name_to_solver(args.next());
    
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap()).collect();
    let solution = solver.solve(lines);
    println!("{}", solution);
}