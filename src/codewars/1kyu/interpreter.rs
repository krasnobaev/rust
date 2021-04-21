use std::fmt;
use std::collections::HashMap;

trait Value {
  fn get_value(self) -> f32;
  fn get_value_byref(&self) -> f32;
  fn add(self, v: f32) -> f32;
  fn sub(self, v: f32) -> f32;
  fn div(self, v: f32) -> f32;
  fn mul(self, v: f32) -> f32;
}

impl fmt::Debug for Box<dyn Value> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Value")
      .field("v", &self.get_value_byref())
      .finish()
  }
}

#[derive(Copy,Clone,Debug)]
struct Cnst {
  value: f32
}

impl Value for Cnst {
  fn get_value(self) -> f32 { self.value }
  fn get_value_byref(&self) -> f32 { self.value }
  fn add(self, v: f32) -> f32 { self.value + v }
  fn sub(self, v: f32) -> f32 { self.value + v }
  fn div(self, v: f32) -> f32 { self.value + v }
  fn mul(self, v: f32) -> f32 { self.value + v }
}

impl Cnst {
  fn new(v: String) -> Cnst {
    Cnst {
      value: v.parse().unwrap(),
    }
  }
}

#[derive(Copy,Clone,Debug)]
struct Var {
  name: char,
  value: Option<f32>,
}

impl Value for Var {
  fn get_value(self) -> f32 {
    match self.value {
      Some(v) => v,
      None => panic!("variable {} is not initialized", self.name),
    }
  }
  fn get_value_byref(&self) -> f32 {
    match self.value {
      Some(v) => v,
      None => panic!("variable {} is not initialized", self.name),
    }
  }
  fn add(self, v: f32) -> f32 { self.get_value() + v }
  fn sub(self, v: f32) -> f32 { self.get_value() + v }
  fn div(self, v: f32) -> f32 { self.get_value() + v }
  fn mul(self, v: f32) -> f32 { self.get_value() + v }
}

impl Var {
  fn new(name: String) -> Var {
    Var {
      name: name.chars().nth(0).unwrap(),
      value: None,
    }
  }

  fn set_value(mut self, v: f32) {
    println!("var {:?} changed value from {:?} to {}", self.name, self.value, v);
    self.value = Some(v);
    println!("changed val {:?}", self);
  }
}

#[derive(Clone,Copy,Debug,PartialEq)]
enum Operation {
  Add = 10,
  Sub = 11,
  Mul = 20,
  Div = 21,
  Mod = 22,
  LPar = 91,
  RPar = 92,
  FnDef = 93,
  Assign = 94,
}

impl Operation {
  fn new(x: char) -> Operation {
    match x {
      '+' => Operation::Add,
      '-' => Operation::Sub,
      '*' => Operation::Mul,
      '/' => Operation::Div,
      '%' => Operation::Mod,
      '(' => Operation::LPar,
      ')' => Operation::RPar,
      '⇒' => Operation::FnDef,
      '=' => Operation::Assign,
      _ => panic!("unknown operation symbol"),
    }
  }

  fn invoke(self, a: f32, b: f32) -> Option<f32> {
    match self {
      Operation::Add => Some(a+b),
      Operation::Sub => Some(a-b),
      Operation::Mul => Some(a*b),
      Operation::Div => Some(a/b),
      Operation::Mod => Some(a%b),
      _ => panic!("uninvokable operation"),
    }
  }

  fn is_left_ass(self) -> bool {
    true
  }
}

#[derive(Clone,Copy,Debug)]
enum Token {
  Const(Cnst),
  Variable(Var),
  Operation(Operation),
}

struct Interpreter {
  data: Vec<Token>,
  ops: Vec<Operation>,
  vars: HashMap<char, Var>,

  chbuf: String,
}

impl Interpreter {
  fn new() -> Interpreter {
    Interpreter {
      data: Vec::new(),
      ops: Vec::new(),
      vars: HashMap::new(),

      chbuf: String::new(),
    }
  }

