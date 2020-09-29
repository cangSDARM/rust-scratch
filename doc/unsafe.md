# 不安全Rust
[返回](../README.md)

**不安全**并不意味着完全关闭Rust的编译器检查, 只是允许以下几点可以使用:

- [不安全Rust](#不安全rust)
    - [解引用裸指针](#解引用裸指针)
    - [调用不安全的函数或方法](#调用不安全的函数或方法)
      - [创建不安全代码的安全抽象](#创建不安全代码的安全抽象)
      - [使用 extern 函数调用外部代码](#使用-extern-函数调用外部代码)
      - [使用 extern 函数, 外部语言调用Rust](#使用-extern-函数-外部语言调用rust)
    - [访问或修改可变静态变量](#访问或修改可变静态变量)
    - [实现不安全 trait](#实现不安全-trait)
    - [访问 union 的字段](#访问-union-的字段)

### 解引用裸指针
裸指针(raw pointers)是类似于引用的新类型, 分为可变(`*mut T`)和不可变的(`*const T`). *不可变*意味着指针解引用之后不能直接赋值. 裸指针满足:

- 忽略借用规则, 可以同时拥有不可变和可变的指针, 或多个指向相同位置的可变指针
- 不保证指向有效的内存
- 允许为空
- 不能实现任何自动清理功能

> 注意: 可以在安全代码中**创建**裸指针，只是不能在unsafe块之外**解引用**裸指针
```rs
//通过引用创建裸指针
let mut num = 5;
let r1 = &num as *const i32;    //不可变
let r2 = &mut num as *mut i32;  //可变的

//指向任意内存地址的裸指针
let address = 0x01234usize;
let r = address as *const i32;
```

> 解引用
```rs
unsafe {
    //only support in unsafe
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}
```

之所以这么设计, 是因为: 创建一个指针不会造成任何危险; 只有当访问其指向的值时才有可能遇到无效的值

### 调用不安全的函数或方法
在此上下文中, 关键字`unsafe`表示该函数具有调用时需要满足的要求, 而 Rust 不会保证满足这些要求

```rs
unsafe fn dangerous() {}

unsafe {
    dangerous();
}
```

#### 创建不安全代码的安全抽象
通常实践中, 将不安全代码封装进安全函数是一个常见的抽象, 而不是暴露一个unsafe的函数
```rs
//例, vec的split_at_mut函数
use std::slice;
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr: *mut i32 = slice.as_mut_ptr();   //访问 slice 的裸指针

    assert!(mid <= len);

    //错误的实践
    //(&mut slice[..mid], &mut slice[mid..]), 借用同一slice不重叠的两部分是可以的, 但是编译器无法理解

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.add(mid), len - mid))
    }
}
```

#### 使用 extern 函数调用外部代码
类似于Cpp, Rust使用extern块来创建和使用外部函数接口(Foreign Function Interface， FFI)

extern 块中声明的函数在 Rust 代码中总是不安全的

> 如调用 "C" 中的函数
```rs
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```
> 其中 "C" 是指外部函数所使用的应用二进制接口(application binary interface，ABI). 即如何在汇编语言层面调用此函数. "C"是最常见的, 指代C语言的 ABI

#### 使用 extern 函数, 外部语言调用Rust
```rs
#[no_mangle]    //意指禁止编译器优化函数名
pub extern "C" fn call_from_c() {
    //这里的 "C" 语义和上面相同
    println!("Just called a Rust function from C!");
}
```

### 访问或修改可变静态变量
访问和修改可变静态变量都是不安全的

不安全意味着可能导致数据竞争, 应尽量避免. 或通过[并发](../src/concurrent.rs)或[线程安全的智能指针](../src/smart_pointer.rs)替代

```rs
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
```

### 实现不安全 trait
当至少有一个方法中包含编译器不能验证的不变量时 trait 是不安全的

> 在 trait 之前增加 unsafe 关键字将 trait 声明为 unsafe, 同时该 trait 的实现也必须标记为 unsafe
```rs
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
```

### 访问 union 的字段
Rust中 union 主要用于和 C 中的union交互.

访问 union 中的字段是不安全的, 因为 Rust 无法保证当前存储在联合体实例中数据的类型

[参看](https://doc.rust-lang.org/reference/items/unions.html)
