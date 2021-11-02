use std::collections::HashMap;

pub struct Runner {
  program: Vec<String>,
  result: String,
  registers: HashMap<String,i64>,
  labels: HashMap<String,usize>,
  stack: Vec<usize>,
  compareflag: Option<i8>,
}

impl Runner {
  pub fn new(input: &str) -> Runner {
    let mut labels = HashMap::new();
    let program = input
      .lines().map(|s| s.trim())
      .map(|s| {
        println!("trimming: {:?}", s);
        if s.starts_with("msg") {
          s.split(":::").collect::<Vec<_>>()
        } else {
          s.split(|x| x == ',' || x == ' ').filter(|ss| ss.len() > 0)
            .collect::<Vec<_>>()
        }
      })
      .map(|v| {
        println!("join: {:?}", v);
        v.join(
          match v.len() > 0 && v[0].starts_with("msg") {
            true => ",",
            false => " "
          }
        )
      })
      .map(|v| {
        println!("removing comments: {:?}", v);
        match v.find(';') {
          Some(i) => v.chars().take(i).collect::<String>(),
          None => v,
        }
      })
      .filter(|s| s.len() > 0)
      .enumerate().map(|(i,x)| {
        println!("making labels: {:?}", x);
        match x.find(':') {
          Some(v) => {
            labels.entry(x.chars().take(v).collect::<String>()).or_insert(i);
          },
          None => {}
        }; x
      })
      .collect();

      println!("refined program: {:?}", program);
      println!("with labels: {:?}", labels);

    Runner {
      program: program,
      result: String::new(),
      registers: HashMap::new(),
      labels: labels,
      stack: Vec::new(),
      compareflag: None,
    }
  }

  pub fn start(&mut self) -> Option<String> {
    println!("firing the engines");

    let mut i: usize = 0;
    while i < self.program.len() {
      let cmd = &self.program[i];
      let opr: Vec<_> = cmd.split(|x| x == ' ' || x == ',').collect();
      println!("state @{} = {:?}", i, self.registers);
      println!("cmd: {:?}", cmd);

      match opr[0] {
        "mov" => {
          let val = self.get_right(&opr);
          *self.registers.entry(String::from(opr[1])).or_insert(val) = val;
        },

        "add" => {
          let val = self.get_right(&opr);
          // println!("left {} is {} and right {} is {}", opr[1], self.registers[opr[1]], opr[2], val);
          self.registers.entry(String::from(opr[1])).and_modify(|v| *v += val);
        },
        "sub" => {
          let val = self.get_right(&opr);
          self.registers.entry(String::from(opr[1])).and_modify(|v| *v -= val);
        },
        "mul" => {
          let val = self.get_right(&opr);
          self.registers.entry(String::from(opr[1])).and_modify(|v| *v *= val);
        },
        "div" => {
          let val = self.get_right(&opr);
          self.registers.entry(String::from(opr[1])).and_modify(|v| *v /= val);
        },

        "inc" => {
          self.registers.entry(String::from(opr[1])).and_modify(|v| *v += 1);
        },
        "dec" => {
          self.registers.entry(String::from(opr[1])).and_modify(|v| *v -= 1);
        },

        "cmp" => {
          let val = self.get_right(&opr);
          self.compareflag = Some(self.compare(self.registers[opr[1]], val));
        },
        "jne" => {
          match self.compareflag {
            Some(v) => if v != 0 {i = *self.labels.get(opr[1]).unwrap()},
            _ => {},
          }
        },
        "je" => {
          match self.compareflag {
            Some(0) => i = *self.labels.get(opr[1]).unwrap(),
            _ => {},
          }
        },
        "jge" => {
          match self.compareflag {
            Some(0) | Some(1) => i = *self.labels.get(opr[1]).unwrap(),
            _ => {},
          }
        },
        "jg" => {
          match self.compareflag {
            Some(1) => i = *self.labels.get(opr[1]).unwrap(),
            _ => {},
          }
        },
        "jle" => {
          match self.compareflag {
            Some(-1) | Some(0) => i = *self.labels.get(opr[1]).unwrap(),
            _ => {},
          }
        },
        "jl" => {
          match self.compareflag {
            Some(-1) => i = *self.labels.get(opr[1]).unwrap(),
            _ => {},
          }
        },

        "jmp" => {
          match self.labels.get(opr[1]) {
            Some(v) => {
              i = *v;
              println!("jump to ({})", i);
            },
            None => return Some(format!("unknown label ({}); known labels are ({:?})", opr[1], self.labels))
          };
        },
        "call" => {
          self.stack.push(i);
          match self.labels.get(opr[1]) {
            Some(v) => {
              i = *v;
              println!("jump to ({})", i);
            },
            None => return Some(format!("unknown label ({}); known labels are ({:?})", opr[1], self.labels))
          };
        },
        "ret" => {
          match self.stack.pop() {
            Some(v) => i = v,
            None => return Some(String::from("unable to return from empty stack"))
          }
        },

        "msg" => {
          self.result = self.format_message(
            cmd.chars().skip(3).collect::<String>().trim()
          );
        },
        "end" => { return Some(self.result.to_string()) },
        _ => {
          println!("unknown operand ({})", opr[0]);
        }
      }

      i += 1;
    }

    println!("end reached with registers = {:?}", self.registers);
    None
  }

