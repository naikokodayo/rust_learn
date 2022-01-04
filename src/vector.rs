// vector
// vector 是使用 `范型` 实现的，`Vec` 是由一个标准库提供的，可以存放任何类型
// vector 允许我们在一个单独的数据结构中存储多余一个的值
// 它在内存中彼此相邻地排列所有的值，vector 只能存储相同类型的值
pub fn vector_fn() {
  // ------------------------ 新建 vector ------------------------
  // 创建一个新的空 vector 可以调用 `Vec::new()` 函数

  // let v: Vec<i32> = Vec::new();
  // 这里添加的一个类型注解，因为没有向这个 vector 中插入任何值，Rust 并不知道我们想要存储什么类型的元素
  // 存放类型时，位于尖括号中，`v` 这个 `Vec` 将存放 `i32` 类型的元素

  // 更实际的代码中，一旦插入值， Rust 就可以推断出想要存放的类型（很少需要类型注解）
  // 常见的方法是使用初始值创建一个 `Vec`, Rust 提供了 `vec!` 宏，这个宏会根据，我们提供的值创建一个新的 `Vec`

  // let v = vec![1, 2, 3];
  // 因为哦们提供了 `i32` 的初始值，Rust 可以推断出 `v` 的类型是 `Vec<i32>` 因此类型注解就不是必须的

  // ------------------------ 更新 vector ------------------------
  // 对于新建的 vector 并向其添加元素，可以使用 `push` 方法

  // let mut v = Vec::new();
  // v.push(5);
  // v.push(6);
  // v.push(7);
  // v.push(8);

  // ------------------------ 丢弃 vector 时也会丢弃其所有元素 ------------------------
  // 类似于任何其他的 `struct`， vector 在其离开作用域时 会被释放

  // 当 vector 被丢弃时，所有其内容也会被丢弃，这意味着它包含的整数将被清理。这可能看起来非常直观，不过一旦开始使用 `vector` 元素的引用，情况就变得有些复杂了

  // ------------------------ 读取 vector 的元素 ------------------------
  // let v = vec![1, 2, 3, 4, 5];

  // let third: &i32 = &v[2];
  // println!("The third element is {}", third);

  // match v.get(2) {
  //   Some(third) => println!("The third element is {}", third),
  //   None => println!("There is no third element")
  // }
  
  // !!! 当引用一个不存在的元素 Rust 会造成 panic，Rust 会认为访问超过 vector 结尾是一个严重的错误的情况
  // let does_not_exist = &v[100];

  // ------------------------ 遍历 vector 的元素 ------------------------
  // 不做赘述
  // let v = vec![100, 32, 57];
  // for i in &v {
  //   println!("{}", i);
  // }

  // 也可以遍历元素并改变他们
  let mut v = vec![100, 32, 57];
  for i in &mut v {
    *i += 50;
    println!("{}", i);
  }

  // ------------------------ 使用枚举来存储多种类型 ------------------------
  // vector 只能存储相同类型的值，这是很不方便的；绝对会有需要存储一系列不同类型的值的用例
  // 可以定义并使用一个枚举
  enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }

  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("name")),
    SpreadsheetCell::Float(10.12)
  ];

  // 1. Rust 在编译时就必须准确的是到 vector 中类型的原因在于它需要知道存储的每个元素 需要多少内存
  // 2. 可以准确知道这个 vector 需要什么类型如果 Rust 允许 vector 存放任意类型，那么当对 vector 元素执行操作时，一个或多个类型的值就有可能会造成错误，使用枚举外加 `match` 意味着 Rust 能在编译时就保证总是会处理所有可能的情况

  // 如果在编写程序时 不能确切无疑地知道运行时，会存储进 vector 的所有类型，枚举技术就行不通了，相反你可以使用 trait 对象
}