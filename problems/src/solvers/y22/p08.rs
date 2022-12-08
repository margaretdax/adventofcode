use utils::*;

pub struct Solver;

impl Solution for Solver {
    fn get_number(self: &Self) -> i32 { 8 }

	fn solve(self: &Self, lines: Vec<String>) -> Option<String> {
		let mut solution = "".to_owned();
		let mut answer;
		let trees = collect_trees(&lines);
		let mut count = 0;
		//let mut debug = "".to_owned();
		// for h in 0..trees.len() {
		// 	for w in 0..trees[h].len() {
		// 		//print!("{}", trees[h][w]);
		// 	}
		// 	//print!("\n");
		// }
		for h in 0..trees.len() {
			for w in 0..trees[h].len() {
				let vis = is_visible((w, h), &trees);
				if vis {
					//debug.push('v');
					count += 1;
					//println!("({},{}) visible\n", w, h);
				} else {
					//debug.push('h');
					//println!("({},{}) not visible\n", w, h);
				}
			}
			//debug.push('\n');
			//print!("=========\n");
		}
		//println!("{}", debug);
		answer = format!("{}", count);
		solution.push_str(format!("Part I: {}\n", answer).as_str());

		answer.clear();
		count = 0;
		for h in 0..trees.len() {
			for w in 0..trees[h].len() {
				let score = scenic_score((w, h), &trees);
				if score > count {
					count = score;
				}
			}
			//println!();
		}
		answer = format!("{}", count);
		solution.push_str(format!("Part II: {}\n", answer).as_str());
		
		Some(solution)
		//None
	}
}

fn collect_trees(lines: &Vec<String>) -> Vec<Vec<usize>> {
	let mut result = vec![];
	for line in lines.iter() {
		result.push(line.as_bytes().iter().map(|b| (b - b'0').into() ).collect());
	}
	result
}

fn scenic_score(p: (usize, usize), trees: &Vec<Vec<usize>>) -> usize {
	let p_h = p.1;
	let p_w = p.0;
	let can = trees[p_h][p_w].clone();
	let h_t = 0;
	let h_b = trees.len() - 1;
	let w_l = 0;
	let w_r = trees[0].len() - 1;

	//let mut debug = "".to_owned();

	let mut top = 0;
	// ph to ht, starts at ph+1 so ht..ph
	//debug.push('t');
	for h in (h_t..p_h).rev() {
		top += 1;
		let test = trees[h][p_w];
		//debug.push_str(format!("{}", test).as_str());
		if test >= can {
			//debug.push('|');
			break;
		}
	}

	let mut bottom = 0;
	//debug.push('b');
	for h in p_h+1..=h_b {
		bottom += 1;
		let test = trees[h][p_w];
		//debug.push_str(format!("{}", test).as_str());
		if test >= can {
			//debug.push('|');
			break;
		}
	}

	let mut left = 0;
	//debug.push('l');
	for w in (w_l..p_w).rev() {
		left += 1;
		let test = trees[p_h][w];
		//debug.push_str(format!("{}", test).as_str());
		if test >= can {
			//debug.push('|');
			break;
		}
	}

	let mut right = 0;
	//debug.push('r');
	for w in p_w+1..=w_r {
		right += 1;
		let test = trees[p_h][w];
		//debug.push_str(format!("{}", test).as_str());
		if test >= can {
			//debug.push('|');
			break;
		}
	}

	let score = left * right * top * bottom;

	//println!("({},{}) debug from {} {}", p_w, p_h, can, debug);
	//println!("({},{}) score is {} (t: {}, b: {}, l: {}, r:{}", p_w, p_h, score, top, bottom, left, right);

	score
}

fn is_visible(p: (usize, usize), trees: &Vec<Vec<usize>>) -> bool {
	let p_h = p.1;
	let p_w = p.0;
	let can = trees[p_h][p_w].clone();
	let h_t = 0;
	let h_b = trees.len() - 1;
	let w_l = 0;
	let w_r = trees[0].len() - 1;
	if p_w == w_l || p_h == h_t || p_w == w_r || p_h == h_b {
		//println!("({},{}) visible from sides", p_w, p_h);
		return true;
	}

	// pls forgive my sins

	let mut visible = true;
	for h in h_t..p_h { // top to can
		if trees[h][p_w] >= can {
			//println!("({},{}) not visible from top, {} > {}", p_w, p_h, trees[h][p_w], can);
			visible = false;
			break;
		} else {
			//println!("({},{}) visible from top, {} < {}", p_w, p_h, trees[h][p_w], can);
		}
	}
	if visible {
		//print!("b");
		return true
	}

	visible = true;
	for h in (p_h+1..=h_b).rev() { // bot to can
		if trees[h][p_w] >= can {
			//println!("({},{}) not visible from bottom, {} > {}", p_w, p_h, trees[h][p_w], can);
			visible = false;
			break;
		} else {
			//println!("({},{}) visible from bottom, {} < {}", p_w, p_h, trees[h][p_w], can);
		}
	}
	if visible {
		//print!("c");
		return true
	}

	visible = true;
	for w in w_l..p_w { // left to can
		if trees[p_h][w] >= can {
			//println!("({},{}) not visible from left, {} > {}", p_w, p_h, trees[p_h][w], can);
			visible = false;
			break;
		} else {
			//println!("({},{}) visible from left, {} < {}", p_w, p_h, trees[p_h][w], can);
		}
	}
	if visible {
		//print!("d");
		return true
	}

	visible = true;
	for w in (p_w+1..=w_r).rev() { // left to can
		if trees[p_h][w] >= can {
			//println!("({},{}) not visible from right, {} > {}", p_w, p_h, trees[p_h][w], can);
			visible = false;
			break;
		} else {
			//println!("({},{}) visible from right, {} < {}", p_w, p_h, trees[p_h][w], can);
		}
	}
	if visible {
		//print!("e");
		return true
	}

	false
}