  fn parse(&mut self, input: &str) {
    let mut nmbuf = String::new();
    let mut eqbuf = String::new();

    for x in input.chars() {
      match x {
        '+' | '-' | '*' | '/' | '%' => {
          if nmbuf.len() > 0 {
            self.data.push(Token::Const(Cnst::new(nmbuf)));
            nmbuf = String::new();
          }

          self.match_operation(Operation::new(x));
        },
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
          nmbuf.push(x);
        },
        ' ' => {
          let chop;
          match eqbuf {
            v if v == "=>" => chop = '⇒',
            v if v == "=" =>  chop = '=',
            _ => continue,
          }
          self.data.push(Token::Operation(Operation::new(chop)));
          eqbuf = String::new();

          if nmbuf.len() > 0 {
            self.data.push(Token::Const(Cnst::new(nmbuf)));
            nmbuf = String::new();
          }
          self.match_chbuf();
        },

        '=' => eqbuf.push(x),
        '>' => eqbuf.push(x),
        '(' => self.ops.push(Operation::new(x)),
        ')' => {
          if nmbuf.len() > 0 {
            self.data.push(Token::Const(Cnst::new(nmbuf)));
            nmbuf = String::new();
          }

          while let maybeop = self.ops.last() {
            match maybeop {
              Some(op) => match op {
                Operation::LPar => {
                  self.ops.pop();
                  break;
                },
                _ => self.data.push(Token::Operation(self.ops.pop().unwrap())),
              },
              None => panic!("unmatched left bracket"),
            }
          }
        },

        'a'..='z' => {
          self.chbuf.push(x);
        }
        _ => panic!("unexpected character `{}`", x),
      }
    }

    if nmbuf.len() > 0 {
      self.data.push(Token::Const(Cnst::new(nmbuf)));
    }

    while let Some(op) = self.ops.pop() {
      self.data.push(Token::Operation(op));
    }

    if self.ops.len() > 0 {
      match self.ops.pop() {
        Some(v) => self.data.push(Token::Operation(v)),
        None => panic!("Unexpected end of operators stack"),
      }
    }

    assert_eq!(self.ops.len(), 0);
  }

  fn match_operation(&mut self, op: Operation) {
    match self.ops.last() {
      Some(top) => {
        if (
          (*top as i32 / 10 > op as i32 / 10) ||
          (*top as i32 / 10 == op as i32 / 10 && op.is_left_ass())
        ) && *top != Operation::LPar {
          match self.ops.pop() {
            Some(v) => self.data.push(Token::Operation(v)),
            None => panic!("Unexpected end of operators stack"),
          }
        }
      },
      _ => { }
    }
    self.ops.push(op);

    println!("  data stack {:?}", self.data);
    println!("  op stack {:?}", self.ops);
  }

  fn match_chbuf(&mut self) {
    match &self.chbuf {
      x if x == "fn" => unimplemented!(),
      x if x == "avg" => unimplemented!(),
      x if x.len() > 0 => {
        self.data.push(Token::Variable(Var::new(x.to_string())));
        self.chbuf = String::new();
      }
      _ => { },
    }
  }

  fn calc(&mut self) -> Result<Option<f32>, String> {
    let mut stack: Vec<f32> = Vec::new();
    let mut assign_var: Option<char> = None;

    println!("calc start");

    for v in &self.data {
      println!("matching {:?}", v);

      match v {
        Token::Const(x) => stack.push(x.get_value()),
        Token::Variable(x) => {
          assign_var = Some(x.name);

          if let Some(v) = assign_var {
            if let Some(x) = self.vars.get(&v) {
              stack.push(x.get_value());
            }
          }
        },
        Token::Operation(op) => {
          match op {
            Operation::Assign => {
              stack.pop();
            },
            Operation::FnDef => unimplemented!(),
            _ => {
              let b = stack.pop().unwrap();
              let a = stack.pop().unwrap();
              match op.invoke(a, b) {
                Some(x) => stack.push(x),
                _ => panic!("nothing to put onto stack"),
              }
            },
          }
        },
      }

      println!("  mid state:");
      println!("    {} ops: {:?}", self.ops.len(), self.ops);
      println!("    {} stack: {:?}", stack.len(), stack);
      // println!("      vars: {:?}", self.vars);
    }

    self.data = Vec::new();

    println!("{}", stack.len());
    match stack.pop() {
      Some(v) => {
        println!("calc end with result: {:?}\n", v);
        if let Some(name) = assign_var {
          match self.vars.get(&name) {
            Some(x) => x.set_value(v),
            None => {
              let var = self.vars.entry(name).or_insert(Var::new(name.to_string()));
              var.set_value(v);
              println!("  val {:?} old_val {:?}", var, v);
            },
          }
        }
        Ok(Some(v))
      },
      None => Err("empty result".to_string()),
    }
  }

  fn input(&mut self, input: &str) -> Result<Option<f32>, String> {
    println!("input: {:?}", input);
    self.parse(input);
    println!("start state:");
    println!("  {} data: {:?}", self.data.len(), self.data);
    println!("  {} ops: {:?}", self.ops.len(), self.ops);
    println!("    vars: {:?}", self.vars);
    self.calc()
  }
}

