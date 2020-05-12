# 模块化
[返回](../README.md)

每个crate的根模块都被隐式创建，叫做`crate`模块

```rs
mod Mod{
  fn some() {}
}

fn main(){}
```
> `Mod`模块包括some函数

**模块可以随便嵌套**
```rs
mod sound {
  mod instrument {
    mod woodwind {
      fn clarinet() {}
    }
  }
  mod voice { }
}
```

**调用模块有两种方式，绝对路径和相对路径**
```rs
// 绝对路径
crate::sound::instrument::clarinet(); //以crate开头

// Relative path
// 可以以self、super或当前模块的标识符开头
sound::instrument::clarinet();

//外部包
use std::collections::HashMap;
```
> 外部包都是**绝对路径**

**模块有访问性边界**
- 所有项(函数、方法、结构体、枚举、模块和常量)默认是私有的
- 可以使用 pub 关键字使项变为公有
- 不允许使用模块的子模块中的私有代码
- 允许使用任何父模块或当前模块中的代码
```rs
mod sound {
  pub mod instrument {
    pub fn clarinet() {} 
  }
}
```
> `pub`关键字作用于：结构体、枚举、函数和方法以及模块<br>
> 对于sound不用加mod，是因为`main`和`sound`在同一个模块中

**对于结构体，字段也需要`pub`关键字才可访问**<br>
**而枚举，只需要`pub enum`即可**<br>

**通过`use`关键字可以缩短调用长度**
```rs
//非use
sound::instrument::clarinet();

//use
use crate_name::sound::instrument;
instrument::clarinet();

//use 相对路径
// 也可以加super
use self::sound::instrument;  // self 是必须的, for now

use crate_name::sound::instrument as Ins;  // as 用法同其他语言
```
> use后，依旧可以按照旧方式调用

**使用`pub use`使得 use 后，可以重导出**
```rs
mod group{
  pub use crate::sound::instrument;
}

group::instrument::clarinet();
```

**可以通过嵌套和glob，将多个带有相同前缀的项引入作用域**
```rs
use std::{cmp::Ordering, io}; //引入 std::cmp::Ordering; std::io;

use std::io::{self, Write}; //引入 std::io; std::io::Write;

//glob
use std::cmp::*;
```

**多个文件内的Mod**
```rs
// src/sound/instrument.rs
pub fn clarinet() {}

// src/sound.rs
pub mod instrument;

// src/main
mod sound;
fn main(){
  sound::instrument::clarinet();
}
```
