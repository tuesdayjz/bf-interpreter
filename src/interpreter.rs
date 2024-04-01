use crate::token::Token;
use std::io::{Read, Write};

pub struct Interpreter {
  code: Vec<Token>,
  pc: usize,
  memory: [u8; u8::MAX as usize + 1],
  ptr: usize,
}

impl Interpreter {
  pub fn new(code: Vec<Token>) -> Interpreter {
    Interpreter {
      code,
      pc: 0,
      memory: [0; u8::MAX as usize + 1],
      ptr: 0,
    }
  }

  pub fn run(mut self, input: &mut dyn Read, output: &mut dyn Write) {
    while self.pc < self.code.len() {
      match self.code[self.pc] {
        Token::IncPtr => self.ptr += 1,
        Token::DecPtr => self.ptr -= 1,
        Token::IncVal => self.memory[self.ptr] = self.memory[self.ptr].wrapping_add(1),
        Token::DecVal => self.memory[self.ptr] = self.memory[self.ptr].wrapping_sub(1),
        Token::Out => {
          output.write(&[self.memory[self.ptr]]).unwrap();
        }
        Token::In => {
          let mut buf = [0; 1];
          input.take(1).read(&mut buf).unwrap();
          self.memory[self.ptr] = buf[0];
        }
        Token::LoopStart => {
          if self.memory[self.ptr] == 0 {
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
          if self.memory[self.ptr] != 0 {
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
