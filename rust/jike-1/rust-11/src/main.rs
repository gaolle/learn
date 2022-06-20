use std::mem::align_of;
use std::mem::size_of;

fn main() {
    println!("Hello, world!");
    t1();
}

// 将堆内存的生命周期和使用它的占内存的生命周期绑在一起，并留了一个小口子leaked机制
// 让堆内存在需要的时候，可以有超出帧存活期的生命周期


// Rust 会自动重排定义的结构，来达到最高效率
// 关闭编译器优化
// #[repr()]
struct S1 {
    a: u8,
    b:u16,
    c:u8,
}

struct S2 {
    a:u8,
    b:u8,
    c:u16,
}

fn t1() {
    println!("sizeof s1: {}, s2: {}", size_of::<S1>(), size_of::<S2>());
    println!("alignof s1: {}, s2: {}", align_of::<S1>(), align_of::<S2>());
}

// enum 标签联合体 大小=标签的大小+最大类型的长度


// Vec<u8>  String
// String  底层 Vec<u8>

// Copy Move 内部实现都是上，都是浅层的按位做内存复制
// Copy 允许访问之前的变量 Move 不允许