use std::fs;

fn main() {
  let contents =
    fs::read_to_string("dec2/input/strategy_guide.txt")
      .expect("file should be readable");

  let rounds: Vec<&str> =
    contents.trim().split('\n').collect();

  let mut total_score: i32 = 0;
  for round in rounds {
    let choices: Vec<&str> = round.split(' ').collect();
    total_score += score(
      choices[0],
      choices[1],
    )
  }

  println!("Total score is {}", total_score)
}

// Score accepts the round results for each player and outputs the resulting
// score.
fn score(opponent: &str, me: &str) -> i32 {
  // 1. what is my choice worth?
  let choice_score = match me {
    "X" => 1,
    "Y" => 2,
    "Z" => 3,
    _ => panic!("unknown player choice")
  };

  // 2. did i win or lose?
  let result_score = match (opponent, me) {
    ("A", "X") => 3,
    ("A", "Y") => 6,
    ("A", "Z") => 0,
    ("B", "X") => 0,
    ("B", "Y") => 3,
    ("B", "Z") => 6,
    ("C", "X") => 6,
    ("C", "Y") => 0,
    ("C", "Z") => 3,
    _ => panic!("unknown permutation")
  };

  // 3. what is my score?
  choice_score + result_score
}
