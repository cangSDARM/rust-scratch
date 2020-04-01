use std::cmp::PartialOrd; // 该 trait 可以实现类型的比较功能
use std::fmt::Display; // 该 trait 用于显示(print之类的)

// trait 类似于接口(interfaces)的功能，但有一些不同。
// 而 trait bounds 指定泛型是任何拥有特定行为的类型。

/// trait 的定义
pub trait Summary {
  /// 方法签名, 必须实现的方法
  fn summarize(&self) -> String;
  /// 默认实现了的方法
  ///
  /// - 如果想要对 X 实例使用默认实现，而不是定义一个自己的实现，
  ///     可以通过`impl Summary for X {}`指定一个空的 impl 块
  ///
  /// - 默认实现可以允许调用相同 trait 的其它方法，**即使这些方法没有默认实现**
  fn summary(&self) -> String {
    format!("{}", self.summarize())
  }
}

struct News {
  content: String,
}

struct Tweet {
  retweet: String,
}

/// 实现 trait
impl Summary for News {
  fn summarize(&self) -> String {
    format!("{}", self.content)
  }
  /// 对默认实现的重载
  fn summary(&self) -> String {
    String::from("重载")
  }
}

/// 不能为外部类型实现外部 trait(有一个位于本地即可)。
/// 例如：不能为 Vec<T> 实现 Display trait。这是因为 Display 和 Vec<T> 都定义于标准库中，而并不位于本地作用域中。这个限制是被称为相干性(coherence)的程序属性的一部分，即孤儿规则(orphan rule)
impl Summary for Tweet {
  fn summarize(&self) -> String {
    self.retweet.clone()
  }
}

/// 限定参数必须实现某些 trait
/// - item 需要实现 Summary
/// - item2 需要实现 Summary 和 PartialOrd
fn _notify(item: impl Summary, _item2: impl Summary + PartialOrd) {
  println!("{}", item.summarize())
}
/// 同样效果。称为**T 的 trait bound**
///
/// 这样的实现可以**限定多个参数类型为同一个**。而第一种只需要参数实现，并不限制类型相同
///
/// 同上，有 `<T: Summary + PartialOrd`
///
/// 另一种写法：
///   ```rust
///   fn _nofified<T, U>(t:T, u:T) -> i32
///     where T: Display + Summary, U: Display + Debug{}
///   ```
fn _nofified<T: Summary>(item: T) {
  println!("{}", item.summary())
}

/// 限定返回值必须实现某些 trait
///
/// 缺点是只适用于返回单一类型的情况
///   ```rust
///   // 这样会报错，无法实现 单态化或类似的东西
///   fn _nofifily(switch: bool) -> impl Summary{
///     if switch{
///       return News{};
///     } else {
///       return Tweent{};
///     }
///   }
///   ```
fn _nofifily(item: impl Summary) -> impl Summary {
  item
}

#[allow(dead_code)]
struct Point<T> {
  x: T,
}

/// 有条件的泛型方法
///
/// 实现了`Display`的 T 才会有该方法块
impl<T: Display> Point<T> {
  fn _cmp(&self) {
    print!("{}", (&self).x)
  }
}

/// 有条件地实现 trait
///
/// 实现了`Display`的 T 就会有`Summary` trait 的方法
impl<T: Display> Summary for T {
  fn summarize(&self) -> String {
    String::from("1")
  }
}


/// 有些 trait 经常被实现，且实现时逻辑相似。因此Rust有快速实现这些的语法糖，称为 可派生的trait
/// 使用 #[derive()] 属性包裹
#[derive(Display)]
struct Dis {}