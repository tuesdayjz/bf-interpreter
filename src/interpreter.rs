use crate::token::Token;
use std::io::{Read, Write};

pub struct Interpreter {
  code: Vec<Token>,
  pc: usize,
  memory: [u8; u8::MAX as usize + 1],
  pointer: usize,
}

impl Interpreter {
  pub fn new(code: Vec<Token>) -> Interpreter {
    Interpreter {
      code,
      pc: 0,
      memory: [0; u8::MAX as usize + 1],
      pointer: 0,
    }
  }

  pub fn run(mut self, input: &mut dyn Read, output: &mut dyn Write) {
    while self.pc < self.code.len() {
      match self.code[self.pc] {
        Token::IncrementPtr => self.pointer += 1,
        Token::DecrementPtr => self.pointer -= 1,
        Token::IncrementVal => self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1),
        Token::DecrementVal => self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1),
        Token::Out => {
          output.write(&[self.memory[self.pointer]]).unwrap();
        }
        Token::In => {
          let mut buf = [0; 1];
          input.take(1).read(&mut buf).unwrap();
          self.memory[self.pointer] = buf[0];
        }
        Token::LoopStart => {
          if self.memory[self.pointer] == 0 {
            let mut depth = 1;
            while depth != 0 {
              self.pc += 1;
              match self.code[self.pc] {
                Token::LoopStart => depth += 1,
                Token::LoopEnd => depth -= 1,
                _ => {}
              }
            }
          }
        }
        Token::LoopEnd => {
          if self.memory[self.pointer] != 0 {
            let mut depth = 1;
            while depth != 0 {
              self.pc -= 1;
              match self.code[self.pc] {
                Token::LoopStart => depth -= 1,
                Token::LoopEnd => depth += 1,
                _ => {}
              }
            }
          }
        }
      }
      self.pc += 1;
    }
  }
}
