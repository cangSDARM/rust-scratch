//std::cmp::PartialOrd trait 可以实现类型的比较功能

// trait 类似于接口(interfaces)的功能，但有一些不同。
// 而 trait bounds 指定泛型是任何拥有特定行为的类型。

/// trait 的定义
pub trait Summary {
  fn summarize(&self) -> String; // trait 中的方法签名
}
