use proctrack::funclog::funclog;

fn main() {
    f0();
    f0();
    f0();
    f0();
    f0();
}

#[funclog]
fn f0() {
    f1()
}

#[funclog]
fn f1() {
    f2()
}

#[funclog]
fn f2() {
    f3()
}

#[funclog]
fn f3() {
    f4()
}

#[funclog]
fn f4() {
    println!("hello")
}
