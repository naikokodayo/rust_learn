// 数据类型
// 标量和复合类型
/**
 * rust 是静态编译语言 需要在编辑时知道变量的具体类型
 * 通常 rust 编译器可以推断出具体类型
 * 如果可能的类型较多 （例如 string 转为 整数的 parse()方法），就必须添加类型的标注，否则编辑会报错
*/
fn main() {
  let guess: u32 = "42".parse().expect("Not a number");
  println!("{}", guess);
  // 一个标量类型表示一个单个的值
  // Rust 主要有 4中标量类型
  // 整数类型
  // u32 无符号的整数类型，占据32位空间 2的32次方减1
  // 无符号整数以 u 开头 非负数  0到 2 (n-1)次方
  // 有符号的整数以 i 开头 正负数都可以 - 2的 (n-1)次方 到 2 (n-1)次方 - 1 
  // isize 和 usize类型由运行程序的计算机架构决定 例如：64位即64位 32即32位
  let num = 5 + 2;
  let product = 3 * 32;
  let reminder = 54 % 5;
  println!("{} {} {} ", num, product, reminder);

  // 浮点类型
  let fu = 3.2;
  let x1: f32 = 3.0;
  let quotient = 56.7 / 32.2;
  let difference = 95.5 - 4.3;
  println!("{} {} {} {}", quotient, difference, fu, x1);

  // 布尔类型
  let t = true;
  let f: bool = false;
  println!("{} {}", t, f);

  // 字符类型
  // Rust char类型 描述语言中最基础的单个类型
  // 字符类型的字面值使用单引号
  // 占用 4个字节大小
  // 是 Unicode 标量值，可以表示比 ASCll 多的多字符内容，例如：拼音、中日韩文、emoji表情、零长度空白字符等
  let e = 'e';
  let r: char = 'r';
  let emoji = '🧒';
  println!("{} {} {}", e, r, emoji);
}