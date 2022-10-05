// Async
// Rust 的 并发模型 是懒启动的。没有await则不会调用
// 并发模型需要调度器、执行器配合。比较出名的调度器runtime有：std::async、tokio、future

// 执行器三种写法：

// 1. raw
fn task1() -> impl Future<Output = ()> {
  sleep(Duration::new(5)).await;
}

// 2. async/awit 即上面的语法糖
async fn task2() {
  sleep(Duration::new(5)).await;
}

// 3. 自己实现 Future 的 pending/ready 模型
struct Asyncer {
  timeout: Instant,
}
impl Future for Asyncer {
  type Output = ();
  // 需要调度器调用poll。
  //   返回 pending 时，调度器将Asyncer挂载(pin)到cx.waker上，等待waker.wake调用
  //   返回 ready 时，调度器返回 Output 的数据
  fn poll(self: Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    if Instant::now() >= self.timeout {
      Poll::Ready(())
    } else {
      let waker = cx.waker().clone();
      let timeout = self.timeout;
      std::thread::spawn(move || {
        let curTime = Instant::now();
        if curTime < self.timeout {
          std::thread::sleep(self.timeout - curTime);
        }
        waker.wake();
      });
      Poll::Pending
    }
  }
}
