/// 定义匿名函数
///
/// 定义以一对 || 开始，|| 中间是匿名函数的参数列表(与Smalltalk和Ruby的匿名函数定义类似)
/// 之后接闭包体的 {}
/// 
/// let a, 意味着有一个匿名函数的声明, 不是代表匿名函数的返回值。而匿名函数的返回值应在使用时被接收
let a = |num:i32, st: String| ->() {
};

/// 匿名函数不要求像 fn 函数那样在参数和返回值上注明类型。因为匿名函数并不用于暴露在外作为接口使用：他们储存在变量中并被使用，不用命名他们或暴露给库的用户调用
let c = |st| st.to_string();

/// 如果匿名函数在一行上面，{} 可以省略
let b = |st| st;

/// 匿名函数的调用：let result = a(num, st)
 
/// 第一次使用 特定类型 值调用时，编译器将推断参数和返回值的类型。
/// 接着这些类型被锁定进该匿名函数中。之后如果尝试对同一匿名函数使用不同类型则会得到类型错误
let x = b(5);
let y = b(String::from("h")); //Panic

/// 该结构体只会在需要结果时执行匿名函数，并会缓存结果值。
/// 这样余下的代码就不必再负责保存结果并可以复用该值。这种模式被称 memoization 或 lazy evaluation
/// 
/// 注意：**每一个匿名函数实例有其自己独有的匿名类型**，因此需要使用泛型和 trait bound。而匿名函数(和函数)都实现了`Fn`/`FnMut`/`FnOnce`中的一个
struct Cacher<T> where T: Fn(u32) -> u32 {
  calculation: T,
  value: Option<u32>,
}
impl<T> Cacher<T> where T: Fn(u32) -> u32 {
  fn new(calculation: T) -> Cacher<T> {
    Cacher {
        calculation,
        value: None,
    }
  }
  fn value(&mut self, arg: u32) -> u32 {
    //memoization 或 lazy evaluation 的简易实现。复杂的请修改value为HashMap或者其他的类型
    match self.value {
      Some(v) => v,
      None => {
        let v = (self.calculation)(arg);
        self.value = Some(v);
        v
      },
    }
  }
}

/// 由于所有匿名函数都可以被调用至少一次，所以所有匿名函数都实现了 FnOnce
/// 没有移动闭包所有权到函数体内的匿名函数实现了 FnMut
/// 不需要对闭包进行可变访问的匿名函数则实现了 Fn
/// 
/// 如果你希望强制匿名函数获取其闭包的所有权，可以在其前面加上 `move` 关键字
let d = move || x;