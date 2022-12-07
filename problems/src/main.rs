use utils::Solution;
mod solvers;

fn main() {
    let solver = match get_solver_number_from_args() {
        Some((num, year)) => solvers::find_solver_matching(num, year),
        None => {
            let mut solvers = solvers::all_solvers();
            if solvers.is_empty() {
                None
            } else {
                let idx = solvers.len() - 1;
                Some(solvers.remove(idx))
            }
        }
    };

    match solver {
        Some(s) => {
            let input_paths = vec![get_solver_input_path(s.as_ref(), true), get_solver_input_path(s.as_ref(), false)];
            for path in input_paths {
                let lines = get_lines(&path);
                let solution = s.solve(lines);
                println!("Running for input \"{}\"", path);
                match solution {
                    Some(solution_str) => println!("{}", solution_str),
                    None => println!("No solution for y{}/p{:0>2}.rs", s.get_year() % 1000, s.get_number())
                }
            }
        }
        None => println!("No solution ")
    }
}

fn get_solver_number_from_args() -> Option<(i32, i32)> {
    let mut args = std::env::args();
    args.next();
    let num_arg = args.next();
    let year_arg = args.next();

    let year = match year_arg {
        Some(year_str) => year_str.parse::<i32>().ok(),
        None => None
    };

    match num_arg {
        Some(num_str) => {
            Some((num_str.parse::<i32>().unwrap(), year.unwrap_or(2022)))
        }
        None => None
    }
}

fn get_solver_input_path(solver: &dyn Solution, is_test: bool) -> String {
    let mut input = format!("problems/input/{}{:0>2}.in", solver.get_year() % 1000, solver.get_number());

    if is_test {
        input.push_str(".test");
    }

    input
}

fn get_lines(path: &String) -> Vec<String> {
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