use std::{collections::HashMap, mem::size_of_val, ops::Mul, thread};

fn main() {
    println!("Hello, world!");
    t();
    t1();
    t2();
    t3();
    t4();
    t6();
}

fn t() {
    let a = "hello";
    let b = "Try";

    let c = |msg: &str| {
        println!("{} {}: {}", a, b, msg);
    };
    c("how are you?");
    println!("{}", a);
}

fn t1() {
    let s = String::from("hello world");

    let handle = thread::spawn(move || {
        println!("moved: {:?}", s);
    });
    handle.join().unwrap();
}

// 闭包是一种匿名类型，一旦声明，就会产生一个新的类型
// 在其他地方无法使用 包含捕获的变量
fn t2() {
    let c1 = || println!("hello world");
    let c2 = |i: i32| println!("hello: {}", i);
    let name = String::from("try");
    let name1 = name.clone();
    let mut table = HashMap::new();
    table.insert("hello", "world");
    let c3 = || println!("hello: {}", name);
    let c4 = move || println!("hello: {}, {:?}", name1, table);
    let name2 = name.clone();
    let c5 = move || {
        let x = 1;
        let name3 = String::from("lindsey");
        println!("hello: {}, {:?}, {:?}", x, name2, &name3);
    };
    println!(
        "c1: {}, c2: {}, c3: {}, c4: {}. c5: {}, main: {}",
        size_of_val(&c1),
        size_of_val(&c2),
        size_of_val(&c3),
        size_of_val(&c4),
        size_of_val(&c5),
        size_of_val(&main),
    )
}

// 闭包 不带move  闭包捕获的是对应自由变量的引用
// 闭包 带move 对应自由变量的所有权会被移动到闭包结构中
// 闭包的大小跟参数、局部变量都无关，只跟捕获的变量有关
// 闭包是存储在栈上，并且除了捕获的数据外，闭包本身不包含任何额外函数指针指向闭包的代码

// FnOnce 只能被调用一次
fn t3() {
    let name = String::from("Try");
    let c = move |greeting: String| (greeting, name);
    let result = c("hello".to_string());

    println!("result: {:?}", result);

    // let result = c("hi".to_string());
}

// 一个闭包 并不转移自己的内部数据，不是FnOnce
fn t4() {
    let name = String::from("Try");

    let c = move |greeting: String| (greeting, name.clone());

    println!("c1 call once: {:?}", c("qiao".into()));
    println!("c1 call two: {:?}", c("bonjour".into()));

    println!("result: {:?}", call_once("hi".into(), c));

    // let result = c("hi".to_string());

    println!("result: {:?}", call_once("hola".into(), not_closure));
}

fn call_once(arg: String, c: impl FnOnce(String) -> (String, String)) -> (String, String) {
    c(arg)
}

fn not_closure(arg: String) -> (String, String) {
    (arg, "Rosie".into())
}

// FnMut : FnOnce 允许在执行时修改闭包的内部数据，可以执行多次
 
fn t5() {
    let mut name = String::from("hello");
    let mut name1 = String::from("hola");

    let mut c = || {
        name.push_str("Try");
        println!("c: {}", name);
    };

    let mut c1 = move || {
        name1.push_str("!");
        println!("c1: {}", name1);
    };

    c();
    c1();

    call_mut(&mut c);
    call_mut(&mut c1);

    call_once1(c);
    call_once1(c1);
}

fn call_mut(c: &mut impl FnMut()) {
    c();
}

fn call_once1(c: impl FnOnce()) {
    c()
}

// Fn : FnMut 不允许修改闭包的内部数据，也可以执行多次

fn t6() {
    let c1 = curry(5);
    println!("5 multiply 2 is: {}", c1(2));
}

fn curry<T>(x: T) -> impl Fn(T) -> T
where
    T: Mul<Output = T> + Copy,
{
    move |y| x * y
}

// 闭包效率高  闭包捕获的变量都存储在栈上，没有堆内存分配
