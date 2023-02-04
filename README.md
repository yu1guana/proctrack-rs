# ProcTrack

This is a tool to make debugging Rust programs easier.

In this repository, there are four crate.

- [proctrack](proctrack)
  - This crate has a main program and export the other crates.
	  Logs are printed only when you building without the `disable` feature in debug mode.
- [typename](typename)
	- This crate defines the trait `TypeName` to get names of structs.
- [typename\_derive](typename_derive)
	- This crate defines a derive macro to implement the trait `TypeName` in the typename crate.
- [func\_log](funclog)
	- This crate defines attribute macros to print logs of entering and exiting functions, and changing of variables in functions into stderr.
	  Logs are printed only when you building without the `disable` feature in debug mode.

# Usage

Example of main.rs and its stderr is as follows.

```rust
use proctrack::funclog::{funclog, methodlog, methodlog_move, methodlog_static};
use proctrack::typename_derive::TypeName;

#[funclog]
fn hello() {
    println!("Hello.");
}

#[derive(TypeName)]
struct A {
    a: i64,
}

impl A {
    #[methodlog_static]
    fn new(a: i64) -> Self {
        Self { a }
    }
    #[methodlog]
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
```

```text
[DEBUG:func_enter(main.rs:4)] hello
[DEBUG:func_exit(main.rs:4)] hello
[DEBUG:func_enter(main.rs:15)] A::new
[DEBUG:func_exit(main.rs:15)] A::new
[DEBUG:func_enter(main.rs:19)] A::add
[DEBUG:func_exit(main.rs:19)] A::add
[DEBUG:func_enter(main.rs:23)] A::take
[DEBUG:func_exit(main.rs:23)] A::take
```

## License

Copyright (c) 2023 Yuichi Ishida  
Released under the MIT license  
[https://opensource.org/licenses/mit-license.php](https://opensource.org/licenses/mit-license.php)
