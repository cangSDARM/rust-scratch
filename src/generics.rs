/// 泛型枚举
#[allow(dead_code)]
enum Optionx<T> {
  SOME(T),
  NONE,
}

/// 泛型函数
///
/// 参见`traits.rs`中的*trait bound*来消除 引用。T 需实现 Copy/Clone trait
fn _generics<T>(list: &[T]) -> &T {
  &list[0]
}

/// 泛型strict
#[allow(dead_code)]
struct Point<T, U> {
  x: T,
  y: U,
}

/// 泛型方法
///
/// 注意**必须在 impl 后面声明 T**。在 impl 之后声明泛型 T ，表示 Point 的尖括号中的类型是泛型而不是具体类型
impl<T, U> Point<T, U> {
  fn _x(&self) -> &T {
    &self.x
  }
}

/// 非泛型方法。该方法块只有 <i32, i32> 类型的才有
impl Point<i32, i32> {
  fn _dis(&self) -> i32 {
    (&self).x //注意需要扩号的原因是，指定是对self的引用，而不是 self.x 的
  }
}

/// 泛型方法和泛型函数的混合用例。
/// - 注意<T,U>是对方法的泛型，<V>是对函数的泛型。
/// - 其泛型作用域不尽相同。因此可以在*作用域规则下*杂糅不同的泛型
impl<T, U> Point<T, U> {
  fn _mixup<V>(self, other: Point<V, U>) -> Point<T, V> {
    Point {
      x: self.x,
      y: other.x,
    }
  }
}

use std::rc::Weak;

/// 泛型的使用
/// -  可以自动推导也可以手动指定，::<T>的形式即是指定
fn _mix() {
  let _p1 = Point { x: 5, y: 10.4 };
  let _p2 = Point { x: "Hello", y: 2.3 };
  //可以想见，p1.mixup::<&str>(p2) 结果的类型是 <i32, &str>

  let _weak: Weak<String> = Weak::<String>::new();
  //指定泛型的struct需要Struct::<...>来指定
}

//Rust 通过在编译时进行 单态化(monomorphization) 来保证泛型的效率

//更多有关泛型高级用法（涉及tarit的）参看`traits.rs`
//涉及生命周期的，参看`lifecricle.rs`
