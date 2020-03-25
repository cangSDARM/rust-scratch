use rand::Rng;
use std::io; //和cpp类似，但是用的use

mod generics;
mod lifecricle;
mod process_control; //使用process_control模块(和文件名相同)
mod traits;

// main 函数是主入口函数，和dart类似
#[allow(dead_code)]
fn main() {
    const MAX_SIZE: u32 = 100_0000;
    // const创建常量
    // 常量必须指定类型
    // 常量无法是：函数返回值或其他在运行时确定的结果
    let mut changeable: String = String::new();
    // 使用let创建变量
    // 但变量是不可变的，即使是基本数据类型
    // mut 使得变量可变
    // :String 声明changeable的类型
    // ::new 说明new是String类型的关联函数，即静态方法
    io::stdin().read_line(&mut changeable).expect("Failed");
    // 若没有引入std::io，则可以使用`std::io::stdin.read_line`
    // & 表示引用，和cpp的作用相同。但默认依旧是不可变的
    // mut 使得引用可变
    // 其read_line返回 Result 枚举，其用于处理错误或成功回调，包含 OK、Err，有点像js的Promise
    println!("{}, {}!", "Hello", "World");
    // ! 表示println是个宏
    // {} 表示格式化字符串。类似于js中的${}
    let changeable: i32 = changeable.trim().parse().expect("a number");
    // 这里表示隐藏(shadow)之前的值，但复用其变量名，通常用于类型转换
    randint(changeable);
}

// fn 函数名(参数 :参数类型) -> 返回值类型 {}
// 使用()代表void
fn randint(max: i32) -> i32 {
    return rand::thread_rng().gen_range(1, max);
}
