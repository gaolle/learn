use std::cell::{Ref, RefCell};
use std::rc::Rc;

fn main() {
    println!("Hello, world!");
    t1();
    t2();
    t3();
}

fn t1() {
    // Rc 指针
    let a = Rc::new(1);
    // Rc 进行clone 不会复制内部数据，只增加引用计数
    // 离开作用域时，drop 减少引用计数，计数为0，清除内存
    let b = a.clone();

    // Rc 创建不受栈内存控制的堆内存，绕过编译时的所有权规则
    // Box 只能指针 强制把任何数据结构创建在堆上
    // Box::leak() 创建的对象，从堆内存泄漏出去，不受栈内存控制，通过引用计数的检查，保证堆内存最终能够释放

}

// #[derive(Debug)]
// struct Node {
//     id: usize,
//     downstream: Option<Rc<Node>>,
// }
//
// impl Node {
//     pub fn new(id: usize) -> Self {
//         Node {
//             id,
//             downstream: None,
//         }
//     }
//
//     pub fn update_downstream(&mut self, downstream: Rc<Node>) {
//         self.downstream = Some(downstream);
//     }
//
//     pub fn get_downstream(&self) -> Option<Rc<Node>> {
//         self.downstream.as_ref().map(|v|v.clone())
//     }
// }
//
// fn t2() {
//     // Rc 只读的引用计数器
//     let mut node1 = Node::new(1);
//     let mut node2  = Node::new(2);
//     let mut node3 = Node::new(3);
//     let node4 = Node::new(4);
//
//     node3.update_downstream(Rc::new(node4));
//     node1.update_downstream(Rc::new(node3));
//     node2.update_downstream(node1.get_downstream().unwrap());
//     println!("node1: {:#?}, node2: {:#?}", node1, node2);
//
//     // annot borrow as mutable
//     // let node5 = Node::new(5);
//     // let mut node3 = node1.get_downstream().unwrap();
//     // node3.update_downstream(Rc::new(node5));
// }

fn t3() {
    // RefCell 值是只读的，但在运行时，这个值可以得到可变借用，从而修改内部的数据
    let data = RefCell::new(1);
    {
        let mut v = data.borrow_mut();
        *v += 1;
    }
    println!("data: {:?}", data.borrow());
}

#[derive(Debug)]
struct Node {
    id: usize,
    downstream: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(id: usize) -> Self {
        Node {
            id,
            downstream: None,
        }
    }

    pub fn update_downstream(&mut self, downstream: Rc<RefCell<Node>>) {
        self.downstream = Some(downstream);
    }

    pub fn get_downstream(&self) -> Option<Rc<RefCell<Node>>> {
        self.downstream.as_ref().map(|v|v.clone())
    }
}

fn t2() {
    // Rc 只读的引用计数器
    let mut node1 = Node::new(1);
    let mut node2  = Node::new(2);
    let mut node3 = Node::new(3);
    let node4 = Node::new(4);

    node3.update_downstream(Rc::new(RefCell::new(node4)));
    node1.update_downstream(Rc::new(RefCell::new(node3)));
    node2.update_downstream(node1.get_downstream().unwrap());
    println!("node1: {:#?}, node2: {:#?}", node1, node2);

    let node5 = Node::new(5);
    let node3 = node1.get_downstream().unwrap();
    node3.borrow_mut().downstream = Some(Rc::new(RefCell::new(node5)));
    println!("node1: {:#?}, node2: {:#?}", node1, node2);
}

// Rc RefCell 单线程 效率高
// Arc/Mutex/RwLock 多线程