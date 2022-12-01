use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut elf_rations: Vec<i32> = vec![0];

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        if line.is_empty() {
            elf_rations.push(0);
            continue;
        }

        let meal: i32= line.parse().unwrap();
        let i = elf_rations.len() - 1;
        elf_rations[i] += meal;
    }

    let max = elf_rations.iter().max().unwrap();
    println!("{}", max);
}
