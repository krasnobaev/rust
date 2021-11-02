use std::collections::HashMap;

trait BrainLuckCommands {
  fn incdp(&mut self) -> &Self;
  fn decdp(&mut self) -> &Self;
  fn incmm(&mut self) -> &Self;
  fn decmm(&mut self) -> &Self;
  fn readch(&mut self) -> &Self;
  fn writch(&mut self) -> &Self;
  fn rjump(&mut self) -> &Self;
  fn ljump(&mut self) -> &Self;
}

impl BrainLuckCommands for BrainLuckInterpreter {
  fn incdp(&mut self) -> &BrainLuckInterpreter {
    self.dp += 1;
    self
  }

  fn decdp(&mut self) -> &BrainLuckInterpreter {
    self.dp -= 1;
    self
  }

  fn incmm(&mut self) -> &BrainLuckInterpreter {
    self.memory[self.dp] += 1;
    self
  }

  fn decmm(&mut self) -> &BrainLuckInterpreter {
    self.memory[self.dp] -= 1;
    self
  }

  fn readch(&mut self) -> &BrainLuckInterpreter {
    self.memory[self.dp] = match self.in_stream.pop() {
      Some(v) => v,
      None => 0,
    };
    self
  }

  fn writch(&mut self) -> &BrainLuckInterpreter {
    self.out_stream.push(self.memory[self.dp]);
    self
  }

  fn rjump(&mut self) -> &BrainLuckInterpreter {
    if self.memory[self.dp] == 0 {
      self.cp = self.lookup[&self.cp];
    }
    self
  }

  fn ljump(&mut self) -> &BrainLuckInterpreter {
    if self.memory[self.dp] != 0 {
      self.cp = self.lookup[&self.cp];
    }
    self
  }
}

pub struct BrainLuckInterpreter {
  cp: usize,           // current operation
  dp: usize,           // current memory pointer
  code: Vec<String>,
  _code: Vec<dyn FnMut(BrainLuckInterpreter) -> BrainLuckInterpreter>,
  memory: Vec<u8>,
  lookup: HashMap<usize,usize>,
  in_stream: Vec<u8>,
  out_stream: Vec<u8>,
}

impl BrainLuckInterpreter {
  pub fn new() -> BrainLuckInterpreter {
    BrainLuckInterpreter {
      cp: 0,
      dp: 0,
      code: Vec::new(),
      memory: Vec::with_capacity(5000),
      lookup: HashMap::new(),
      in_stream: Vec::new(),
      out_stream: Vec::new(),
    }
  }

  pub fn parse(mut self, code: &str) -> BrainLuckInterpreter {
    code.chars().for_each(|x| {
      self.code.push(String::from(match x {
        '>' => { "incdp" },
        '<' => { "decdp" },
        '+' => { "incmm" },
        '-' => { "decmm" },
        ',' => { "readch" },
        '.' => { "writch" },
        '[' => { "rjump" },
        ']' => { "ljump" },
        _ => { panic!("Unknown operation") },
      }));
    });

    let mut stack: Vec<usize> = Vec::new();
    for (i,ch) in code.chars().enumerate() {
      match ch {
        '[' => stack.push(i),
        ']' => match stack.pop() {
          Some(v) => { self.lookup.entry(v).or_insert(i); },
          None => panic!("unmatched parenthesis"),
        },
        _ => {},
      };
    }

    println!("parsed program {:?}", self.code);
    println!("lookup table {:?}", self.lookup);

    self
  }

  pub fn run(mut self, input: Vec<u8>) -> Vec<u8> {
    self.in_stream = input;

    self.code.iter().fold(self, |acc, op| self.code[op]()).collect().out_stream
  }
}

fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
  let itp = BrainLuckInterpreter::new();
  itp.parse(code).run(input)
}

fn main() { }
