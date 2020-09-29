/// 迭代器是 惰性的（lazy），这意味着在使用迭代器之前它都不会有效果
///
/// 迭代器都实现了一个叫做 Iterator 的 trait
pub trait IIterator {
  /// `type Item` 和 `Self::Item`，他们定义了 trait 的 关联类型，
  /// 表明实现 Iterator trait 要求同时定义一个 Item 类型，而且这个 Item 类型被用作 next 方法的返回值类型
  type Item;

  fn next(&mut self) -> Option<Self::Item>;

  // 此处省略了方法的默认实现
}
//典型的 iter (如Vec.iter) 在迭代器上调用 next 方法改变了迭代器中用来记录序列位置的状态，因此 iter 需要是可变的
//而for 循环会获取 iter 的所有权并在后台使 iter 可变


/// Iterator trait 有一系列不同的默认实现的方法。

/// 一些方法在其定义中调用了 next 方法，被称为消费适配器(consuming adaptors)
let v2 = vec![1, 2, 3].iter().sum()

/// 还有另一类方法，被称为迭代器适配器（iterator adaptors），他们允许我们将当前迭代器变为不同类型的迭代器。可以用来生成链式调用的迭代器
let v3: Vec<_> = vec![1, 2, 3].iter().map(|x| x + 1).collect();
//注意，因为所有的迭代器都是惰性的，必须调用一个消费适配器方法以便获取迭代器适配器调用的结果。但可以使用 .collect 暂存起来

#[derive(PartialEq, Debug)]
struct Shoe {
  size: u32,
  style: String,
}

fn shoes_fit_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
  shoes.into_iter() //获得 iter 的所有权，并返回拥有所有权的 iter
    .filter(|s| s.size == shoe_size)
    .collect()
}

#[test]
fn filters_by_size() {
  let shoes = vec![
    Shoe { size: 10, style: String::from("sneaker") },
    Shoe { size: 13, style: String::from("sandal") },
    Shoe { size: 10, style: String::from("boot") },
  ];
  let fit_size = shoes_fit_size(shoes, 10);
  assert_eq!(
    fit_size,
    vec![
      Shoe { size: 10, style: String::from("sneaker") },
      Shoe { size: 10, style: String::from("boot") },
    ]
  );
}

/// 自定义迭代器
/// 
/// 只需要实现 next 方法即可
impl Iterator for Shoe {
  type Item = u32;
  fn next(&mut self) -> Option<Self::Item> {
      self.size += 1;
      if self.size < 6 {
          Some(self.size)
      } else {
          None
      }
  }
}

// 迭代器是 Rust 的 零成本抽象(zero-cost abstractions)之一, 它意味着抽象并不会引入运行时开销, 完全可以放心大胆的使用
