use core::fmt;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("Hello, world!");
    t5();
    t4();
    t3();
    t2();
}

// Debug {:?} 打印
// From<T> /  TryFrom<T> into() 数据转换
// Clone / Copy 深拷贝/浅拷贝
// Read / Write 对I/O读写的行为
// Iterator 约定了迭代器的行为
// Default 约定数据类型的缺省值如何产生的行为

// 内存相关

// 派生宏 move 语义
// #[derive(Clone)]

// Copy 标记trait
// #[derive(Copy)]

// Drop

// Copy Drop 不能共存
// Copy 按位浅拷贝  默认拷贝的数据没有需要释放的资源
// Drop 释放资源

// #[allow(dead_code)] 取消未使用警告

// #[allow(dead_code)] 编译器对未使用的代码会产生警告
fn t() {}

// 标记相关

// Sized 标记有具体大小的类型 编译器自动为泛型参数添加Sized约束
// ？Sized 可变大小类型

// Send / Sync  并发安全基础

// Send   T 安全从一个线程移动到另一个线程  所有权可以在线程间移动 线程独占
// Sync &T 安全地可以在多个线程中共享， 线程间只读安全
// T 满足 Sync ,当前仅当&T 满足Send

// 不支持 Send/Sync
// 1、裸指针 *const T / *mut T  不安全
// 2、UnsafeCell<T>   任何使用了Cell/RefCell的数据结构不支持Sync
// 3、引用计数Rc不支持Send也不支持Sync

// 'static 拥有所有权的类型或者是一个拥有静态生命周期的引用

// Arc<Mutex<T>> 可以多线程共享且修改数据
fn arc_mutext_is_send_sync() {
    let a = Arc::new(Mutex::new(1));
    let b = a.clone();
    let c = a.clone();
    let handle = thread::spawn(move || {
        let mut g = c.lock().unwrap();
        *g += 1;
    });

    {
        let mut g = b.lock().unwrap();
        *g += 1;
    }
    handle.join().unwrap();
    println!("a={:?}", a);
}

fn t5() {
    arc_mutext_is_send_sync();
}

// Unpin 自引用类型

// 类型转化相关
// From<T> / Into<T> / AsRef<T> / AsMut<T> 把一种数据格式转为另一种数据格式

// 值类型到值类型的转换 From<T> / Into<T> / TryFrom<T> / TryInto<T>
// 引用类型到引用类型的转换 AsRef<T> / AsMut<T>

fn t4() {
    let s = String::from("hello world");
    let s1: String = "hello world".into();
    println!("{:?}", s);
}

// From<T> 可以根据上下文做类型推导， 实现了From<T>会自动实现Into<T>

// AsRef<T> / AsMut<T>

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Language {
    Rust,
    TypeScript,
}

impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        match self {
            Language::Rust => "Rust",
            Language::TypeScript => "typeScript",
        }
    }
}

fn print_ref(v: impl AsRef<str>) {
    println!("{}", v.as_ref());
}

fn t3() {
    let lang = Language::Rust;
    print_ref("hello world");
    print_ref("hello world".to_string());
    print_ref(lang);
}

// 操作符相关
// Deref / DerefMut 解引用

struct Buffer<T>(Vec<T>);

impl<T> Buffer<T> {
    // pub fn new(v: impl Into<Vec<T>) -> Self {
    //     Self(v.into())
    // }
}

impl<T> Deref for Buffer<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

// Debug / Display / Default
// Debug 为开发者调试打印数据结构所设计 派生宏直接生成
// Display 为用户显示数据结构所设计 手工实现

#[derive(Debug, Clone, Default)]
struct Developer {
    name: String,
    age: u8,
    lang: Language,
}

impl Default for Language {
    fn default() -> Self {
        Language::Rust
    }
}

impl Developer {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            ..Default::default()
        }
    }
}

impl fmt::Display for Developer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{} years old): {:?} developer",
            self.name, self.age, self.lang
        )
    }
}
fn t2() {
    let dev1 = Developer::default();
    let dev2: Developer = Default::default();
    let dev3 = Developer::new("Try");
}

