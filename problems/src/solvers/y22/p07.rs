use utils::*;
use std::collections::HashMap;

pub struct Solver;

impl Solution for Solver {
    fn get_number(self: &Self) -> i32 { 7 }

	fn solve(self: &Self, lines: Vec<String>) -> Option<String> {
		let mut solution = "".to_owned();
		let mut answer = "".to_owned();
		{
			let files = collect_files(&lines);
			let mut dirs = std::collections::HashMap::new();
			for file in files {
				let mut parts: Vec<&str> = file.path.split("/").collect();
				parts.pop(); // don't need the file name
				loop {
					let key = parts.join("/");
					let entry = dirs.entry(key);
					entry.and_modify(|e| *e += file.size).or_insert(file.size);

					if parts.is_empty() {
						break;
					} else { 
						parts.pop();
					}
				}
				println!("{}\t{}", file.size, file.path);
			}
			for (k, v) in dirs.iter() {
				println!("{}: {} [{}]", k, v, *v <= 100000);
			}
			answer = format!("{}", dirs.iter().map(|(k,v)| v).filter(|&v| *v <= 100000).sum::<u64>())
		}
		solution.push_str(format!("Part I: {}\n", answer).as_str());

		answer.clear();
		for _line in lines.iter() {

		}
		solution.push_str(format!("Part II: {}\n", answer).as_str());
		
		Some(solution)
	}
}

struct File {
	path: String,
	size: u64,
}

fn collect_files(lines: &Vec<String>) -> Vec<File> {
	let mut result = vec![];

	let mut iter = lines.iter();
	let mut path: Vec<String> = vec![];
	loop {
		match iter.next() {
			Some(l) => {
				let mut parts = l.split(" ");
				match parts.next() {
					Some(a) => {
						match a {
							"$" => { // is a command
								match parts.next() {
									Some(c) => {
										match c {
											"cd" => {
												match parts.next() {
													Some(b) => {
														match b {
															".." => {
																path.pop();
															}
															"/" => {
																path.clear();
															}
															_ => {
																path.push(b.to_owned());
															}
														}
													}
													None => panic!("invalid cd param \"{}\"", l)
												}
											}
											"ls" => {
												continue
											}
											_ => panic!("unknown command {}", c)
										}
									}
									None => panic!("waat? {}", l)
								}
							}
							"dir" => { }
							_ => {
								let b = parts.next().unwrap();
								let pj = if path.is_empty() {
									"/".to_owned()
								} else {
									format!("/{}/", path.join("/"))
								};
								let f: File = File { path: format!("{}{}", pj, b), size: a.parse::<u64>().unwrap()};
								result.push(f);
							}
						}
					}
					None => panic!("wat?")
				}
			}
			None => break
		};
	}

	result
}