  fn format_message(&self, s: &str) -> String {
    println!("formatting (({:?}))", s);
    self.parse_msg(s).iter().map(|v| {
      // println!("searching for {} with size {}", v, v.len());
      if v.len() == 1 && v.chars().all(|x| x.is_alphabetic()) {
        let val = self.registers.get(v);
        match val {
          Some(_val) => _val.to_string(),
          None => format!("undefined var ({})", v),
        }
      } else {
        v.to_string()
      }
    }).collect()
  }

  fn parse_msg(&self, s: &str) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    let mut token: Vec<char> = Vec::new();
    let mut btokenfill = false;

    s.chars().for_each(|x| {
      match x {
        '\'' => {
          if btokenfill {
            ret.push(token.iter().fold(String::new(), |str, char| str + &char.to_string()));
            token = Vec::new();
            btokenfill = false;
          } else {
            token = Vec::new();
            btokenfill = true;
          }
        },
        'a'..='z' | '0'..='9' => {
          if btokenfill {
            token.push(x);
          } else {
            ret.push(x.to_string());
          }
        },
        _ => {
          if btokenfill {
            token.push(x);
          } else {
            // ignore
          }
        },
      }
    });

    println!("parsed msg: {:?}", ret);
    ret
  }

  fn get_right(&self, opr: &Vec<&str>) -> i64 {
    let val;
    if opr[2].chars().nth(0).unwrap().is_alphabetic() {
      val = self.registers[opr[2]];
    } else {
      val = opr[2].parse::<i64>().unwrap();
    }
    val
  }

  fn compare(&self, l: i64, r: i64) -> i8 {
    if l == r {
      0
    } else if l > r {
      1
    } else {
      -1
    }
  }

}

pub struct AssemblerInterpreter {
}

impl AssemblerInterpreter {
  pub fn interpret(input: &str) -> Option<String> {
    Runner::new(input).start()
  }

}

fn main() {}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
pub mod tests {
  use super::*;

  #[test]
  fn test1() {
    let program = "\n; My first program\nmov  a, 5\ninc  a\ncall function\nmsg  '(5+1)/2 = ', a    ; output message\nend\n\nfunction:\n    div  a, 2\n    ret\n";
    let expected = Some(String::from("(5+1)/2 = 3"));

    let actual = AssemblerInterpreter::interpret(program);
    assert_eq!(actual, expected);
  }

