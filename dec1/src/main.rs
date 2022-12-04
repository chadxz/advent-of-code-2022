use std::fs;
use itertools::Itertools;

fn main() {
  let top3_elves: Vec<i32> =
    fs::read_to_string("dec1/input/calories.txt")
      .expect("file should be readable")
      .trim()
      .split("\n\n")
      .map(|elf| {
        elf.split('\n').fold(0, |acc, calories| {
          let c: i32 =
            calories.trim().parse().expect("calories should be a number");
          acc + c
        })
      })
      .sorted()
      .rev()
      .take(3)
      .collect();

  println!("part 1: the most caloric elf has {} calories.",
         top3_elves.first().expect("there should be a first elf"));

  println!("part 2: the top 3 most caloric elves have {} calories.",
        top3_elves.into_iter().sum::<i32>())
}
