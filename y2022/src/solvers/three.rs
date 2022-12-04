use utils::*;

pub struct Solver;

impl Solution for Solver {
    fn solve(self: Box<Self>, lines: Vec<String>) -> String {
        let mut solution = "".to_owned();
        let mut score = 0;
        
        for line in lines.iter() {
            let (left, right) = split_sack(line);
            let common = left.iter().filter(|c| right.contains(c)).next().unwrap();
            score += priority(common);
        }
        solution.push_str(format!("Part I: {}\n", score).as_str());

        score = 0;
        for chunk in lines.chunks(3) {
            let a = chunk[0].as_bytes();
            let b = chunk[1].as_bytes();
            let c = chunk[2].as_bytes();
            
            let common = a.iter().filter(|x| b.contains(x) && c.contains(x)).next().unwrap();

            score += priority(common);
        }

        solution.push_str(format!("Part II: {}\n", score).as_str());

        solution
    }
}

fn split_sack(line: &String) -> (Vec<u8>, Vec<u8>) {
    let index = line.len() / 2;
    let (left, right) = line.split_at(index);
    if left.len() != right.len() {
        panic!("halfing is borked");
    }

    (left.as_bytes().to_vec(), right.as_bytes().to_vec())
}

fn priority(item: &u8) -> i32 {
    if *item >= b'a' &&  *item <= b'z' {
        (item - b'a' + 1).into()
    } else if *item >= b'A' && *item <= b'Z' {
        (item - b'A' + 27).into()
    } else {
        panic!("invalid item")
    }
}