pub struct DayFour;

impl utils::Solution for DayFour {
	fn solve(self: Box<Self>, lines: Vec<String>) -> String {
		let mut solution = "".to_owned();
		let mut pairs = 0;
		for line in lines.iter() {
			let pair : Vec<&str> = line.split(',').collect();
			let left = pair[0];
			let right = pair[1];
			let left_range : Vec<&str> = left.split('-').collect();
			let right_range : Vec<&str> = right.split('-').collect();
			let al : i32 = left_range[0].parse::<i32>().unwrap();
			let ar : i32 = left_range[1].parse::<i32>().unwrap();
			let bl : i32 = right_range[0].parse::<i32>().unwrap();
			let br : i32 = right_range[1].parse::<i32>().unwrap();
			// al-ar,bl-br
			// al >= bl && br <= ar
			// bl >= al && ar <= br
			// 
			

			if al <= bl && br <= ar {
				println!("A al {} <= bl {} && br {} <= ar {}", al, bl, br, ar);
				pairs += 1;
			} else if bl <= al && ar <= br {
				println!("B bl {} <= al {} && ar {} <= br {}", bl, al, ar, br);
				pairs += 1;
			}
		}
		solution.push_str(format!("Part I: {}\n", pairs).as_str());

		solution
	}
}

