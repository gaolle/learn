use std::fs;
mod abi;

fn main() {
    t1();
    t2();


    let s = abi::pb::Server{};
}

fn t1() {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";

    println!("Fecting url:{}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown had benn saves in {}", output);
}

// 函数一等公民
fn apply(value: i32, f: fn(i32)->i32) -> i32 {
    f(value)
}

fn square(value: i32) -> i32 {
    value * value
}

fn cube(value: i32) -> i32 {
    value * value * value
}

fn t2() {
    println!("apply square: {}", apply(2, square));
    println!("apply cube: {}", apply(2, cube));
}


// 派生宏
#[derive(Debug)]
enum Gender {
    Unspecified = 0, 
    Female = 1,
    Male = 2,
}

// Clone 深拷贝
// Copy 浅拷贝

#[derive(Debug, Clone, Copy)]
struct UserId(u64);

#[derive(Debug, Clone, Copy)]
struct TopicId(u64);

#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}

#[derive(Debug)]
struct Topic {
    id: TopicId, 
    name: String,
    owner: UserId,
}

enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}


fn lib_loop(n: u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;


    loop {
        let c = a + b;
        a = b;
        b = c;

        i += 1;
        println!("next val is {}", b);

        if i >= n {
            break;
        }
    }
}


fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);

    while i <= n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
        println!("next val is {}", b);
    }
}


// Iterator 迭代器 

fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 1);

    for _1 in 2..n { // 左闭右开  range 下标为 usize 类型 不能为负数
        let c = a + b;
        a = b;
        b = c;
        println!("next val is {}", c);
    }

}


// 条件编译
// #[cfg(test)]


// tests 集成测试  只能测试 crate 下的公共接口


// 声明宏
// 过程宏 1、函数宏 （println!） 2、派生宏（derive）3、属性宏（attribute）