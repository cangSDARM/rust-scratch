# 特殊类型

[返回](../README.md)

## Newtype

[Newtype 模式](../src/traits.rs)

## 类型别名

类型别名(type alias)意味着别名和真实类型是同义词(synonym), 这两个类型是完全可以替代使用的

类型别名的主要用途是减少重复

```rust
type Thunk = Box<dyn Fn() + Send + 'static>;
let f: Thunk = Box::new(|| println!("hi"));
fn toke() -> Thunk {}
fn gave(f: Thunk) {}
```

类型别名系统类似于 Typescript 中的, 也可以用泛型

```rust
type Result<T> = std::result::Result<T, std::io::Error>;
```

## Nevertype

rust 中也存在一个 never 的类型(称为 empty type), 即`!`

Nevertype 表示一个函数**从不返回值**(和[void](../src/main.rs)区别, void 返回 void, void 确实是个类型), 这种从不返回值的函数叫做发散函数(diverging functions)

```rust
fn bar() -> ! {}
```

Nevertype 常常用来绕过类型系统检查, 如`match`分支中(此时可以认为, **Nervertype 被强转为了其他任意类型**)

```rust
// match 分支要求返回类型相同. 这里`continue`返回了个Nevertype
let guess = match guess.trim().parse() {
  Ok(_) => 5,
  Err(_) => continue, //continue只能在loop里用. 这里忽略了大部分上下文
}
```

常见的返回 Nervertype 的: `panic!`, `loop`, `continue`

## 动态大小的类型

**Rust 编译器需要知道每个函数的返回类型需要多少空间**, 这意味着所有函数都必须返回一个具体类型

但如同`str`, 直到运行时都不知道大小的类型, Rust 也有部分动态大小类型(dynamically sized types, DST)

```rust
let s: str = "str?";
```

> 但 DST 的变量**是无法直接声明的**, 上面的 s 的类型应该是`&str`. `&str`的长度是固定的(usize 的两倍), 存储了字符串长度及头地址

所以, 动态大小类型的总是满足: 必须将动态大小类型的值置于某种指针之后. 而 Rust 的解决办法通常是用一些额外的元信息来储存动态信息的大小

### dyn

`dyn`关键字用于附加一些元信息来扩展指针(如长度等), 来将动态大小约束为固定大小

通常有: `&dyn Trait`, `Box<dyn Trait>` 或 `Rc<dyn Trait>`:

```rust
// Returns some struct that implements Animal, but we don't know which one at compile time.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
  if random_number < 0.5 {
    Box::new(Sheep {})
  } else {
    Box::new(Cow {})
  }
}
```

不是所有的 trait 都能使用 dyn, 只有**对象安全**的才行:

- 返回值类型不为`Self`
- 方法没有任何泛型类型参数

### Sized

对于自己声明的 Trait, Rust 使用`Sized` trait 来限制 Trait.

这个 trait 使得编译器在编译时就知道类型实现时的大小. 而且对于泛型来讲, 它是隐式实现的

```rust
fn generic<T>(t: T) { }
//In compiler time:
fn generic<T: Sized>(t: T) {}
```

这个限制可以使用`?Trait`来放宽

```rust
fn generic_dst<T: ?Sized>(t: &T) {}
```

> 放宽后的参数需要约束, 这里选择了 T 的引用
