fn main() {
    println!("Hello, world!");
    t1();
    // t2();
    t3();
    // t4();
    // t5();
    t6();
}

// 所有参数传递都是传值
// 借用不能超过值的生存期
// 堆上可以使用栈上内存的引用  栈内存生命周期>=堆上内存生命周期
fn t1() {
    let data = vec![1, 2, 3, 4];
    let data1 = &data;
    println!("sum of data1: {}", sum(&data1));
    println!("data1: {:?}", data1);
    println!("sum of data: {}", sum(&data));
}

fn sum(data: &Vec<u32>) -> u32 {
    data.iter().fold(0, |acc, x| acc + x)
}

// fn t2() {
//     let r = local_ref();
//     println!("r: {:p}", r);
// }
//
// fn local_ref<'a>() -> &'a i32 {
//     let a = 42;
//     &a
// }

fn t3() {
    let mut data: Vec<&u32> = Vec::new();
    let v = 32;
    data.push(&v);
    println!("data: {:?}", data);
}

// fn t4() {
//     let mut data: Vec<&u32> = Vec::new();
//     push_local_ref(&mut data);
//     println!("data: {:?}", data);
// }
//
// fn push_local_ref(data: &mut Vec<&u32>) {
//     let v = 42;
//     data.push(&v);
// }

// // 同一作用域下多个可变引用
// fn t5() {
//     let mut data = vec![1, 2, 3];
//     for item in data.iter_mut() {
//         data.push(*item + 1);
//     }
// }


// 一个作用域内，只允许一个活跃（真正被使用来修改数据的可变引用）的可变引用
// 一个作用域内，活跃的可变引用（写）和只读引用（读）是互斥的，不能同时存在
// 引用的生命周期不能超出值的生命周期
fn t6() {
    let mut data = vec![1, 2, 3];
    let data1 = vec![&data[0]];
    println!("data[0]: {:p}", &data1[0]);

    for i in 0..100 {
        data.push(i);
    }

    println!("data[0]: {:p}", &data[0]);
    // println!("boxed: {:p}", &data1);
}