fn main() {
  let mut i = Interpreter::new();
  i.input("1 + 1").map_err(|err| println!("{:?}", err)).ok();
}

#[cfg(test)]
pub mod tests {
  use::*;

  #[test]
  fn basic_arithmetic() {
      let mut i = Interpreter::new();
      assert_eq!(i.input("1 + 1"), Ok(Some(2.0)));
      assert_eq!(i.input("2 - 1"), Ok(Some(1.0)));
      assert_eq!(i.input("2 * 3"), Ok(Some(6.0)));
      assert_eq!(i.input("8 / 4"), Ok(Some(2.0)));
      assert_eq!(i.input("7 % 4"), Ok(Some(3.0)));
  }

  #[test]
  fn basic_arithmetic_ternar() {
    let mut i = Interpreter::new();
    assert_eq!(i.input("1 + 1 + 1 + 1 + 1 + 1"), Ok(Some(6.0)));
    assert_eq!(i.input("1 + 1 * 3"), Ok(Some(4.0)));
    assert_eq!(i.input("1 * 1 + 3"), Ok(Some(4.0)));
    assert_eq!(i.input("2 + 2 * 2"), Ok(Some(6.0)));
    assert_eq!(i.input("2 * 2 + 2"), Ok(Some(6.0)));
  }

  #[test]
  fn bracket_arithmetic() {
    let mut i = Interpreter::new();
    assert_eq!(i.input("(2 + 2) * 2"), Ok(Some(8.0)));
    assert_eq!(i.input("2 * (2 + 2)"), Ok(Some(8.0)));
    assert_eq!(i.input("(2 * 2) + 2"), Ok(Some(6.0)));
    assert_eq!(i.input("(1 + 2) * (3 + 4)"), Ok(Some(21.0)));
    assert_eq!(i.input("(((((2+2)))))"), Ok(Some(4.0)));
  }

  #[test]
  fn variables() {
      let mut i = Interpreter::new();
      assert_eq!(i.input("x = 1"), Ok(Some(1.0)));
      assert_eq!(i.input("x"), Ok(Some(1.0)));
      assert_eq!(i.input("x + 3"), Ok(Some(4.0)));
      assert!(i.input("y").is_err());
  }

  #[test]
  fn functions() {
      let mut i = Interpreter::new();
      assert_eq!(i.input("fn avg x y => (x + y) / 2"), Ok(None));
      assert_eq!(i.input("avg 4 2"), Ok(Some(3.0)));
      assert!(i.input("avg 7").is_err());
      assert!(i.input("avg 7 2 4").is_err());
  }

  #[test]
  fn conflicts() {
      let mut i = Interpreter::new();
      assert_eq!(i.input("x = 1"), Ok(Some(1.0)));
      assert_eq!(i.input("fn avg x y => (x + y) / 2"), Ok(None));
      assert!(i.input("fn x => 0").is_err());
      assert!(i.input("avg = 5").is_err());
  }
}
