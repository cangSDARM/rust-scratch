# Structure

[返回](../README.md)

Rust 中，是基于结构体构建对象的

```rust
struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
} //声明

let user1 = User {
  email: String::from("someone@example.com"),
  username: String::from("someusername123"),
  active: true,
  sign_in_count: 1,
};  //实例

let email = user1.email;
```

> 若是可变的，则整个实例必须是可变的; Rust 不允许只将某个字段标记为可变

**变量与字段同名时的字段初始化**

```rust
fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}
```

**结构体更新语法可以从其他实例创建实例**

```rust
let user2 = User {
  email: String::from("another@example.com"),
  username: String::from("anotherusername567"),
  active: user1.active,
  sign_in_count: user1.sign_in_count,
};
```

> 注意这会破坏 user1 的所有权。参看后面的“结构体所有权”来修复

**对象解构(Rust 中称结构体更新)**

```rust
let user2 = User {
  email: String::from("another@example.com"),
  username: String::from("anotherusername567"),
  ..user1
};
```

> 注意 Rust 中是**两个**点<br>
> 这也会破坏所有权(因为这只是上一种写法的语法糖)，参看后面的“结构体所有权”来修复

**没有命名字段的元组结构体**

```rust
struct S(String);
struct Color(i32, i32, i32);
struct Paint(i32, i32, i32);  //定义的每一个结构体有其自己的类型，即使结构体中的字段有着相同的类型
```

> 即是对元组命名，方便调用。使用同元组

**也可以定义一个没有任何字段的结构体，其被称为类单元结构体**

```rust
struct Union;
```

> 类单元结构体(unit-like structs)的名字，是因为其类似于`()`即**Unit**类型

**结构体的打印并不和基本类型相似**

```rust
#[device(Debug)]  //需要这个才能打印。这个称为trait注解。参看`../src/traits.rs`
struct User{}

{
  print!("{}", user1); //Error!
  print!("{:?}", user1);  //Ok
  print!("{:#?}", user1); //Ok, more prety
}
```

## 结构体的所有权

在 User 结构体的定义中，我们使用了自身拥有所有权的`String`类型而不是`&str`类型。这是一个有意而为之的选择，因为我们想要这个结构体拥有它所有的数据，为此只要整个结构体是有效的话其数据也是有效的。
可以使结构体存储被其他对象拥有的数据的引用，这么做的话需要用上生命周期(lifetimes)。(参看[生命周期](../src/lifecricle.rs))

## 方法

```rust
struct User();

impl User {
  fn signin() {
    //静态函数，使用 User::signin()调用
    //Rust 称为关联函数
  }

  fn login(&self) -> u32 {
    //方法函数，使用 User.login()调用
    self.active = true;
    1
  }
}

//Implement 可以有多个
impl User{
  fn exit(self) {}
}
```

> 方法第一个参数是 self，和 python 类似<br>
> 同样，如果单独是`self`会修改所有权，`&self`是引用，`&mut self`是可变引用

**当使用 object.something() 调用方法时，Rust 会自动为 object 添加 &、&mut 或 \* 以便使 object 与方法签名匹配，称为自动引用和解引用**

```rust
user1.login();
(&user1).login();
```

> 上面两句是完全等价的。Rust 可以明确地计算出方法是仅仅读取(&self)，做出修改(&mut self)或者是获取所有权(self)
