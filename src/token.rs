pub enum Token {
  IncPtr,
  DecPtr,
  IncVal,
  DecVal,
  Out,
  In,
  LoopStart,
  LoopEnd,
}

pub fn tokenize(input: &str) -> Vec<Token> {
  input.chars().filter_map(|c| match c {
    '>' => Some(Token::IncPtr),
    '<' => Some(Token::DecPtr),
    '+' => Some(Token::IncVal),
    '-' => Some(Token::DecVal),
    '.' => Some(Token::Out),
    ',' => Some(Token::In),
    '[' => Some(Token::LoopStart),
    ']' => Some(Token::LoopEnd),
    _ => None,
  }).collect()
}
