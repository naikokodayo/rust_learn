// 变量与可变性

/**
 * 常量(constant) 绑定值以后是不可变的
 * 1. 不可以使用 mut 关键词， 常量永远都不可变
 * 2. 声明需要使用 const 关键词，且 必须标注类型
 * 3. 可以在任何作用域中声明，包括全局作用域
 * 4. 常量在编译时就确定了，无法绑定到函数的调用结果，或者只能在运算时计算褚的值
*/
const MAX_POINT:u32 = 10_000;

fn main() {
    // 常量: const 关键字 不允许被修改
    println!("这是一个常量: {}", MAX_POINT);

    // 变量默认是不可变的
    // mut 关键词 可以把变量改变成可变的变量
    let mut x = 5;
    println!("The value of {}", x);
    x = 6;
    println!("The update value o {}", x);
    
    // 以下代码会报错 不应该修改变量的类型
    // let mut spaces = "   ";
    // spaces = spaces.len();

    // let 变量可以重复声明
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces 长度是: {}", spaces);

    // shadowing 
    let y = 5;
    // 这里第一个 y 是重新计算的变量， 第二个y 则是使用上面已声明的变量
    let y = y + 1;
    println!("shadowing y of value: {}", y);
}