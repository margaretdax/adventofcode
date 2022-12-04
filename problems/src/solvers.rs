mod p2201;
mod p2202;
mod p2203;
mod p2204;
mod p2205;

pub fn name_to_solver(name: Option<String>) -> Box<dyn utils::Solution> {
    match name {
        Some(x) => match x.as_str() {
            "1" => Box::new(p2201::Solver),
            "2" => Box::new(p2202::Solver),
            "3" => Box::new(p2203::Solver),
            "4" => Box::new(p2204::Solver),
            "5" => Box::new(p2205::Solver),
            _ => {println!("no option {}", x); panic!("oops")}
        },
        None => Box::new(p2205::Solver)
    }
}