mod y22;

pub fn name_to_solver(name: Option<String>) -> Option<Box<dyn utils::Solution>> {
    let mut list: Vec<Box<dyn utils::Solution>> = vec![
        y22::get_solvers()
    ].into_iter().flatten().collect();
    
    match name {
        Some(name_str) => {
            let num = name_str.parse::<i32>().unwrap();
            let idx = list.iter().position(|x| x.get_number() == num).unwrap();
            Some(list.remove(idx))
        }
        None => {
            let idx = list.len() - 1;
            if !list.is_empty() {
                Some(list.remove(idx))
            } else {
                None
            }
        }
    }
}