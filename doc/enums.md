# 枚举

[返回](../README.md)

```rust
enum Ip{
  V4,
  V6,
}

let v4 = Ip::V4;
let v6 = Ip::V6;
```

**枚举成员可以指定类型**

```rust
enum Ip{
  V4(u8, u8, u8, u8), //元组
  V6(String), //字符串
  V8 { x: i32, y: u8 },  //匿名结构体
}

let home = Ip::V4(127, 0, 0, 1);
let router = Ip::V6(String::from("::1"));
```

> 很显然，枚举中成员类型可以不一致<br>
> 可以将**任意类型**的数据放入枚举成员中:例如字符串、数字类型或者结构体。也可以包含另一个枚举

**枚举也能定义方法**

```rust
impl Message {
  fn call(&self) {
    // 在这里定义方法体
  }
}
let m = Ip::V6(String::from("hello"));
m.call();
```

### match 中的枚举

更详细的`match`信息，请查看：[process_control](../src/process_control.rs)

```rust
enum Coin{
  Penny,
  Quarter(Ip),
}

match coin {
  Coin::Penny => 12,
  Coin::Quarter(ip) => {
    print!("{:?}", ip);
    1
  }
}
```

> 其中，ip 是绑定的 Ip 类型(自动引用和解引用)。Penny 没有就自然没写
