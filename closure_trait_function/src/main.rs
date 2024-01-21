use std::borrow::Borrow;

fn main() {
    println!("Hello, closure trait function !");
    wrap_fn(fn1);
    wrap_fn(fn2);
    wrap_fn_box(Box::new(fn_box_1));
    wrap_fn_box(Box::new(fn_box_2));
}

fn fn1() -> i32 {
    1
}
fn fn2() -> i32 {
    2
}

fn fn_box_1() -> &'static str {
    "1"
}

fn fn_box_2() -> &'static str {
    "2"
}

fn wrap_fn_box(fns: Box<dyn Fn() -> &'static str>) {
    println!("a box fn return : {}", fns());
}

fn wrap_fn(_fn: fn() -> i32) {
    println!("a normal stack fn return : {}", _fn());
}


// Fn -> FnMut -> FnOnce
// Fn 是基于FnMut, FnMut是基于FnOnce
// 他们说 Fn 是 FnMut 的超特征, FnMut 是 FnOnce 的超特征
// 所以，任何实现了Fn的必须实现FnMut, 任何实现了FnMut的必须实现FnOnce