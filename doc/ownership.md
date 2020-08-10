# 所有权Ownership
[返回](../README.md)

Rust通过所有权管理内存，使得无需Rust垃圾回收，也不用像c那样复杂地手动管理

### Scratch
- 栈 stack
    + 后进先出，操作速度快
    + 每个对象占据同样大小空间，耗费空间
- 堆 heap
    + 通过指针访问，操作速度慢
    + 每个对象占据对应大小，节省空间


## 规则
1. 每一个值都有一个**所有者变量**
2. 且值**有且只有一个**所有者
3. 当**所有者变量**离开作用域，值将被丢弃

## 要点
**内存在拥有它的变量离开作用域后就被自动释放**
```rs
{
  let mut s = String::from("s");  //此处起，s是有效的
} //调用 `drop` 函数
```
>在 C++ 中，这种 item 在生命周期结束时释放资源的模式有时被称作 资源获取即 初始化(Resource Acquisition Is Initialization (RAII))

**但有时会不可测，使用`移动`解决**
```rs
{
  let s1 = String::from("s");
  {
    let s2 = s1;  //s2 从 s1 copy数据
    //若拷贝栈(指针、长度等)和堆(String内容)，则会产生性能影响
    
    print!("{}", s1); //因此Rust阻止在 s1 被拷贝后的访问。此处 s1 实际上已经不存在
  }
} //若只拷贝栈数据，则此处产生double free错误
```
> 有鉴于此，**Rust 永远也不会自动创建数据的 “深拷贝”**

**但确实有深拷贝的需求，则使用`克隆`**
```rs
{
  let s1 = String::from("s");
  let s2 = s1.clone();  //`clone`是通用函数

  print!("{}", s1); //OK
}
```
> 克隆只是给了一个解决方案，依旧会耗费性能的

**对于完全在栈上的数据，可以不用克隆。会自动克隆**
```rs
{
  let x = 13;
  let y = x;  //auto `clone`, 但是不是这个名，详见下面内容
  
  print!("{}, {}", x, y);  //Ok
}
```
> Rust 有 `Copy` trait 的特殊注解，可以用在完全存储在栈上的类型上。如果一个类型拥有 `Copy` trait，旧的变量在将其赋值给其他变量后其仍然可用。Rust不允许自身或其任何部分实现了 `Drop` trait 的类型使用 `Copy` trait。如果对其值离开作用域时需要特殊处理的类型使用 `Copy`，将会触发编译时错误。要为你的类型增加 Copy 注解，请阅读“可派生的 trait”

**对于函数调用来说，以上规则同样适用。向函数传递值可能会移动或者复制，就像赋值语句一样**
```rs
fn main() {
  let s = String::from("hello");  // s 进入作用域
  takes_ownership(s); // s 的值移动到函数里
  // 所以到这里不再有效

  let x = 5;  // x 进入作用域
  makes_copy(x);  // x 应该移动函数里，
  // 但 i32 是 Copy 的，所以在后面可继续使用 x

} // 这里 x 先移出了作用域，然后是 s。但因为 s 的值已被移走，所以不会有Drop触发

fn takes_ownership(string: String) { // string 进入作用域
  println!("{}", string);
} // 这里，string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(integer: i32) { // integer 进入作用域 
  println!("{}", integer);
} // 这里，integer 移出作用域。不会有特殊操作
```
> 尝试在调用 takes_ownership 后使用 s 时，Rust 会抛出一个编译时错误。

**如何解决函数的所有权移交问题呢？下面有个例子(Rust强调纯函数概念)**
```rs
fn main() {
  let s1 = gives_ownership(); // gives_ownership 将返回值移给 s1
  let s2 = String::from("hello"); // s2 进入作用域
  let s3 = takes_and_gives_back(s2);  // s2 被移动到 takes_and_gives_back 中, 它也将返回值移给 s3
} // 这里, s1/s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，所以什么也不会发生

fn gives_ownership() -> String { // gives_ownership 将返回值移动给调用它的函数
  let string = String::from("hello"); // string 进入作用域.
  return string; // 返回 string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(string: String) -> String { // string 进入作用域
  string // 返回 string 并移出给调用的函数 
}
```

**变量的所有权总是遵循相同的模式:将值赋给另一个变量时移动它**<br>
**并且注意，函数复杂的所有权管理可以通过引用解决**

## Ref。引用
即对引用对象增加指针。该指针**不变更所有权**
```rs
fn main() {
  let s1 = String::from("S");

  //非引用版本
  {
    let (s2, len) = non_ref(s1);  //通过 move 解决
    print!("{}, {}", s2, len);
  }

  //引用版本
  {
    let len = use_ref(s1);  // s1 所有权仍在main
    print!("{}, {}", s1, len);
  }
}

fn non_ref(s :String) ->(String, usize) {
  return (s, s.len());
}

fn use_ref(s :&String) ->usize {  // s 称为 借用(borrow)
  return s.len();
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权
```

**但引用默认是不可修改的，用mut解决，称为可变引用**
```rs
{
  let x = String::from("x");
  let y = &mut x; //borrow
  let z = &mut x; //Error! 在特定作用域中的特定数据有且只有一个可变引用
}
```
> 这个限制的好处是 Rust 可以在编译时就避免数据竞争<br>
> 当然，多个不可变引用同时存在是可以的

**在有不可变引用时，不能同时拥有可变引用**
```rs
{
  let mut x = String::from("x");
  let y = &x;
  let z = &x;
  let e = &mut x; //Error! 
}
```

**引用必须总是有效。否则会触发悬垂引用(无效指针)，并报错**
```rs
fn dangle() -> &String { // dangle 返回一个字符串的引用
  let s = String::from("hello"); // s 是一个新字符串
  &s // 返回字符串 s 的引用。改成返回String就不会出错了
} // 这里 s 离开作用域并被丢弃。其内存被释放。但 s 的引用被返回，因此会报错
```

**引用取值采用解引用运算符(dereference operator)`*`来取值**
```rs
let y = &String::from("hello");
assert_eq!("hello", *y);
```

## Slice。切片
slice允许引用集合中一段**连续的**元素序列，而不用引用整个集合。slice也**不会变更所有权**
```rs
{
  let s = String::from("hello world");
  let hello = &s[0..5];  //取 0-5，但不包括5
  let hello = &s[..5];  //和上面一样。并且hello被shadow了
  let world = &s[6..=10];  //取 6-10，且包括10

  let end :&str = &s[1..];  //取 1-s.len()
  let sr :&str = &s[..];  //取 0-s.len()
}
```
> slice 的数据结构存储了其开始位置指针和长度<br>
> 字符串 slice 的索引必须位于有效的 UTF-8 字符边界内。如果尝试从一个多 字节字符的中间位置创建字符串 slice，则将会因错误而退出

**除了字符串可以slice外，其他的slice也是相似的**
```rs
{
  let a = [1, 2, 3, 4];
  let as = &a[..];  //as 的类型是 &[i32]
}
```
