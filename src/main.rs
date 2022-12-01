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

    elf_rations.sort();
    let sum: i32 = elf_rations.iter().rev().take(3).sum();

    println!("{}", sum);
}
