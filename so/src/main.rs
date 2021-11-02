// https://stackoverflow.com/questions/40869345/rust-match-struct-attribute-in-method-fails-borrow-checker

// option 1 - use reference in match
// option 2 - derive Clone and copy for E
// option 3 - use reference e in arm

// option 4 - return Boxed

#[derive(Debug,Clone)]
enum E {
    Foo(u32),
    Bar,
    Baz
}

struct S {
  a: E,
}

impl S {
    fn print(&self) {
        match &self.a {
            // E::Foo(a) => println!("{:?}", a),
            e => println!("{:?}", e)
        }
    }

    fn to_box(&self) -> Box<E> {
      match &self.a {
          // E::Foo(a) => Box::new(E::Foo(*a)),
          e => Box::new(e.clone()),
      }
    }
}

fn main() {
    let a = S {
      // a: E::Bar
      a: E::Foo(5),
    };
    a.print();
    let b = a.to_box();
    println!("{:?}", b);

}
