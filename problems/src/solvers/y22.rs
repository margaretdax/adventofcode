mod p01;
mod p02;
mod p03;
mod p04;
mod p05;
mod p06;
mod p07;
mod p08;
mod p09;

pub fn get_solvers() -> Vec<Box<dyn utils::Solution>> {
	vec![
		Box::new(p01::Solver),
		Box::new(p02::Solver),
		Box::new(p03::Solver),
		Box::new(p04::Solver),
		Box::new(p05::Solver),
		Box::new(p06::Solver),
		Box::new(p07::Solver),
		Box::new(p08::Solver),
		Box::new(p09::Solver),
	]
}