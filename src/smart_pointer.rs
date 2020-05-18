//智能指针通常使用结构体实现。其实现了 Deref 和 Drop trait
use std::ops::Deref;  //Deref 允许智能指针结构体实例表现的像引用一样(重载 解引用运算符)
use std::ops::DerefMut; //可变引用使用 DerefMut
use std::ops::Drop; //Drop 允许自定义当智能指针离开作用域时运行的代码
use std::rc::Rc;  //一个不可变强引用的计数类型，其数据可以有多个所有者

/// 智能指针（smart pointers）是一类数据结构，他们的表现类似指针，但是也拥有额外的元数据和功能
/// 在大部分情况下，智能指针**拥有**他们指向的数据

// 内部可变性（interior mutability）
// 引用循环（reference cycles）

let s = String::from("Smart");
let v = vec!['p','o','i','n','t','e','r'];
let re = Ref<String>::new();  //通过 RefCell<T> 访问，一个在运行时而不是在编译时执行借用规则的类型
let rm = RefMut<String>::new(); //同上

/// Box 用于在堆上分配值。除了数据被储存在堆上而不是栈上之外，box 没有性能损失
///
/// 从根本上说，Box<T> 被定义为包含一个元素的元组结构体
///常用于： 
/// 1. 当有一个编译时大小未知的类型，而又想在需要确切大小的上下文中使用这个类型值的时候(如 递归类型(recursive type))
/// 2. 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
/// 3. 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候(称为 trait 对象(trait object))
let b = Box<String>::new(5);  //使用 box 在堆上储存一个 i32 值

//Box 生成链表(定长链表?)的例子。链表就是一个典型的递归类型
enum List {
  Cons(i32, Box<List>), //使用Box包裹List，因为Box定长
  Nil,
}
let cons_list = Cons(1,
  Box::new(Cons(2,
      Box::new(Cons(3,
          Box::new(Nil))))));

// 自定义智能指针
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
// 则可以使用：
assert_eq!(*MyBox::new(String::from(1)), 1);
// 即：*y ==> *(y.deref()) ==> *(&y.0)

/// 解引用强制多态(deref coercions)
/// Rust 可以自动调用 Deref 链直到一个未实现 Deref 的类型
&MyBox::new(String::from("hello")) == "hello".as_str();  //成为可能(否则就需要 &(*m)[..] 来达成)
/// 满足：
/// - 当 T: Deref<Target=U> 时从 &MT 到 &U
/// - 当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U
/// - 当 T: Deref<Target=U> 时从 &mut T 到 &U
///
/// 该行为发生在编译时，并没有运行时惩罚

/// Drop 不能手动调用。如需手动调用，使用 std::mem::drop (已经作为 prolude)
impl Drop for MyBox {
  fn drop(&mut self) {
    println!("Dropping data :{}", self.0)
  }
}

enum List {
  Cons(i32, Rc<List>),
  Nil,
}
let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
let b = Cons(3, Rc::clone(&a)); //克隆 a 所包含的 Rc，将引用计数从 1 增加到 2 并允许 a 和 b 共享 Rc 中数据的所有权
let c = Cons(4, Rc::clone(&a));
