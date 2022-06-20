// use std::alloc::{Allocator, Global};
// use std::ptr::Unique;

use std::io::Read;

fn main() {
    println!("Hello, world!");
}

// 泛型函数后使用 ::<T> 强制使用类型T

// // A 具有默认值 Global
// pub struct RawVec<T, A: Allocator = Global> {
//     ptr: Unique<T>,
//     cap: usize,
//     alloc: A,
// }

// Sized 固定大小的类型
// ？Sized 代表可变大小的类型
// ToOwned 可以把借用的数据克隆出一个拥有所有权的数据
// pub enum Cow<'a, B: ?Sized + 'a>
// where
//     B: ToOwned, {
//
// }

// 子类型可以强制转换为父类型

// 定义一个带有泛型参数R的 reader 此处不限制R
// struct MyRead<R> {
//     reader: R,
//     buf: String,
// }
//
// // new 函数 不限制R
// impl<R> MyRead<R> {
//     pub fn new(reader: R) -> Self {
//         Self {
//             reader,
//             buf: String::with_capacity(1024),
//         }
//     }
// }
//
// impl<R> MyRead<R>
// where
//     R: Read,
// {
//     pub fn process(&mut self) -> Result<usize> {
//         self.reader.read_to_string(&mut self.buf)
//     }
// }

fn id<T>(x: T) -> T {
    return x;
}