  #[test]
  fn test2() {
    let program = "\nmov   a, 5\nmov   b, a\nmov   c, a\ncall  proc_fact\ncall  print\nend\n\nproc_fact:\n    dec   b\n    mul   c, b\n    cmp   b, 1\n    jne   proc_fact\n    ret\n\nprint:\n    msg   a, '! = ', c ; output text\n    ret\n";
    let expected = Some(String::from("5! = 120"));

    let actual = AssemblerInterpreter::interpret(program);
    assert_eq!(actual, expected);
  }

  #[test]
  fn test3() {
    let program = "\nmov   a, 8            ; value\nmov   b, 0            ; next\nmov   c, 0            ; counter\nmov   d, 0            ; first\nmov   e, 1            ; second\ncall  proc_fib\ncall  print\nend\n\nproc_fib:\n    cmp   c, 2\n    jl    func_0\n    mov   b, d\n    add   b, e\n    mov   d, e\n    mov   e, b\n    inc   c\n    cmp   c, a\n    jle   proc_fib\n    ret\n\nfunc_0:\n    mov   b, c\n    inc   c\n    jmp   proc_fib\n\nprint:\n    msg   'Term ', a, ' of Fibonacci series is: ', b        ; output text\n    ret\n";
    let expected = Some(String::from("Term 8 of Fibonacci series is: 21"));

    let actual = AssemblerInterpreter::interpret(program);
    assert_eq!(actual, expected);
  }

  #[test]
  fn test4() {
    let program = "\nmov   a, 11           ; value1\nmov   b, 3            ; value2\ncall  mod_func\nmsg   'mod(', a, ', ', b, ') = ', d        ; output\nend\n\n; Mod function\nmod_func:\n    mov   c, a        ; temp1\n    div   c, b\n    mul   c, b\n    mov   d, a        ; temp2\n    sub   d, c\n    ret\n";
    let expected = Some(String::from("mod(11, 3) = 2"));

    let actual = AssemblerInterpreter::interpret(program);
    assert_eq!(actual, expected);
  }

  #[test]
  fn test5() {
    let program =  "\nmov   a, 81         ; value1\nmov   b, 153        ; value2\ncall  init\ncall  proc_gcd\ncall  print\nend\n\nproc_gcd:\n    cmp   c, d\n    jne   loop\n    ret\n\nloop:\n    cmp   c, d\n    jg    a_bigger\n    jmp   b_bigger\n\na_bigger:\n    sub   c, d\n    jmp   proc_gcd\n\nb_bigger:\n    sub   d, c\n    jmp   proc_gcd\n\ninit:\n    cmp   a, 0\n    jl    a_abs\n    cmp   b, 0\n    jl    b_abs\n    mov   c, a            ; temp1\n    mov   d, b            ; temp2\n    ret\n\na_abs:\n    mul   a, -1\n    jmp   init\n\nb_abs:\n    mul   b, -1\n    jmp   init\n\nprint:\n    msg   'gcd(', a, ', ', b, ') = ', c\n    ret\n";
    let expected = Some(String::from("gcd(81, 153) = 9"));

    let actual = AssemblerInterpreter::interpret(program);
    assert_eq!(actual, expected);
  }

  #[test]
  fn test6() {
    let program =  "\ncall  func1\ncall  print\nend\n\nfunc1:\n    call  func2\n    ret\n\nfunc2:\n    ret\n\nprint:\n    msg 'This program should return null'\n";
    let expected = None;

    let actual = AssemblerInterpreter::interpret(program);
    assert_eq!(actual, expected);
  }

  #[test]
  fn test7() {
    let program =  "\nmov   a, 2            ; value1\nmov   b, 10           ; value2\nmov   c, a            ; temp1\nmov   d, b            ; temp2\ncall  proc_func\ncall  print\nend\n\nproc_func:\n    cmp   d, 1\n    je    continue\n    mul   c, a\n    dec   d\n    call  proc_func\n\ncontinue:\n    ret\n\nprint:\n    msg a, '^', b, ' = ', c\n    ret\n";
    let expected = Some(String::from("2^10 = 1024"));

    let actual = AssemblerInterpreter::interpret(program);
    assert_eq!(actual, expected);
  }

