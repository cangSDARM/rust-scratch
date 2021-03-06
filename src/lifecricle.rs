// Rust 中的每一个引用都有其 生命周期（lifetime），也就是引用保持有效的作用域。
// 大部分时候生命周期是隐含并可以推断的，正如大部分时候类型也是可以推断的一样。

/// 生命周期注解的逻辑类似于泛型
/// - 不同于其他泛型帮助我们确保类型拥有期望的行为，生命周期注解有助于确保引用在我们需要他们的时候一直有效
/// - 其**并不改变任何引用的生命周期的长短**。当指定了生命周期注解后，函数也能接受任何生命周期的引用。
///
/// 例子：
/// ```rust
///   &i32        // 引用
///   &'a i32     // 带有显式生命周期的引用
///   &'a mut i32 // 带有显式生命周期的可变引用
/// ```
#[allow(unused_variables)]
fn _longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  //现在函数签名表明
  //  对于一定的生命周期 'a，函数获取两个参数，他们在生命周期 'a 中有效，且返回的字符串 slice ，生命周期 'a 中同样也需要有效
  //  'a 所替代的生命周期是 min(x的作用域, y的作用域)
  //  这样保证了**return的值在该生命周期内有效**，以避免出现对较长生命周期对象的悬垂引用
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
//当从函数返回一个引用，返回值的生命周期参数需要与一个参数的生命周期参数相匹配（否则编译器会猜测返回值是在函数中声明的

/// 带引用的结构体
///
/// 该注解意味着`ImportantExcerpt`的实例不能比`part`中的引用存在的更久
#[allow(dead_code)]
struct ImportantExcerpt<'a> {
  part: &'a str,
}
/// 带生命周期注解的 impl 块
///
/// 类似于泛型，impl和类型后都需要写
///
/// 该例子适用于下面的省略规则的1,3
impl<'a> ImportantExcerpt<'a> {
  fn _level(&self, another: &str) -> &str {
    format!("{}", another);
    self.part
  }
}

/// 生命周期注解省略规则(lifetime elision rules)
///
/// 该规则使得可以省略写生命周期注解，适用于*fn, impl*
///
/// 1. 每一个**引用参数**都有不同的生命周期。即：
///   有一个引用参数的：`fn foo<'a>(x: &'a i32)`，
///   有两个引用参数的：`fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`，
///   以此类推
/// 2. 如果只有一个引用参数，那么输入输出生命周期相同：
///   `fn foo<'a>(x: &'a i32) -> &'a i32`
/// 3. 如果方法有多个引用参数，且包含`&self`或`&mut self`，那么`self`的生命周期被约束给所有返回引用
///   *参考目录中写到*：第三条规则真正能够适用的就只有方法签名
#[allow(unused_variables)]
fn _elision(s: &str) -> &str {
  &s[..]
}

/// 静态生命周期注解
/// 生命周期能够存活于整个程序期间
/// 所有的字符串字面值都拥有 'static 生命周期
#[allow(unused_variables)]
fn _static_life() {
  //这是字面量，参看(../doc/base_type.md)
  let s: &'static str = "I have a static lifetime.";
}

use std::fmt::Display;
/// 生命周期注解、trait、泛型和引用共同使用的例子
///
/// 用于返回两个字符串 slice 中较长者
fn _longest_with<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
  T: Display,
{
  println!("Announcement! {}", ann);
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
