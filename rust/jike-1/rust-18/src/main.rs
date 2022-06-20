use std::{fs::File, io::Read};

fn main() {
    println!("Hello, world!");
    t1();
}

fn read_file(name: &str) -> Result<String, std::io::Error> {
    let mut f = File::open(name)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

// ? 操作符内部展开
// match result {
//     Ok<v> => v,
//     Err(e) => return Err(e.into)
// }

// panic!() 不可恢复或者不想恢复错误
use std::panic;
fn t1() {
    let result = panic::catch_unwind(|| {
        println!("hello!");
    });
    assert!(result.is_ok());
    let result = panic::catch_unwind(|| {
        panic!("oh no!");
    });
    assert!(result.is_err());
    println!("panic captured: {:#?}", result);
}
