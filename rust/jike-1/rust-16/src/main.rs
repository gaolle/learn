fn main() {
    println!("Hello, world!");
    t1();
}

// 集合容器

// 切片 [T] 同一类型、长度不确定、在内存中连续存放的数据结构
// &[T] 一个只读的切片引用
// &mut [T] 一个可写的切片引用
// Box<T> 一个在堆上分配的切片

// 切片 迭代器
fn t1() {
    // 懒接口 collect 才运行
    let result = vec![1, 2, 3, 4]
    .iter()
    .map(|v|v*v)
    .filter(|v| *v < 16)
    .take(1)
    .collect::<Vec<_>>();
}

// &str

// Box<T> 一旦生成固定下来，没有capacity，也无法增长
// Box<T> &[T] 栈上有一个包含长度的胖指针，指向内存数据的内存位置
// Box[T] 只指向堆 &[T] 指向的位置可以是栈也可以是堆
// Box<[T]> 具有所有权 &[T] 借用