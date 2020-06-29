///
/// 对象:
/// - Rust 结构体和枚举包含数据而 impl 块提供了在结构体和枚举之上的方法
/// 封装:
/// - 在代码中不同的部分使用 pub 与否可以封装其实现细节
/// 继承:
/// - Rust 无法定义一个结构体继承父结构体的成员和方法, 但使用默认 trait 方法实现来进行共享
/// 多态:
/// - 对于继承来说, 这些类型通常是子类. Rust 则通过泛型来对不同的可能类型进行抽象, 并通过 trait bounds 对这些类型所必须提供的内容施加约束(这有时被称为 bounded parametric polymorphism)

/// trait bound 的泛型类型参数一次只能替代一个具体类型, 而 trait object 可以包含同类型的类型
pub trait Draw {
  fn draw(&self);
}
// 使用 trait object, Rust 要求[对象安全(object safe)](base_type.md)的 trait, 且使用动态分发(dynamic dispatch)即运行时确定调用什么方法的代码
pub struct Screen {
  pub components: Vec<Box<dyn Draw>>,
}
impl Screen {
  pub fn run(&self) {
      for component in self.components.iter() {
          component.draw();
      }
  }
}

// 使用 trait bound, Rust 使用静态分发(static dispatch)对代码进行[单态化](generics.rs)
pub struct Screen<T: Draw> {
  pub components: Vec<T>,
}
impl<T> Screen<T>
  where T: Draw {
  pub fn run(&self) {
      for component in self.components.iter() {
          component.draw();
      }
  }
}
