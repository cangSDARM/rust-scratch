///并发编程(Concurrent programming), 代表程序的不同部分相互独立的执行
///并行编程(Parallel programming)代表程序不同部分于同时执行
// Rust 标准库只提供了 1:1 线程模型实现 (Rust线程:OS线程 = 1)
use std::thread;
use std::time::Duration;

/// 线程基础
fn spawn_thread() {
  let v = vec![1, 2, 3];

  let handle = thread::spawn(move || {
    //使用move捕获闭包需要内容的所有权
    println!("{:?}", v);
    //线程运行的代码 (伴随线程)
    for i in 1..10 {
      println!("hi number {} from the spawned thread!", i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  //handle 是拥有所有权的值, 当对其调用 join 方法时, 它会等待其线程结束
  handle.join().unwrap();
}

/// 消息传递共享数据(message passing)
/// - 发送或接受任意一端断开则结束
/// - 生产者并不约束发送的数据的顺序
use std::sync::mpsc; // 多个生产者，单个消费者(multiple producer, single consumer)
fn message_passing() {
  // 发送者(transmitter)和 接收者(receiver)
  let (tx, rx) = mpsc::channel();

  //通过clone发送端产生多个生产者
  let tx1 = mpsc::Sender::clone(&tx);
  thread::spawn(move || {
    let val = String::from("hi thread 1");
    tx1.send(val).unwrap();
  });

  thread::spawn(move || {
    let val = String::from("hi thread 2");
    tx.send(val).unwrap(); //send移动所有权
  });

  let received = rx.recv().unwrap(); //block style
  /// let received = rx.try_recv().unwrap();  //unblock style. Ok 值包含可用信息, Err 值代表此时没有任何消息, 自己轮询
  print!("Got:{}", received);
}

/// 共享状态(shared state)
/// - Rust 不能(自动)解决deadlock, 而只是(自动)解决了锁的获取和释放以及原子操作的安全性
/// 这里使用了一个共享内存的例子: 互斥器(mutual exclusion, mutex)和原子引用计数(atomically reference counted)
use std::sync::{Arc, Mutex, MutexGuard};
fn shared_state() {
  //[Mutex提供了内部可变性](smart_pointer.rs)
  let m = Mutex::new(5);

  {
    //阻塞直到获取锁
    let mut num: MutexGuard = m.lock().unwrap();
    *num = 6;
  } //离开作用域时自动释放锁

  print!("{:?}", m);
}

fn shared_state1() {
  // 原子级Mutex, 类似于Rc, 使得多线程可以引用同一数据, 但有性能惩罚
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];

  for _ in 0..10 {
    // 增加引用计数
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
      let mut num = counter.lock().unwrap();

      *num += 1;
    });
    handles.push(handle);
  } //Drop 自动减少引用计数

  for handle in handles {
    handle.join().unwrap();
  }

  println!("Result: {}", *counter.lock().unwrap());
}

/// 可扩展的并发逻辑实现
/// - Rust 的并发并不依赖于语言本身, 只是恰巧生命周期对相关逻辑做了有趣限定
///   但仍有两个并发相关的概念: Sync 和 Send (之后又有async await :)

//Send(trait) 表明类型的所有权可以在线程间传递,
// - 几乎所有基本类型都是`Send`的(除了裸指针raw pointer)

//Sync(trait) 表明类型可以安全的在多个线程中拥有其引用,
// - Send的都是Sync的
// - Sync的引用可以发送给其他线程

// 手动实现`Send`和`Sync`是不安全的, 除非你清楚的了解相关概念.
// 相关参考: [The Rustonomicon](https://doc.rust-lang.org/stable/nomicon/#the-rustonomicon)
