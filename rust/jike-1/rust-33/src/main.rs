use std::sync::mpsc::channel;

fn main() {
    println!("Hello, world!");
}

// 自由竞争模式 多个并发任务会竞争同一个临界区的访问权

// cas 先比较后赋值

// atomic

// mutex  读写互斥

// SpinLock 线程忙等待
// Mutex lock  线程在等待锁的时候会被调度出去，等锁可用时再次被调度回来

// channel
