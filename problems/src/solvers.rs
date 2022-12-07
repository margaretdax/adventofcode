use utils::Solution;

mod y22;

pub fn all_solvers() -> Vec<Box<dyn Solution>> {
    vec![
        y22::get_solvers()
    ].into_iter().flatten().collect()
}

pub fn find_solver_matching(num: i32, year: i32) -> Option<Box<dyn Solution>> {
    let mut list: Vec<Box<dyn utils::Solution>> = all_solvers();
    
    let idx = list.iter().position(|s| s.get_number() == num && s.get_year() == year);

    match idx {
        Some(i) => Some(list.remove(i)),
        None => None
    }
}