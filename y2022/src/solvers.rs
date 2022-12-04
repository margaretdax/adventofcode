mod one;
mod two;
mod three;
mod four;

pub fn name_to_solver(name: Option<String>) -> Box<dyn utils::Solution> {
    match name {
        Some(x) => match x.as_str() {
            "1" => Box::new(one::DayOne),
            "2" => Box::new(two::DayTwo),
            "3" => Box::new(three::DayThree),
            "4" => Box::new(four::DayFour),
            _ => {println!("no option {}", x); panic!("oops")}
        },
        None => Box::new(four::DayFour)
    }
}