fn main() {
    println!("Hello, world!");
    t1();
    t2();
}

// 静态生命周期 贯穿整个进程的生命周期
// 值拥有静态生命周期，其引用也具有静态生命周期
// 'static 表示
// &'static str 这是一个具有静态生命周期的字符串引用

fn t1() {
    let s1 = String::from("Lindsey");
    let s2 = String::from("Rosie");

    let result = max(&s1, &s2);
    println!("bigger one: {}", result);
}

// 生命周期参数，描述的是参数和参数之间，参数和返回值之间的关系，并不改变原来的生命周期
fn max<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1 > s2 {
        s1
    } else {
        s2
    }
}

// 生命周期标注：在参数和返回值之间建立联系或者约束
// 编译器简单的规则添加生命周期的标注
// 1、所有引用类型的参数都有独立的生命周期'a,'b 等
// 2、如果只有一个引用型输入，它的生命周期会赋给所有输出
// 3、如果有多个引用类型的参数，其中一个是self，那么它的生命周期会赋给所有输出

fn first<'a>(s: &'a str) -> &'a str {
    let trimmed = s.trim();
    match trimmed.find(' ') {
        None => "",
        Some(pos) => &trimmed[..pos],
    }
}

fn t2() {
    let s = "hello world".to_owned();
    let mut s1 = s.as_str();
    let hello = strtok(&mut s1, ' ');
    println!("hello is: {}, s1: {}, s: {}", hello, s1, s);
}

fn strtok<'a>(s: &mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i+delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

// 结构体自身的生命周期，需要小于等于其内部字段的所有引用的生命周期
struct Employee<'a, 'b> {
    name: &'a str,
    title: &'b str,
    age: u8,
}