// 字符串集合
// Rust 核心语言中只有一种字符串类型： `str`
// 字符串 slice， 它通常以被借用的形式出现，`&str`
// 称作 `String` 的类型是由标准库提供的，而没有写进核心语言部分，它是可增长的、可变的、有所有权的、UTF-8编码的字符串类型的
// 字符串 slice: 它是一些存在别处的 UTF-8 编码字符串数据的引用，比如字符串字面值被存储在程序的二进制输出中，字符串slice 也是如此
// 字符串：通常指 `String` 和 字符串 slice `&str` 类型，而不仅仅是其中之一，这个两个类型在 Rust 标准库中都被广泛使用，`String` 和字符串 slice 都是 UTF-8 编码的
// Rust 字符串不支持索引
// 标准库中还提供一系列其他字符串类型，比如 `OsString`、`OsStr`、`CString`、`CStr` 相关库
pub fn string_collection() {
  // ------------------------ 新建字符串 ------------------------
  // 很多 `Vec` 可用的操作在 `String` 中同样可用，从以 `new` 函数创建字符串开始
  // let mut s = String::new();

  // 初始字符串数据
  // 例子 1
  // let data = "initial contents";
  // let s = data.to_string();
  // 例子 2
  // let s = "initial contents".to_string();
  // 例子 3
  // let s = String::from("initial contents");
  
  // `String::from` 和 `.to_string` 最终做了完全相同的工作，所以如何选择就是风格问题了

  // ------------------------ 更新字符串 ------------------------
  // 可以使用 `+` 运算符或 `format!` 宏来拼接 `String` 值

  // 使用 push_str 和 push 附加字符串
  // 可以通过 `push_str` 方法来附加字符串 slice， 从而使 `String` 变长

  // let mut s = String::from("foo");
  // s.push_str("bar");
  
  // 执行这两行代码之后，`s` 将会包含 `foobar`。`push_str` 方法采用字符串 slice，因为我们并不需要获取参数的所有权

  // let mut s1 = String::from("foo");
  // let s2 = "bar";
  // s1.push_str(s2);
  // println!("s2 is {}", s2);

  // 如果 `push_str` 方法获取了 `s2` 的所有权，就不能在最后一行打印其值了
  // push 方法被定义为获取一个单独的字符作为参数，并附加到 `String` 中，展示了使用 `push` 方法将字母/加入 `String` 的代码

  // let mut s = String::from("lo");
  // s.push('l');

  // ------------------------ 使用 + 运算符或 format! 宏拼接字符串 ------------------------
  // 通常你会希望两个已知的字符串合并在一起

  // let s1 = String::from("Hello,");
  // let s2 = String::from("world!");
  // let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用

  // s1 相加后不在有效的原因，使用s2的引用的原因，与使用 `+` 运算符时调用的函数签名有关。 `+` 运算符使用 `add` 函数，函数签名看起来像这样：
  // fn add(self, s: &str) -> String { };
  // 这并不是标准库中实际的签名；标准库中的 `add` 使用范型定义。这里我们看到的 `add` 签名使用具体类型代替了范型
  // 之所以能够在 `add` 中调用 `&s2` 是因为 `&String` 可以被强转（coerced） 成 `&str`。当 `add` 函数被调用时，Rust使用了一个被称为 Deref 强制转换。因为 `add` 没有获取参数的所有权，所以 `s2` 在这个操作后依然是有效的 `String`

  // 其次，可以发现签名中 `add` 获取了 `self` 的所有权，因为 `self` 没有使用 `&`。 意味着 `s1` 的所有权将被移动到 `add` 调用中，之后就不再有效，所以虽然 `let s3 = s1 + &s2`; 看起来就像它会复制两个字符串并创建一个新的字符串，而实际上语句会获取 `s1` 的所有权，附加上从 `s2` 中拷贝的内容，并返回结果的所有权
  // ”看起来生成的了很多的了拷贝，不过实际上并没有：这个实现比拷贝更有效“

  // 要想级联多个字符串，`+` 的行为就显得笨重了
  // let s1 = String::from("tic");
  // let s2 = String::from("tac");
  // let s3 = String::from("toe");

  // old
  // let s = s1 + "-" + &s2 + "-" + &s3;

  // new 
  // let s = format!("{}-{}-{}", s1, s2, s3);
  // println!("{}", s);

  // `String` 是一个 `Vec<u8>` 的封装
  // let len = String::from("Hello").len();
  
  // 谨慎使用 字符串切片

  // ------------------------ 遍历字符串的方法 ------------------------
  // 如果你需要单独操作 Unicode 标量值，最好的选择是使用 `chars` 
  for c in "नमस्ते".chars() {
    println!("{}", c);
  }
  // `bytes` 方法返回每一个原始字节
  // PS: Unicode 标量值可能不是有一个字节组成
  for b in "नमस्ते".bytes() {
    println!("{}", b);
  }
}
