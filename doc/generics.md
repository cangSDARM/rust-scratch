# 泛型
## 简单泛型
```rs
/// 泛型strict
struct Point<T> {
  x: T,
}

/// 泛型枚举
enum Optionx<T> {
  SOME(T),
  NONE,
}

/// 泛型函数
fn generics<T>(list: &[T]) -> &T {
  //CamelCase for type
  &list[0]
}
```

**泛型impl**

```rs
/// 泛型方法
impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}
```
> 注意必须在 impl 后面声明 T 。在 impl 之后声明泛型 T ，表示 Point 的尖括号中的类型是泛型而不是具体类型

也可以对具体对类型实现方法，该方法块将只有该类型的才有

```rs
/// 该方法块只有 i32 类型的才有
impl Point<i32> {
  fn dis(&self) -> i32 {
    (&self).x
  }
}
```
> 注意需要扩号的原因是，指定是对self的引用，而不是 self.x 的