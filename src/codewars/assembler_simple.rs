use std::io;
use std::collections::HashMap;

fn simple_assembler(program: Vec<&str>) -> HashMap<String, i64> {
  let mut registers = HashMap::new();

  let mut i: usize = 0;
  while i < program.len() {
    let cmd = program[i];
    let opr: Vec<&str> = cmd.split(" ").collect();

    match opr[0] {
      "mov" => {
        if opr[2].chars().nth(0).unwrap().is_alphabetic() {
          let regval = registers[opr[2]];
          *registers.entry(String::from(opr[1])).or_insert(regval) = regval;
        } else {
          let opval = opr[2].parse::<i64>().unwrap();
          *registers.entry(String::from(opr[1])).or_insert(opval) = opval;
        }
      },
      "inc" => {
        registers.entry(String::from(opr[1])).and_modify(|v| *v += 1);
      },
      "dec" => {
        registers.entry(String::from(opr[1])).and_modify(|v| *v -= 1);
      },
      "jnz" => {
        if (opr[1].chars().nth(0).unwrap().is_alphabetic() && registers[opr[1]] != 0) ||
           (opr[1].chars().nth(0).unwrap().is_numeric()) {
          i = (i as i64 + opr[2].parse::<i64>().unwrap()) as usize;
          continue;
        }
      },
      _ => {}
    }

    i += 1;
  }

  println!("{:?}", registers);
  registers
}

fn main() {
  let stdin = io::stdin();

  let mut program = String::new();
  stdin.read_line(&mut program).expect("failed to read stdin");
  let program: String = program.trim().parse().expect("invalid input");

  let mut program2 = String::new();
  stdin.read_line(&mut program2).expect("failed to read stdin");
  let program2: String = program2.trim().parse().expect("invalid input");

  println!("{:?}", simple_assembler(vec![&program, &program2]));
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! map {
        ($($key:expr => $value:expr),*) => {{
             let mut map = HashMap::new();
             $(
                 map.insert($key.to_string(), $value);
             )*
             map
        }};
    }

    #[test]
    fn short_tests() {
        let program = vec![
            "mov c 12",
            "dec c",
            "jnz c -1",
            "jnz 0 1",
            "mov a c",
        ];
        let expected = map! { "a" => 0, "c" => 0 };
        compare_registers(expected, simple_assembler(program));

        let program = vec!["mov a 5", "mov b 10", "mov a b"];
        let expected = map! { "a" => 10, "b" => 10 };
        compare_registers(expected, simple_assembler(program));

        let program = vec!["mov a 5", "inc a", "dec a", "dec a", "jnz a -1", "inc a"];
        let expected = map! { "a" => 1 };
        compare_registers(expected, simple_assembler(program));

        let program = vec![
            "mov c 12",
            "mov b 0",
            "mov a 200",
            "dec a",
            "inc b",
            "jnz a -2",
            "dec c",
            "mov a b",
            "jnz c -5",
            "jnz 0 1",
            "mov c a",
        ];
        let expected = map! { "a" => 409600, "c" => 409600, "b" => 409600};
        compare_registers(expected, simple_assembler(program));
    }

    fn compare_registers(expected: HashMap<String, i64>, actual: HashMap<String, i64>) {
        let result = expected
            .iter()
            .all(|(key, value)| actual.get(key).map(|v| v == value).unwrap_or(false));
        assert!(
            result,
            "Expected the registers to be like that:\n{:#?}\n\nBut got this:\n{:#?}\n",
            expected, actual
        )
    }
}
