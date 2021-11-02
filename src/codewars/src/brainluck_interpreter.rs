use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
enum BrainLuckCommand {
  IncDp,
  DecDp,
  IncMm,
  DecMm,
  ReadCh,
  WritCh,
  RJump,
  LJump,
}

impl BrainLuckCommand {
  fn parse(x: char) -> BrainLuckCommand {
    match x {
      '>' => { BrainLuckCommand::IncDp },
      '<' => { BrainLuckCommand::DecDp },
      '+' => { BrainLuckCommand::IncMm },
      '-' => { BrainLuckCommand::DecMm },
      ',' => { BrainLuckCommand::ReadCh },
      '.' => { BrainLuckCommand::WritCh },
      '[' => { BrainLuckCommand::RJump },
      ']' => { BrainLuckCommand::LJump },
      _ => { panic!("Unknown operator") },
    }
  }
}

pub struct Interpreter {
  cc: usize,           // command count
  cp: usize,           // current operation
  code: Vec<BrainLuckCommand>,
  dp: usize,           // current memory pointer
  memory: Vec<u8>,
  leftlookup: HashMap<usize,usize>,
  rightlookup: HashMap<usize,usize>,
  in_stream: Vec<u8>,
  out_stream: Vec<u8>,
}

impl Interpreter {
  pub fn new() -> Interpreter {
    Interpreter {
      cc: 0,
      cp: 0,
      dp: 0,
      code: Vec::new(),
      memory: vec![0; 5000],
      leftlookup: HashMap::new(),
      rightlookup: HashMap::new(),
      in_stream: Vec::new(),
      out_stream: Vec::new(),
    }
  }

  pub fn parse(mut self, code: &str) -> Interpreter {
    code.chars().for_each(|x| {
      self.code.push(BrainLuckCommand::parse(x));
    });

    let mut stack: Vec<usize> = Vec::new();
    for (i,ch) in code.chars().enumerate() {
      match ch {
        '[' => stack.push(i),
        ']' => match stack.pop() {
          Some(v) => {
            self.leftlookup.entry(v).or_insert(i);
            self.rightlookup.entry(i).or_insert(v);
          },
          None => { },
        },
        _ => { },
      };
    }

    println!("parsed program {:?}", self.code);
    println!("lookup table {:?}", self.leftlookup);

    self
  }

  pub fn run(mut self, input: Vec<u8>) -> Vec<u8> {
    self.in_stream = input;
    println!("in_stream={:?}", self.in_stream);
    let mut in_stream_iter = self.in_stream.iter();

    while self.cp < self.code.len() {
      let cmd = self.code[self.cp];
      println!("#{}.invoking {:?}", self.cc, cmd);

      match cmd {
        BrainLuckCommand::IncDp => { self.dp += 1; },
        BrainLuckCommand::DecDp => { self.dp -= 1; },
        BrainLuckCommand::IncMm => {
          self.memory[self.dp] = match self.memory[self.dp].checked_add(1) {
            Some(v) => v,
            None => 0
          };
        },
        BrainLuckCommand::DecMm => { self.memory[self.dp] -= 1; },
        BrainLuckCommand::ReadCh => {
          self.memory[self.dp] = match in_stream_iter.next() {
            Some(v) => *v,
            None => 0,
          };
        },
        BrainLuckCommand::WritCh => {
          self.out_stream.push(self.memory[self.dp]);
        },
        BrainLuckCommand::RJump => {
          if self.memory[self.dp] == 0 {
            println!("  JUMP");
            self.cp = self.leftlookup[&self.cp];
          }
        },
        BrainLuckCommand::LJump => {
          if self.memory[self.dp] != 0 {
            println!("  JUMP");
            self.cp = self.rightlookup[&self.cp];
          }
        },
      };

      println!("  status: dp={} cp={} out_stream={:?}", self.cp, self.cp, self.out_stream);
      println!("  memory: {:?}", &self.memory[0..10]);
      println!("  memory: {:?}", self.memory[0..10].iter().fold(String::new(), |s, x| {
          s + &(char::from(*x)).to_string()
      }));

      self.cp += 1;
      self.cc += 1;

      if self.cc > 10000 {
        println!("timeout after {} operations", self.cc);
        break;
      }
    };

    self.out_stream
  }

  pub fn get_out_stream(self) -> Vec<u8> {
    Vec::new()
  }
}

fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
  let itp = Interpreter::new();
  itp.parse(code).run(input)
}

fn main() {}

#[test]
fn example_tests() {
  // assert_eq!(String::from_utf8(brain_luck("+-+", ez_vec("", 255))).unwrap(), "Codewars");

  // Echo until byte 255 encountered
  // assert_eq!(String::from_utf8(brain_luck(",+[-.,+]", ez_vec("Codewars", 255))).unwrap(), "Codewars");
  // Echo until byte 0 encountered
  // assert_eq!(String::from_utf8(brain_luck(",[.[-],]", ez_vec("Codewars", 0))).unwrap(), "Codewars");
  // Multiply two numbers
  assert_eq!(brain_luck(",>,<[>[->+>+<<]>>[-<<+>>]<<<-]>>.", vec![8, 9]), vec![72]);
}
