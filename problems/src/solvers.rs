mod one;
mod two;
mod three;
mod four;
mod five;

pub fn name_to_solver(name: Option<String>) -> Box<dyn utils::Solution> {
    match name {
        Some(x) => match x.as_str() {
            "1" => Box::new(one::Solver),
            "2" => Box::new(two::Solver),
            "3" => Box::new(three::Solver),
            "4" => Box::new(four::Solver),
            "5" => Box::new(five::Solver),
            _ => {println!("no option {}", x); panic!("oops")}
        },
        None => Box::new(five::Solver)
    }
}