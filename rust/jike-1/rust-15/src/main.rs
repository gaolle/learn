use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;

fn main() {
    println!("Hello, world!");
}

// 智能指针

// 智能指针一定是一个胖指针，但胖指针不一定是一个智能指针

// 智能指针 拥有所有权

// 凡是需要做资源回收的数据结构，且实现了Deref/DerefMut/Drop 都是智能指针

// 堆上创建内存的Box<T>
// box 内部关键字 用于分配堆内存

// 提供写时克隆的Cow<'a, B>
// 包裹一个只读借用，但如果调用者需要所有权或者需要需要修改内容，clone借用的数据

// 用于数据加锁的MutexGuard<T>
// 不允许Send，只允许Sync

const MINI_STRING_MAX_LEN: usize = 30;

struct MiniString {
    len: u8,
    data: [u8; MINI_STRING_MAX_LEN],
}

impl MiniString {
    fn new(v: impl AsRef<str>) -> Self {
        let bytes = v.as_ref().as_bytes();
        let len = bytes.len();
        let mut data = [0u8; MINI_STRING_MAX_LEN];
        data[..len].copy_from_slice(bytes);
        Self {
            len: len as u8,
            data,
        }
    }
}

impl Deref for MiniString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        std::str::from_utf8(&self.data[..self.len as usize]).unwrap()
    }
}

impl Debug for MiniString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.deref())
    }
}

#[derive(Debug)]
enum MyString {
    Inline(MiniString),
    Standard(String),
}

impl Deref for MyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match *self {
            MyString::Inline(ref v) => v.deref(),
            MyString::Standard(ref v) => v.deref(),
        }
    }
}


impl From<&str> for MyString {
    fn from(s: &str) -> Self {
        match s.len() > MINI_STRING_MAX_LEN{
            true => Self::Standard(s.to_owned()),
            _ => Self::Inline(MiniString::new(s)),
        }
    }
}

impl Display for MyString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.deref())
    }
}

fn t1() {
    let len1 = std::mem::size_of::<MyString>();
    let len2 = std::mem::size_of::<MiniString>();
    println!("Len: MyString {}, MiniString {}", len1, len2);
}