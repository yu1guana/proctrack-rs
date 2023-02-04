use proctrack::funclog::{funclog, methodlog, methodlog_move, methodlog_static};
use proctrack::typename_derive::TypeName;

#[funclog]
fn hello() {
    println!("Hello.");
}

#[derive(Debug, TypeName)]
struct A {
    a: i64,
}

impl A {
    #[methodlog_static]
    fn new(a: i64) -> Self {
        Self { a }
    }
    #[methodlog(self)]
    fn add(&mut self, b: i64) {
        self.a += b;
    }
    #[methodlog_move]
    fn take(self) -> i64 {
        self.a
    }
}

fn main() {
    hello();
    let mut a = A::new(10);
    a.add(2);
    let _ = a.take();
}
