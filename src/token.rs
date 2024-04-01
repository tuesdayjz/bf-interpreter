pub enum Token {
  IncrementPtr,
  DecrementPtr,
  IncrementVal,
  DecrementVal,
  Out,
  In,
  LoopStart,
  LoopEnd,
}

pub fn tokenize(input: &str) -> Vec<Token> {
  input.chars().filter_map(|c| match c {
    '>' => Some(Token::IncrementPtr),
    '<' => Some(Token::DecrementPtr),
    '+' => Some(Token::IncrementVal),
    '-' => Some(Token::DecrementVal),
    '.' => Some(Token::Out),
    ',' => Some(Token::In),
    '[' => Some(Token::LoopStart),
    ']' => Some(Token::LoopEnd),
    _ => None,
  }).collect()
}
