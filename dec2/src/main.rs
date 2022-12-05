use std::fs;

fn main() {
  let contents =
    fs::read_to_string("dec2/input/strategy_guide.txt")
      .expect("file should be readable");

  let rounds: Vec<Vec<&str>> =
    contents.trim()
      .split('\n')
      .map(|round| round.split(' ').collect())
      .collect();

  let part1_score: i32 =
    rounds.iter().map(|strategy| score_part1(strategy[0], strategy[1]))
      .sum();

  let part2_score: i32 =
    rounds.iter().map(|strategy| score_part2(strategy[0], strategy[1]))
      .sum();

  println!("part 1: Total score is {}", part1_score);
  println!("part 2: Total score is {}", part2_score);
}

// Given a symbol representing the opponent's choice ("A", "B", or "C") and my
// own choice ("X", "Y", or "Z"), determine my overall score for this round. The
// round score formula is the sum of the point value of my choice and the point
// value of whether I won, lost or tied.
fn score_part1(opponent: &str, me: &str) -> i32 {
  // 1. what is my choice worth?
  let choice_score = choice_score(me);

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

// Given a symbol representing the opponent's choice ("A", "B", or "C") and
// a symbol representing whether I need to lose ("X"), tie ("Y"), or win ("Z"),
// determine my overall score for this round. The round score formula is the
// sum of the point value of my choice and the point value of whether I lost,
// tied, or won.
fn score_part2(opponent: &str, target: &str) -> i32 {
  match target {
    "X" /* lose */ => choice_score(losing_choice(opponent)),
    "Y" /* tie */ => choice_score(tie_choice(opponent)) + 3,
    "Z" /* win */ => choice_score(winning_choice(opponent)) + 6,
    _ => panic!("unknown target state")
  }
}

fn choice_score(choice: &str) -> i32 {
  match choice {
    "X" => 1,
    "Y" => 2,
    "Z" => 3,
    _ => panic!("unknown choice")
  }
}

fn winning_choice(opponent: &str) -> &str {
  match opponent {
    "A" => "Y",
    "B" => "Z",
    "C" => "X",
    _ => panic!("unknown opponent choice")
  }
}

fn losing_choice(opponent: &str) -> &str {
  match opponent {
    "A" => "Z",
    "B" => "X",
    "C" => "Y",
    _ => panic!("unknown opponent choice")
  }
}

fn tie_choice(opponent: &str) -> &str {
  match opponent {
    "A" => "X",
    "B" => "Y",
    "C" => "Z",
    _ => panic!("unknown opponent choice")
  }
}
