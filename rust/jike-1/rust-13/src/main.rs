use regex::Regex;
use std::fmt::Arguments;
use std::future::Future;
use std::io::{IoSlice, Write};
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
    t1();
    t3();
}

// 特设多态 包括运算符重载，是指同一种行为有很多不同的实现
// 类型多态 把子类型当成父类型使用

// self -> self:Self
// &self -> self:&Self
// &mut self -> self:&mut Self

#[derive(Debug)]
struct BufBuilder {
    buf: Vec<u8>,
}

impl BufBuilder {
    pub fn new() -> Self {
        Self {
            buf: Vec::with_capacity(1024),
        }
    }
}

impl Write for BufBuilder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn t1() {
    let mut buf = BufBuilder::new();
    buf.write_all(b"hello world!").unwrap();
    println!("{:#?}", buf);
}

// 调用T::parse(str)
pub trait Parse {
    // Error 返回的错误 trait 不能确定 不同实现者可以使用不同的错误类型
    // 关联类型
    type Error;
    fn parse(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

// // u8 结构体实现parse
// impl Parse for u8 {
//     fn parse(s: &str) -> Self {
//         let re: Regex = Regex::new(r"^[0-9]+").unwrap();
//         if let Some(captures) = re.captures(s) {
//             captures
//                 .get(0)
//                 .map_or(0, |s| s.as_str().parse().unwrap_or(0))
//         } else {
//             0
//         }
//     }
// }

// #[test]
// fn parse_should_work() {
//     assert_eq!(u8::parse("123abcd"), 123);
//     assert_eq!(u8::parse("1234abcd"), 0);
//     assert_eq!(u8::parse("abcd"), 0);
// }

// fn t2() {
//     println!("result: {}", u8::parse("255 hello world"));
// }

impl<T> Parse for T
where
    T: FromStr + Default,
{
    type Error = String;
    fn parse(s: &str) -> Result<Self, Self::Error> {
        let re: Regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        if let Some(captures) = re.captures(s) {
            captures
                .get(0)
                .map_or(Err("failed to capture".to_string()), |s| {
                    s.as_str()
                        .parse()
                        .map_err(|_err| "failed to parse captured string".to_string())
                })
        } else {
            Err("failed to parse string".to_string())
        }
    }
}

// 不提供泛型参数，add 左右值类型要相同
// pub trait Add<Rhs = Self> {
//     type Output;
//     #[must_use]
//     fn add(self, rhs: Rhs) -> Self::Output;
// }

struct Complex {
    real: f64,
    imagine: f64,
}

impl Complex {
    pub fn new(real: f64, imagine: f64) -> Self {
        Self { real, imagine }
    }
}

use std::ops::Add;
use std::task::{Context, Poll};

impl Add for Complex {
    // XXX
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;
        Self::new(real, imagine)
    }
}

// 所有权不转移
// 默认范型 同类型
impl Add for &Complex {
    // XXX
    type Output = Complex;
    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;
        Complex::new(real, imagine)
    }
}

// 自定义范型 不同类型
impl Add<f64> for &Complex {
    type Output = Complex;
    fn add(self, rhs: f64) -> Self::Output {
        !todo!()
    }
}

// 泛型 trait 对同一种类型的同一个trait，有多个实现
pub trait Service<Request> {
    type Response;
    type Error;
    type Future: Future;
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;
    fn call(&mut self, req: Request) -> Self::Future;
}

struct Cat;
struct Dog;

trait Animal {
    fn name(&self) -> &'static str;
}

impl Animal for Cat {
    fn name(&self) -> &'static str {
        "cat"
    }
}

impl Animal for Dog {
    fn name(&self) -> &'static str {
        "dog"
    }
}

// fn name(animal: impl Animal) -> &'static str {
//     animal.name()
// }
fn name<T: Animal>(animal: &T) -> &'static str {
    animal.name()
}

fn t3() {
    let cat = Cat;
    println!("cat: {}", name(&cat));
    let dog = Dog;
    println!("dog: {}", name(&dog));
}

pub trait Formatter {
    fn format(&self, fmt: &mut String) ->bool;
}

struct MarkdownFormatter;
impl Formatter for MarkdownFormatter {
    fn format(&self, fmt: &mut String) -> bool {
        todo!()
    }
}

struct RustFormatter;
impl Formatter for RustFormatter {
    fn format(&self, fmt: &mut String) -> bool {
        todo!()
    }
}

struct HtmlFormatter;
impl Formatter for HtmlFormatter {
    fn format(&self, fmt: &mut String) -> bool {
        todo!()
    }
}

//告诉编译器 需要且仅需要任何实现了Formatter接口的数据类型
// Trait Object -> &dyn Trait 或者 Box<dyn Trait>
// Trait Object 胖指针 数据本身+虚函数表
pub fn format(input: &mut String, formatters: Vec<&dyn Formatter>) {
    for formatter in formatters {
        formatter.format(input);
    }
}

fn t4() {
    let mut text = "text".to_string();
    let html: &dyn Formatter = &HtmlFormatter;
    let rust: &dyn Formatter = &RustFormatter;
    let formatters = vec![html, rust];
    format(&mut text, formatters);

    println!("text: {}", text);
}

// Trait 存在对象安全
// 只有满足对象安全的trait才能使用trait object

// trait 所有方法 返回值是Sel或者携带泛型参数，就不能产生trait object
// 不允许返回Self   trait object 产生时，原来的类型会被抹去，原来的Self 不知道是谁
// 不允许携带泛型参数  Rust对带泛型的类型在编译时会做单态化，trait object 是运行时的产物，两者不能兼容