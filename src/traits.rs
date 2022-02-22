use std::cmp::PartialOrd; // 该 trait 可以实现类型的比较功能
use std::fmt::{Display, Formatter, Result}; // 该 trait 用于显示(print之类的)

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
/// 可以使用 Newtype 模式绕过
/// 其使用一个 [元组结构体](../doc/structure.md) 来包裹外部 trait 来实现
/// 
/// 但要注意, newtype(Wrapper)没有任何被封装类型(Vec<String>)的方法, 必须自己手动实现. 或者使用 [Deref](./smart_pointer.rs) 封装函数来返回内部的类型. 同时, 要实现部分被封装类型的方法的话, 只能是**手动自己实现**
/// 
/// Newtype 也可以用来隐藏其内部的泛型类型. 如下面的 Wrapper 就隐藏了 Vec<String>
struct Wrapper(Vec<String>);
impl Display for Wrapper {
  fn fmt(&self, f: &mut Formatter) -> Result {
    // self.0 用于访问元组的第一个元素类型
    write!(f, "[{}]", self.0.join(", "))
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
///   fn _notified<T, U>(t:T, u:T) -> i32
///     where T: Display + Summary, U: Display + Debug{}
///   ```
fn _notified<T: Summary>(item: T) {
  println!("{}", item.summary())
}

/// 限定返回值必须实现某些 trait
///
/// 缺点是只适用于返回单一类型的情况
///   ```rust
///   // 这样会报错，无法实现 单态化或类似的东西
///   fn _notifily(switch: bool) -> impl Summary{
///     if switch{
///       return News{};
///     } else {
///       return Tweent{};
///     }
///   }
///   ```
fn _notifily(item: impl Summary) -> impl Summary {
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
    String::from(self.to_string())
  }
}

/// 有些 trait 经常被实现，且实现时逻辑相似。因此Rust有快速实现这些的语法糖，称为 可派生的trait
/// 使用 #[derive()] 属性包裹
#[cfg(feature = "derive")]
#[derive(Display)]
struct Dis {}

/// 父trait (supertrait)
/// 
/// 要求实现该 trait 的类型必须首先实现该 trait 的父类型.
/// 不同于 "有条件地实现 trait", 这是有条件的声明 trait
trait OutlinePrint: Display {
  fn print_outline() {
    print!("outline");
  }
}

///关联类型(associated types)
///  这类似于泛型, 但使用时无需重复的的声明类型
///  但不同于 "泛型", 这是有条件的声明 trait 
///  且只能有一个 impl AssociatedTypes for XXX
pub trait AssociatedTypes {
  type Item;

  fn next(&mut self) -> Option<Self::Item>;
}
//实现
impl AssociatedTypes for String {
  type Item = usize;
  fn next(&mut self) -> Option<Self::Item> { Some(self.len()) }
}
//使用
//  String::from("xxx").next();  //类型会被显式标注为usize, 而不是泛型的T

/// 完全限定语法(fully qualified syntax)
/// 
/// 用于处理同名的 方法及关联函数(静态方法)
/// 其优先级: self上的函数 -> 编译器第一个编译的类型 -> ...
/// 
/// 方法可以使用 Human::fly 这种来限定;
/// 关联函数需要用 完全限定语法
trait Pilot {
  fn fly(&self);
}
trait Wizard {
  fn fly(&self);
  fn name() -> String;
}
struct Human;
impl Pilot for Human {
  fn fly(&self) {
    println!("This is your captain speaking.");
  }
}
impl Wizard for Human {
  fn fly(&self) { println!("Up!"); }
  fn name() -> String { String::from("Wizard") }
}
impl Human {
  fn fly(&self) { println!("*waving arms furiously*"); }
  fn name() -> String { String::from("human") }
}
// 调用
#[allow(dead_code)]
fn same_name() {
  let person = Human;
  
  //关联函数
  //完全限定语法
  print!("name in human: {}", Human::name());
  // print!("name in wizard: {}", Wizard::name()) 这样是不行的
  print!("name in wizard: {}", <Human as Wizard>::name());
  
  //方法
  //特定的fly
  person.fly(); //默认调用 Human的fly
  Pilot::fly(&person);
  Wizard::fly(&person);
  Human::fly(&person);
  <Human as Wizard>::fly(&person); //也可以用完全限定语法
}
