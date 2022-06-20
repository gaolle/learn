// Future 协程 只有主动await后才能开始执行
//  async/await

// fn main() {

// }
// use anyhow::Result;
// use serde_yaml::Value;
// use tokio::{fs, try_join};

// #[tokio::main]
// async fn main() {
//     let f1 = fs::read_to_string("./Cargo.toml");
//     let f2 = fs::read_to_string("./Cargo.lock");
//     // let (content1, content2) = try_join!(f1, f2);

// }

// fn toml2yml(content: &str) -> Result<String> {
//     let value: Value = toml::from_str(&content)?
//     Ok(serde::toml2yml::to_string(&value)?)
// }
use futures::executor::block_on;
use std::{str, future::Future};

#[tokio::main]
async fn main() {
    let name1 = "Try".to_string();
    let name2 = "Lindsey".to_string();


    say_hello(&name1).await;
    say_hello1(&name2).await;

    block_on(say_hello(&name1));
    block_on(say_hello1(&name2));
}

async fn say_hello(name: &str) -> usize {
    println!("Hello {}", name);
    42
}

fn say_hello1<'Fut>(name: &'Fut str) -> impl Future<Output = usize> + 'Fut  {
    async move {
        println!("Hello {}", name);
        42
    }
}

// executor Future 调度器

// Reactor Pattern
// task: 待处理的任务。任务可以被打断，并且把控制权交给executor，等待之后的调度
// executor：一个调度器。维护等待运行的任务，以及被阻塞的任务
// reactor 维护事件队列。当事件来临时，通知executor唤醒某个任务等待运行


// 计算密集型任务
// CPU密集型 线程
// IO密集型 Future 协作式多任务  除非Future主动放弃CPU，否则一直执行，直到运行结束

