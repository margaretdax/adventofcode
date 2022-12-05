use utils::Solution;
mod solvers;

fn main() {
    let solver = get_solver();

    match solver {
        Some(s) => {
            let lines = get_lines(s.as_ref());
            let solution = s.solve(lines);
            match solution {
                Some(solution_str) => println!("{}", solution_str),
                None => println!("No solution for y{}/p{:0>2}.rs", s.get_year() % 1000, s.get_number())
            }
        }
        None => println!("boop")
    }
}

fn get_solver() -> Option<Box<dyn Solution>> {
    let mut args = std::env::args();
    args.next(); // removes the program name from args iterator
    solvers::name_to_solver(args.next())
}

fn get_lines(_solver: &dyn Solution) -> Vec<String> {
    let path = format!("problems/input/{}{:0>2}.in", _solver.get_year() % 1000, _solver.get_number());

    match std::fs::read_to_string(path) {
        Ok(file) => {
            let mut lines : Vec<String> = file.split('\n').map(|s| s.to_owned()).collect();
            if lines.last().unwrap().is_empty() {
                lines.pop();
            }
            lines
        }
        Err(_) => vec![]
    }
}