
fn main() {
    println!("Hello, world!");
    t1();
    t2();
}

fn t1() {
    // 动态数组大小 编译时无法确定，存放在堆上，在栈上有一个包含了长度和容量的指针指向堆内存
    let data = vec![1, 2, 3];
    // 分配在栈上
    let v = 2;
    // 一个值只能被一个变量所拥有
    // 一个值同一时刻只能有一个所有者
    // 当所有者离开作用域，其拥有的值被丢弃
    {
        let v = 1;
        {
            let v1 = 2;
            // v1 离开括号时，被丢弃
        }
    }
    // 当一个值移动时，先检测Copy浅拷贝是否实现，未实现然后默认使用Move语义 所有权转移

    // data 所有权转移到 find_pos
    // v 实现了 Copy  浅拷贝
    if let Some(pos) = find_pos(data, v) {
        println!("Found {} at {}", v, pos);
    }
}

fn find_pos(data: Vec<u32>, v:u32) -> Option<usize> {
    for (pos, item) in data.iter().enumerate() {
        if *item == v {
            return Some(pos);
        }
    }
    None
}

fn is_copy<T: Copy>() {}

fn types_impl_copy_trait() {
    is_copy::<bool>();
    is_copy::<char>();

    is_copy::<i8>();
    is_copy::<u64>();
    is_copy::<i64>();
    is_copy::<usize>();

    is_copy::<fn()>();

    is_copy::<*const String>();
    is_copy::<*mut String>();

    is_copy::<&[Vec<u8>]>();
    is_copy::<&String>();

    is_copy::<[u8; 4]>();
    is_copy::<(&str, &str)>();
}

fn types_not_impl_copy_trait() {
    is_copy::<str>();
    is_copy::<[u8]>();
    is_copy::<Vec<[u8]>>();
    is_copy::<String>();

    is_copy::<&mut String>();

    is_copy::<[Vec<u8>; 4]>();
    is_copy::<(String, u32)>();
}

// 原声类型 函数、不可变引用 裸指针 实现了Copy
// 数组 元组 内部实现了Copy 也实现了Copy
// 可变应用没有实现Copy
// 非固定大小的数据结构没有实现Copy

fn t2() {
    types_impl_copy_trait();
    types_not_impl_copy_trait();
}