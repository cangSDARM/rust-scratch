# 自动化测试
[返回](../README.md)

Rust 中的测试就是带有`test`**属性**注解的函数

属性(attribute)是关于Rust代码片段的元数据
```rs
// cargo new --lib 生成的测试模块样例
fn main() {}
#[cfg(test)]
mod tests {
  use super::*; //通常test模块会引用所有的模块项

  #[test]
  fn it_works() {
    assert!(true, "info");
    //eq和ne要求比较对象实现`PartialEq`和`Debug`两个trait
    assert_eq!(2 + 2, 4);
    assert_ne!(1, 0);
  }
}
```

**除了检查代码是否返回期望的值之外，检查代码是否正确处理错误也是很重要的**
```rs
#[test]
#[should_panic(expected = "Info")]
fn guess(){
  Guess::new(200);  //should painc in Guess::new
}
```
> 注意，`should_painc`中的expected必须是**发生panic的信息的子串**。这样做是为了保证发生的painc是预期的

**也可以使用Result枚举来控制测试是否正常**
```rs
#[test]
fn rusults() -> Result<(), String>{
  if 2+2 == 4{
    Ok(())
  }else{
    Err(Stirng::from("Error happed"))
  }
}
```
> 注意，`should_painc`和`Result`不能混用

## 测试控制流程及分类
使用`cargo test`运行测试(带test宏的)，其包括

1. 普通测试，由`running X test`显示
2. 被忽略的，由`X ignored`显示
3. 被过滤的，由`X filtered out`显示
4. 性能测试，由`X measured`显示
5. 文档测试，由`Doc-tests`显示

### 控制测试流程