  #[test]
  fn all_simplel_tests() {
      let simple_programs = &[
          "\n; My first program\nmov  a, 5\ninc  a\ncall function\nmsg  '(5+1)/2 = ', a    ; output message\nend\n\nfunction:\n    div  a, 2\n    ret\n",
          "\nmov   a, 5\nmov   b, a\nmov   c, a\ncall  proc_fact\ncall  print\nend\n\nproc_fact:\n    dec   b\n    mul   c, b\n    cmp   b, 1\n    jne   proc_fact\n    ret\n\nprint:\n    msg   a, '! = ', c ; output text\n    ret\n",
          "\nmov   a, 8            ; value\nmov   b, 0            ; next\nmov   c, 0            ; counter\nmov   d, 0            ; first\nmov   e, 1            ; second\ncall  proc_fib\ncall  print\nend\n\nproc_fib:\n    cmp   c, 2\n    jl    func_0\n    mov   b, d\n    add   b, e\n    mov   d, e\n    mov   e, b\n    inc   c\n    cmp   c, a\n    jle   proc_fib\n    ret\n\nfunc_0:\n    mov   b, c\n    inc   c\n    jmp   proc_fib\n\nprint:\n    msg   'Term ', a, ' of Fibonacci series is: ', b        ; output text\n    ret\n",
          "\nmov   a, 11           ; value1\nmov   b, 3            ; value2\ncall  mod_func\nmsg   'mod(', a, ', ', b, ') = ', d        ; output\nend\n\n; Mod function\nmod_func:\n    mov   c, a        ; temp1\n    div   c, b\n    mul   c, b\n    mov   d, a        ; temp2\n    sub   d, c\n    ret\n",
          "\nmov   a, 81         ; value1\nmov   b, 153        ; value2\ncall  init\ncall  proc_gcd\ncall  print\nend\n\nproc_gcd:\n    cmp   c, d\n    jne   loop\n    ret\n\nloop:\n    cmp   c, d\n    jg    a_bigger\n    jmp   b_bigger\n\na_bigger:\n    sub   c, d\n    jmp   proc_gcd\n\nb_bigger:\n    sub   d, c\n    jmp   proc_gcd\n\ninit:\n    cmp   a, 0\n    jl    a_abs\n    cmp   b, 0\n    jl    b_abs\n    mov   c, a            ; temp1\n    mov   d, b            ; temp2\n    ret\n\na_abs:\n    mul   a, -1\n    jmp   init\n\nb_abs:\n    mul   b, -1\n    jmp   init\n\nprint:\n    msg   'gcd(', a, ', ', b, ') = ', c\n    ret\n",
          "\ncall  func1\ncall  print\nend\n\nfunc1:\n    call  func2\n    ret\n\nfunc2:\n    ret\n\nprint:\n    msg 'This program should return null'\n",
          "\nmov   a, 2            ; value1\nmov   b, 10           ; value2\nmov   c, a            ; temp1\nmov   d, b            ; temp2\ncall  proc_func\ncall  print\nend\n\nproc_func:\n    cmp   d, 1\n    je    continue\n    mul   c, a\n    dec   d\n    call  proc_func\n\ncontinue:\n    ret\n\nprint:\n    msg a, '^', b, ' = ', c\n    ret\n"];

      let expected = &[
          Some(String::from("(5+1)/2 = 3")),
          Some(String::from("5! = 120")),
          Some(String::from("Term 8 of Fibonacci series is: 21")),
          Some(String::from("mod(11, 3) = 2")),
          Some(String::from("gcd(81, 153) = 9")),
          None,
          Some(String::from("2^10 = 1024"))];

      for (prg, exp) in simple_programs.iter().zip(expected) {
          let actual = AssemblerInterpreter::interpret(*prg);
          assert_eq!(actual, *exp);
      }
  }


}
