/// 智能指针（smart pointers）是一类数据结构，他们的表现类似指针，但是也拥有额外的元数据和功能
/// 在大部分情况下，智能指针**拥有**他们指向的数据

//智能指针通常使用结构体实现。其实现了 Deref 和 Drop trait
//- Deref 允许智能指针结构体实例表现的像引用一样
//- Drop 允许自定义当智能指针离开作用域时运行的代码

// 内部可变性（interior mutability）
// 引用循环（reference cycles）

let s = String::from("Smart");
let v = vec!['p','o','i','n','t','e','r'];
let r = Rc<String>::new();  //一个引用计数类型，其数据可以有多个所有者
let re = Ref<String>::new();  //通过 RefCell<T> 访问，一个在运行时而不是在编译时执行借用规则的类型
let rm = RefMut<String>::new(); //同上

/// Box 用于在堆上分配值。除了数据被储存在堆上而不是栈上之外，box 没有性能损失
///
///常用于： 
/// 1. 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候(如 递归类型(recursive type))
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
