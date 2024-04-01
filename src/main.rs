mod token;
mod interpreter;

use interpreter::Interpreter;
use token::tokenize;

fn main() {
  let code = include_str!("../examples/mandelbrot.bf");
  let code = tokenize(code);
  let interpreter = Interpreter::new(code);
  let stdin = std::io::stdin();
  let stdout = std::io::stdout();
  let mut stdin = stdin.lock();
  let mut stdout = stdout.lock();
  interpreter.run(&mut stdin, &mut stdout);
}
